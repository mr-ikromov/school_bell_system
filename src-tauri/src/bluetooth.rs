use std::io::{BufRead, BufReader};
use std::thread;
use std::time::{Duration, Instant};

use tauri::{AppHandle, Emitter, Manager};

use crate::audio;
use crate::models::BtStatus;
use crate::routing;
use crate::state::AppState;

const HINTS: [&str; 5] = ["bluez", "bluetooth", "a2dp", "hands-free", "headset"];

fn looks_bluetooth(name: &str) -> bool {
    let low = name.to_ascii_lowercase();
    HINTS.iter().any(|h| low.contains(h))
}

pub fn detect(devices: &[String]) -> Option<String> {
    devices.iter().find(|d| looks_bluetooth(d)).cloned()
}

pub fn spawn(app: AppHandle) {
    thread::Builder::new()
        .name("bell-bluetooth".into())
        .spawn(move || watch(app))
        .expect("bluetooth ipini ochib bo'lmadi");
}

const COALESCE: Duration = Duration::from_millis(400);

const POLL: Duration = Duration::from_secs(2);

fn watch(app: AppHandle) {
    let marker = routing::own_marker();
    let mut last_status: Option<BtStatus> = None;

    if !routing::available() {
        log::warn!(
            "ovoz marshrutini boshqarib bo'lmaydi (bu platformada hali bajarilmagan) \
             — kompyuterning boshqa ovozlari kolonkaga borishi mumkin"
        );
        return;
    }

    apply(&app, &marker, &mut last_status);

    loop {
        let Some(mut child) = routing::subscribe() else {
            log::info!("hodisa oqimi yo'q — har {POLL:?} da tekshiriladi");
            loop {
                thread::sleep(POLL);
                apply(&app, &marker, &mut last_status);
            }
        };
        let Some(out) = child.stdout.take() else {
            let _ = child.kill();
            return;
        };

        let mut last_run = Instant::now();
        for line in BufReader::new(out).lines().map_while(Result::ok) {
            if !routing::event_matters(&line) {
                continue;
            }
            let since = last_run.elapsed();
            if since < COALESCE {
                thread::sleep(COALESCE - since);
            }
            last_run = Instant::now();
            apply(&app, &marker, &mut last_status);
        }

        let _ = child.kill();
        let _ = child.wait();
        log::warn!("ovoz serveri hodisalari uzildi — qayta ulanamiz");
        thread::sleep(Duration::from_secs(3));
    }
}

fn apply(app: &AppHandle, marker: &str, last: &mut Option<BtStatus>) {
    if app.state::<AppState>().enabled() {
        routing::keep_others_off_speaker(marker);
    }
    report(app, last);
}

pub fn report(app: &AppHandle, last: &mut Option<BtStatus>) {
    let st = current_status(&app.state::<AppState>());
    if last.as_ref().is_none_or(|p| *p != st) {
        match (&st.connected, &st.device) {
            (true, Some(d)) => log::info!("kolonka ulandi: {d}"),
            _ => log::info!("kolonka ulanmagan"),
        }
        let _ = app.emit("bt-status", st.clone());
        *last = Some(st);
    }
}

pub fn current_status(state: &AppState) -> BtStatus {
    let playing = state.audio.any_playing();

    if routing::available() {
        let count = routing::bluetooth_sinks().len();
        return match routing::bluetooth_sink() {
            Some(s) => BtStatus {
                connected: true,
                muted: !playing,
                device: Some(s.name),
                count,
            },
            None => BtStatus::default(),
        };
    }

    match detect(&audio::output_devices()) {
        Some(name) => BtStatus {
            connected: true,
            muted: !playing,
            device: Some(name),
            count: 1,
        },
        None => BtStatus::default(),
    }
}
