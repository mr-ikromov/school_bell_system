use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::time::{Duration, Instant};

use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::source::Source;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use serde::Serialize;

use crate::routing::SpeakerRoute;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Bell,
    Anthem,
    Alarm,
}

impl Kind {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "bell" => Some(Self::Bell),
            "anthem" => Some(Self::Anthem),
            "alarm" => Some(Self::Alarm),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bell => "bell",
            Self::Anthem => "anthem",
            Self::Alarm => "alarm",
        }
    }

    fn idx(self) -> usize {
        self as usize
    }

    fn loops(self) -> bool {
        self == Self::Alarm
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SoundState {
    pub kind: String,
    pub playing: bool,
}

const IDLE_CLOSE: Duration = Duration::from_secs(1);

const OPEN_RETRIES: u32 = 6;
const OPEN_RETRY_DELAY: Duration = Duration::from_millis(250);

enum Cmd {
    Play {
        kind: Kind,
        path: Option<PathBuf>,
        volume: f32,
    },
    Stop(Kind),
    StopAll,

    Volume(f32),
}

#[derive(Default)]
struct Flags {
    playing: [AtomicBool; 3],
}

#[derive(Clone)]
pub struct Audio {
    tx: Sender<Cmd>,
    flags: Arc<Flags>,
}

impl Audio {
    pub fn spawn<F, E>(on_change: F, on_error: E) -> Self
    where
        F: Fn(SoundState) + Send + 'static,
        E: Fn(&str, String) + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let flags = Arc::new(Flags::default());
        let worker_flags = Arc::clone(&flags);

        std::thread::Builder::new()
            .name("bell-audio".into())
            .spawn(move || audio_thread(rx, worker_flags, on_change, on_error))
            .expect("ovoz ipini ochib bo'lmadi");

        Self { tx, flags }
    }

    pub fn is_playing(&self, kind: Kind) -> bool {
        self.flags.playing[kind.idx()].load(Ordering::Relaxed)
    }

    pub fn any_playing(&self) -> bool {
        self.flags.playing.iter().any(|f| f.load(Ordering::Relaxed))
    }

    pub fn play(&self, kind: Kind, path: Option<PathBuf>, volume: f32) {
        let _ = self.tx.send(Cmd::Play { kind, path, volume });
    }

    pub fn stop(&self, kind: Kind) {
        let _ = self.tx.send(Cmd::Stop(kind));
    }

    pub fn stop_all(&self) {
        let _ = self.tx.send(Cmd::StopAll);
    }

    pub fn set_volume(&self, v: f32) {
        let _ = self.tx.send(Cmd::Volume(v));
    }
}

pub fn output_devices() -> Vec<String> {
    let host = rodio::cpal::default_host();
    host.output_devices()
        .map(|it| it.filter_map(|d| d.name().ok()).collect())
        .unwrap_or_default()
}

fn audio_thread<F, E>(rx: Receiver<Cmd>, flags: Arc<Flags>, on_change: F, on_error: E)
where
    F: Fn(SoundState),
    E: Fn(&str, String),
{
    let mut engine = Engine::new();
    let mut sinks: [Vec<Sink>; 3] = [Vec::new(), Vec::new(), Vec::new()];

    loop {
        match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(Cmd::Play { kind, path, volume }) => {
                for k in [Kind::Bell, Kind::Anthem, Kind::Alarm] {
                    stop_one(&mut sinks, k, &flags, &on_change);
                }
                match engine.start(kind, path.as_deref(), volume) {
                    Ok(new) => {
                        sinks[kind.idx()] = new;
                        set_flag(&flags, kind, true, &on_change);
                    }
                    Err(e) => {
                        log::error!("{} CHALINMADI: {e}", kind.as_str());
                        set_flag(&flags, kind, false, &on_change);
                        on_error(kind.as_str(), e);
                    }
                }
            }
            Ok(Cmd::Stop(kind)) => stop_one(&mut sinks, kind, &flags, &on_change),
            Ok(Cmd::Volume(v)) => {
                let v = v.clamp(0.0, 1.0);
                for k in [Kind::Bell, Kind::Anthem, Kind::Alarm] {
                    for s in &sinks[k.idx()] {
                        s.set_volume(v);
                    }
                }
            }
            Ok(Cmd::StopAll) => {
                for k in [Kind::Bell, Kind::Anthem, Kind::Alarm] {
                    stop_one(&mut sinks, k, &flags, &on_change);
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                for k in [Kind::Bell, Kind::Anthem, Kind::Alarm] {
                    let v = &sinks[k.idx()];
                    if !v.is_empty() && v.iter().all(|s| s.empty()) {
                        sinks[k.idx()].clear();
                        set_flag(&flags, k, false, &on_change);
                    }
                }
                let busy = sinks.iter().any(|v| !v.is_empty());
                engine.close_if_idle(busy);
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    log::info!("ovoz ipi yopildi");
}

fn stop_one<F: Fn(SoundState)>(
    sinks: &mut [Vec<Sink>; 3],
    kind: Kind,
    flags: &Flags,
    on_change: &F,
) {
    for s in sinks[kind.idx()].drain(..) {
        s.stop();
    }
    set_flag(flags, kind, false, on_change);
}

fn set_flag<F: Fn(SoundState)>(flags: &Flags, kind: Kind, on: bool, on_change: &F) {
    let prev = flags.playing[kind.idx()].swap(on, Ordering::Relaxed);
    if prev != on {
        on_change(SoundState {
            kind: kind.as_str().into(),
            playing: on,
        });
    }
}

struct Engine {
    streams: Vec<OutputStream>,

    route: SpeakerRoute,
    idle_since: Option<Instant>,
}

impl Engine {
    fn new() -> Self {
        Self {
            streams: Vec::new(),
            route: SpeakerRoute::None,
            idle_since: None,
        }
    }

    fn open(&mut self) {
        self.idle_since = None;

        let route = crate::routing::speaker_route();
        if !self.streams.is_empty() && self.route == route {
            return;
        }

        self.streams.clear();
        self.route = route.clone();

        let stream = match &route {
            SpeakerRoute::ByDevice(name) => open_stream(Some(name)).or_else(|| {
                log::warn!("'{name}' qurilmasi ochilmadi — signal kompyuter dinamigidan chiqadi");
                open_stream(None)
            }),
            _ => open_stream(None),
        };

        let Some(s) = stream else {
            log::error!("ovoz chiqishi {OPEN_RETRIES} urinishdan keyin ham ochilmadi");
            return;
        };
        self.streams.push(s);

        if let SpeakerRoute::ByDevice(name) = &route {
            log::info!("ovoz chiqishi: {name}");
        }
    }

    fn close_if_idle(&mut self, busy: bool) {
        if busy || self.streams.is_empty() {
            self.idle_since = None;
            return;
        }
        match self.idle_since {
            None => self.idle_since = Some(Instant::now()),
            Some(t) if t.elapsed() >= IDLE_CLOSE => {
                self.streams.clear();
                self.route = SpeakerRoute::None;
                self.idle_since = None;
            }
            Some(_) => {}
        }
    }

    fn start(&mut self, kind: Kind, path: Option<&Path>, volume: f32) -> Result<Vec<Sink>, String> {
        self.open();
        if self.streams.is_empty() {
            return Err("chiqish qurilmasi yo'q".into());
        }

        let mut warned = false;
        let mut sinks = Vec::with_capacity(self.streams.len());

        for stream in &self.streams {
            let sink = Sink::connect_new(stream.mixer());
            sink.set_volume(volume.clamp(0.0, 1.0));

            match path.and_then(decode) {
                Some(src) => append(&sink, src, kind),
                None => {
                    if let (Some(p), false) = (path, warned) {
                        log::warn!(
                            "{} fayli o'qilmadi ({}) — ichki ovoz",
                            kind.as_str(),
                            p.display()
                        );
                        warned = true;
                    }
                    append(&sink, synth::build(kind), kind);
                }
            }
            sinks.push(sink);
        }

        if self.route == SpeakerRoute::MoveAfterOpen {
            route_to_speaker();
        }

        Ok(sinks)
    }
}

fn route_to_speaker() {
    let marker = crate::routing::own_marker();
    for attempt in 0..40 {
        if crate::routing::send_our_streams_to_speaker(&marker) > 0 {
            crate::routing::set_speaker_volume_max();
            if attempt > 0 {
                log::debug!("oqim {}-urinishda kolonkaga o'tdi", attempt + 1);
            }
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    log::warn!("oqim kolonkaga yo'naltirilmadi — signal kompyuter dinamigidan chiqadi");
}

fn open_stream(named: Option<&str>) -> Option<OutputStream> {
    for attempt in 1..=OPEN_RETRIES {
        let opened = match named {
            Some(n) => open_named(n),
            None => OutputStreamBuilder::open_default_stream().ok(),
        };
        if let Some(mut s) = opened {
            if attempt > 1 {
                log::info!("ovoz chiqishi {attempt}-urinishda ochildi");
            }
            s.log_on_drop(false);
            return Some(s);
        }
        if attempt < OPEN_RETRIES {
            std::thread::sleep(OPEN_RETRY_DELAY);
        }
    }
    None
}

fn open_named(name: &str) -> Option<OutputStream> {
    let device = rodio::cpal::default_host()
        .output_devices()
        .ok()?
        .find(|d| d.name().map(|n| n == name).unwrap_or(false))?;
    OutputStreamBuilder::from_device(device)
        .ok()?
        .open_stream()
        .ok()
}

pub fn probe(path: &Path) -> Result<(), String> {
    const MIN_SECS: f64 = 0.2;

    const PROBE_SECS: f64 = 2.0;

    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let src = Decoder::new(std::io::BufReader::new(file)).map_err(|_| "undecodable")?;

    let per_sec = f64::from(src.sample_rate().max(1)) * f64::from(src.channels().max(1));
    let count = src.take((per_sec * PROBE_SECS) as usize).count();

    if count as f64 / per_sec < MIN_SECS {
        return Err("too-short".into());
    }
    Ok(())
}

fn decode(path: &Path) -> Option<Decoder<std::io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(path).ok()?;
    Decoder::new(std::io::BufReader::new(file)).ok()
}

fn append<S>(sink: &Sink, source: S, kind: Kind)
where
    S: Source + Send + 'static,
{
    if kind.loops() {
        sink.append(source.repeat_infinite());
    } else {
        sink.append(source);
    }
}

mod synth {
    use super::Kind;
    use rodio::buffer::SamplesBuffer;

    const RATE: u32 = 44_100;

    pub fn build(kind: Kind) -> SamplesBuffer {
        let data = match kind {
            Kind::Bell => bell(),
            Kind::Anthem => anthem(),
            Kind::Alarm => siren(),
        };
        SamplesBuffer::new(1, RATE, data)
    }

    fn bell() -> Vec<f32> {
        const PARTIALS: [(f32, f32); 5] = [
            (1.0, 1.0),
            (2.01, 0.42),
            (2.98, 0.26),
            (4.17, 0.14),
            (5.43, 0.08),
        ];
        let strike_len = (RATE as f32 * 0.85) as usize;
        let tail = (RATE as f32 * 1.6) as usize;
        let mut out = vec![0.0f32; strike_len * 3 + tail];

        for hit in 0..3 {
            let start = hit * strike_len;
            for i in 0..(strike_len + tail) {
                let t = i as f32 / RATE as f32;
                let env = (-3.2 * t).exp();
                let mut s = 0.0;
                for (mul, amp) in PARTIALS {
                    s += amp * (std::f32::consts::TAU * 660.0 * mul * t).sin();
                }
                if let Some(v) = out.get_mut(start + i) {
                    *v += s * env * 0.16;
                }
            }
        }
        out
    }

    fn anthem() -> Vec<f32> {
        const SEQ: [(f32, f32); 9] = [
            (392.0, 0.50),
            (392.0, 0.28),
            (523.0, 0.60),
            (494.0, 0.32),
            (440.0, 0.32),
            (392.0, 0.55),
            (330.0, 0.38),
            (349.0, 0.32),
            (392.0, 0.95),
        ];
        let mut out = Vec::new();
        for (freq, dur) in SEQ {
            let n = (RATE as f32 * dur) as usize;
            for i in 0..n {
                let t = i as f32 / RATE as f32;
                let env = (1.0 - t / dur).clamp(0.0, 1.0).powf(0.4);
                out.push((std::f32::consts::TAU * freq * t).sin() * env * 0.22);
            }
        }
        out
    }

    fn siren() -> Vec<f32> {
        let n = (RATE as f32 * 1.4) as usize;
        let mut out = Vec::with_capacity(n);
        let mut phase = 0.0f32;
        for i in 0..n {
            let t = i as f32 / RATE as f32;
            let freq = 620.0 + 300.0 * (std::f32::consts::TAU * 1.4 * t).sin();
            phase += std::f32::consts::TAU * freq / RATE as f32;
            out.push(phase.sin() * 0.28);
        }
        out
    }
}
