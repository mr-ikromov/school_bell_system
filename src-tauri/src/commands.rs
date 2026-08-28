use std::path::PathBuf;

use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::DialogExt;

use crate::audio::Kind;
use crate::models::{Bell, BtStatus, Settings, Speaker};
use crate::state::AppState;

type R<T> = Result<T, String>;

#[tauri::command]
pub fn schedule_list(state: State<'_, AppState>) -> R<Vec<Bell>> {
    state.db.bells()
}

#[tauri::command]
pub fn schedule_save(state: State<'_, AppState>, items: Vec<Bell>) -> R<usize> {
    let written = state.db.replace_bells(&items)?;

    let stored = state.db.bells()?;
    if stored.len() != items.len() {
        log::warn!(
            "jadvaldan {} ta yozuv rad etildi (takror yoki noto'g'ri)",
            items.len() - stored.len()
        );
    }
    state.set_bells(stored);
    Ok(written)
}

#[tauri::command]
pub fn settings_get(state: State<'_, AppState>) -> Settings {
    state.db.settings()
}

#[tauri::command]
pub fn settings_set(app: AppHandle, state: State<'_, AppState>, settings: Settings) -> R<()> {
    let was = state.db.settings();
    state.db.set_settings(&settings)?;
    crate::routing::set_preferred_speaker(settings.speaker.clone());

    if was.volume != settings.volume {
        state.audio.set_volume(settings.volume as f32);
    }

    if was.enabled != settings.enabled {
        crate::set_autostart(&app, settings.enabled);
        crate::sleep::keep_awake(settings.enabled);
        if settings.enabled {
            log::info!("tizim yoqildi");

            crate::routing::keep_others_off_speaker(&crate::routing::own_marker());
        } else {
            log::info!("tizim o'chirildi — jadval, ovoz va marshrut to'xtatildi");

            state.audio.stop_all();

            crate::routing::release_to_speaker();
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn sound_pick(app: AppHandle, kind: String) -> R<Option<String>> {
    if kind != "bell" {
        return Err("bell-only".into());
    }

    let (tx, mut rx) = tauri::async_runtime::channel(1);
    app.dialog()
        .file()
        .add_filter("Ovoz fayli", &["mp3", "wav", "ogg", "flac"])
        .pick_file(move |picked| {
            let _ = tx.try_send(picked);
        });

    let Some(picked) = rx.recv().await.flatten() else {
        return Ok(None);
    };
    let src = PathBuf::from(picked.to_string());

    crate::audio::probe(&src)?;

    let state = app.state::<AppState>();
    state.install_sound(Kind::Bell, &src)?;

    if state.audio.is_playing(Kind::Bell) {
        state.audio.stop(Kind::Bell);
    }

    let shown = src
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("bell.mp3")
        .to_string();

    let mut settings = state.db.settings();
    settings.bell_file = shown.clone();
    state.db.set_settings(&settings)?;

    log::info!("qo'ng'iroq ovozi almashtirildi: {shown}");
    Ok(Some(shown))
}

#[tauri::command]
pub fn sound_toggle(state: State<'_, AppState>, kind: String) -> R<bool> {
    let k = Kind::parse(&kind).ok_or("unknown-kind")?;

    if !state.enabled() {
        return Err("system-off".into());
    }

    if state.audio.is_playing(k) {
        state.audio.stop(k);
        return Ok(false);
    }

    state.audio.play(k, state.sound_file(k), state.volume());
    Ok(true)
}

#[tauri::command]
pub fn bt_status(state: State<'_, AppState>) -> BtStatus {
    crate::bluetooth::current_status(&state)
}

#[tauri::command]
pub fn speaker_list(state: State<'_, AppState>) -> Vec<Speaker> {
    let chosen = state.db.settings().speaker;
    let active = crate::routing::bluetooth_sink().map(|s| s.id);

    crate::routing::bluetooth_devices()
        .into_iter()
        .map(|d| Speaker {
            selected: d.usable
                && match &chosen {
                    Some(c) => *c == d.id,
                    None => active.as_deref() == Some(d.id.as_str()),
                },
            id: d.id,
            name: d.name,
            usable: d.usable,
        })
        .collect()
}

#[tauri::command]
pub fn volume_set(state: State<'_, AppState>, volume: f64) {
    state.audio.set_volume(volume.clamp(0.0, 1.0) as f32);
}

#[tauri::command]
pub fn app_quit(app: AppHandle, state: State<'_, AppState>) {
    let mut s = state.db.settings();
    if s.enabled {
        s.enabled = false;
        let _ = state.db.set_settings(&s);
        crate::set_autostart(&app, false);
        crate::routing::release_to_speaker();
        crate::sleep::keep_awake(false);
        log::info!("qizil tugma: tizim o'chirildi, ilova yopilmoqda");
    }
    state.audio.stop_all();
    app.exit(0);
}

#[tauri::command]
pub fn app_tray(app: AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
        log::info!("oyna trayga yashirildi — tizim ishlamoqda");
    }
}
