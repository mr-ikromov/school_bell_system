use std::collections::HashSet;
use std::thread;
use std::time::{Duration, Instant};

use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone};
use tauri::{AppHandle, Emitter, Manager};

use crate::models::Bell;
use crate::state::AppState;

const TICK: Duration = Duration::from_millis(500);

const JUMP_MS: i64 = 5_000;

const CATCH_UP_SECS: i64 = 90;

const SLOW_TICK_MS: i64 = 3_000;

#[derive(Clone, serde::Serialize)]
pub struct RingEvent {
    pub id: String,
    pub label: String,
    pub time: String,
}

#[derive(Clone, serde::Serialize)]
pub struct JumpEvent {
    pub from: String,
    pub to: String,
    pub skipped: usize,
}

pub fn spawn(app: AppHandle) {
    thread::Builder::new()
        .name("bell-scheduler".into())
        .spawn(move || run(app))
        .expect("rejalashtiruvchi ipini ochib bo'lmadi");
}

fn run(app: AppHandle) {
    let mut fired: HashSet<(NaiveDate, String)> = HashSet::new();
    let mut today = Local::now().date_naive();

    let mut last_wall = Local::now();
    let mut last_mono = Instant::now();

    belgila_otganlarni(&app, &mut fired, last_wall);

    log::info!("rejalashtiruvchi ishga tushdi");

    loop {
        thread::sleep(TICK);

        let mono_now = Instant::now();
        let wall_now = Local::now();

        let mono_ms = mono_now.duration_since(last_mono).as_millis() as i64;
        let wall_ms = wall_now.signed_duration_since(last_wall).num_milliseconds();
        let drift = wall_ms - mono_ms;

        if drift.abs() > JUMP_MS {
            let skipped = mark_skipped(&app, &mut fired, last_wall, wall_now);
            log::warn!(
                "soat sakradi: {} ms ({} -> {}), {} signal o'tkazib yuborildi",
                drift,
                last_wall.format("%Y-%m-%d %H:%M:%S"),
                wall_now.format("%Y-%m-%d %H:%M:%S"),
                skipped
            );
            let _ = app.emit(
                "clock-jump",
                JumpEvent {
                    from: last_wall.to_rfc3339(),
                    to: wall_now.to_rfc3339(),
                    skipped,
                },
            );
        }

        if mono_ms > SLOW_TICK_MS {
            log::warn!(
                "rejalashtiruvchi tiki {mono_ms} ms davom etdi (kutilgani {} ms) — \
                 signal kechikishi mumkin",
                TICK.as_millis()
            );
        }

        last_wall = wall_now;
        last_mono = mono_now;

        let date = wall_now.date_naive();
        if date != today {
            today = date;
            fired.clear();
            log::info!("yangi kun: {date}");
        }

        let iso_day = wall_now.weekday().number_from_monday() as u8;
        let state = app.state::<AppState>();

        if !state.enabled() {
            continue;
        }

        for b in state.snapshot() {
            if !b.rings_on(iso_day) {
                continue;
            }
            let key = (date, b.id.clone());
            if fired.contains(&key) {
                continue;
            }

            let Some(due) = at_time(wall_now, b.hour, b.minute) else {
                continue;
            };
            let late = wall_now.signed_duration_since(due).num_seconds();

            if !(0..=CATCH_UP_SECS).contains(&late) {
                continue;
            }

            fired.insert(key);
            ring(&app, &b, late);
        }
    }
}

fn at_time(now: DateTime<Local>, h: u8, m: u8) -> Option<DateTime<Local>> {
    Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), h as u32, m as u32, 0)
        .single()
}

fn skipped_bells(
    bells: &[Bell],
    from: DateTime<Local>,
    to: DateTime<Local>,
) -> Vec<(NaiveDate, String)> {
    if to <= from {
        return Vec::new();
    }

    let mut out = Vec::new();

    let mut day = from.date_naive();
    let last_day = to.date_naive().min(day + chrono::Duration::days(7));

    while day <= last_day {
        let iso = day.weekday().number_from_monday() as u8;
        for b in bells {
            if !b.rings_on(iso) {
                continue;
            }
            let Some(due) = day
                .and_hms_opt(b.hour as u32, b.minute as u32, 0)
                .and_then(|n| Local.from_local_datetime(&n).single())
            else {
                continue;
            };

            if due > from
                && due <= to
                && to.signed_duration_since(due).num_seconds() > CATCH_UP_SECS
            {
                out.push((day, b.id.clone()));
            }
        }
        day += chrono::Duration::days(1);
    }
    out
}

fn belgila_otganlarni(
    app: &AppHandle,
    fired: &mut HashSet<(NaiveDate, String)>,
    now: DateTime<Local>,
) {
    let date = now.date_naive();
    let iso = now.weekday().number_from_monday() as u8;
    let mut n = 0;

    for b in app.state::<AppState>().snapshot() {
        if !b.rings_on(iso) {
            continue;
        }
        let Some(due) = at_time(now, b.hour, b.minute) else {
            continue;
        };
        if due <= now {
            fired.insert((date, b.id));
            n += 1;
        }
    }
    if n > 0 {
        log::info!("ishga tushishda bugungi {n} ta o'tgan signal belgilandi");
    }
}

fn mark_skipped(
    app: &AppHandle,
    fired: &mut HashSet<(NaiveDate, String)>,
    from: DateTime<Local>,
    to: DateTime<Local>,
) -> usize {
    let state = app.state::<AppState>();
    let bells = state.snapshot();
    let mut count = 0;

    for (day, id) in skipped_bells(&bells, from, to) {
        if fired.insert((day, id.clone())) {
            let label = bells
                .iter()
                .find(|b| b.id == id)
                .map(|b| b.label.as_str())
                .unwrap_or("");
            log::warn!(
                "o'tkazib yuborildi: {} ({day})",
                if label.is_empty() { &id } else { label }
            );
            count += 1;
        }
    }
    count
}

fn ring(app: &AppHandle, b: &Bell, late: i64) {
    let time = format!("{:02}:{:02}", b.hour, b.minute);
    log::info!("SIGNAL {time} — {} (kechikish {late}s)", b.label);

    let app = app.clone();
    let bell = b.clone();
    std::thread::Builder::new()
        .name("bell-ring".into())
        .spawn(move || {
            let state = app.state::<AppState>();

            let kind = crate::audio::Kind::Bell;
            state
                .audio
                .play(kind, state.sound_file(kind), state.volume());

            let _ = app.emit(
                "bell-ring",
                RingEvent {
                    id: bell.id.clone(),
                    label: bell.label,
                    time,
                },
            );
        })
        .ok();
}
