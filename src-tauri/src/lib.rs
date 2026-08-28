mod audio;
mod bluetooth;
mod commands;
mod db;
mod models;
mod routing;
mod scheduler;
mod sleep;
mod state;

use tauri::{Emitter, Manager};
use tauri_plugin_autostart::MacosLauncher;

use audio::Audio;
use db::Db;
use state::AppState;

#[cfg(target_os = "linux")]
fn avoid_webkit_teardown_crash() {
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
}

#[cfg(not(target_os = "linux"))]
fn avoid_webkit_teardown_crash() {}

#[cfg(target_os = "linux")]
fn silence_gtk() {
    use std::ffi::c_void;

    unsafe extern "C" fn jim(
        _domain: *const i8,
        _level: u32,
        _message: *const i8,
        _data: *mut c_void,
    ) {
    }

    #[link(name = "glib-2.0")]
    extern "C" {
        fn g_log_set_default_handler(
            func: unsafe extern "C" fn(*const i8, u32, *const i8, *mut c_void),
            data: *mut c_void,
        ) -> *mut c_void;
    }

    unsafe { g_log_set_default_handler(jim, std::ptr::null_mut()) };
}

#[cfg(not(target_os = "linux"))]
fn silence_gtk() {}

pub fn set_autostart(app: &tauri::AppHandle, on: bool) {
    use tauri_plugin_autostart::ManagerExt;
    let mgr = app.autolaunch();
    let now = mgr.is_enabled().unwrap_or(false);
    if now == on {
        return;
    }
    let r = if on { mgr.enable() } else { mgr.disable() };
    match r {
        Ok(()) => log::info!("avtostart {}", if on { "yoqildi" } else { "o'chirildi" }),
        Err(e) => log::error!("avtostartni o'zgartirib bo'lmadi: {e}"),
    }
}

fn build_tray(app: &tauri::AppHandle) -> tauri::Result<()> {
    use tauri::menu::{Menu, MenuItem};
    use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

    let (m_ochish, m_chiqish, m_nom) = match app.state::<AppState>().db.settings().language.as_str()
    {
        "en" => ("Open window", "Quit", "School bell"),
        "ru" => ("Открыть окно", "Выход", "Школьный звонок"),
        _ => ("Oynani ochish", "Chiqish", "Maktab qo'ng'irog'i"),
    };

    let ochish = MenuItem::with_id(app, "ochish", m_ochish, true, None::<&str>)?;
    let chiqish = MenuItem::with_id(app, "chiqish", m_chiqish, true, None::<&str>)?;
    let menyu = Menu::with_items(app, &[&ochish, &chiqish])?;

    TrayIconBuilder::with_id("asosiy")
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip(m_nom)
        .menu(&menyu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, e| match e.id.as_ref() {
            "ochish" => show_window(app),
            "chiqish" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_window(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

fn show_window(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    avoid_webkit_teardown_crash();
    silence_gtk();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let path = app
                .path()
                .app_data_dir()
                .expect("app_data_dir topilmadi")
                .join("bell.db");
            log::info!("baza: {}", path.display());

            let database = Db::open(&path).expect("bazani ochib bo'lmadi");

            let handle = app.handle().clone();
            let sound = Audio::spawn(
                move |st| {
                    let _ = handle.emit("sound-state", st);

                    let bt = bluetooth::current_status(&handle.state::<AppState>());
                    let _ = handle.emit("bt-status", bt);
                },
                |kind, msg| {
                    log::error!("ovoz chiqmadi ({kind}): {msg}");
                },
            );

            let sounds_dir = path.parent().expect("app_data_dir").join("sounds");
            let _ = std::fs::create_dir_all(&sounds_dir);

            let state = AppState::new(database, sound, sounds_dir);
            state.install_built_in();
            let saved = state.db.settings();
            routing::set_preferred_speaker(saved.speaker.clone());
            let state_enabled = saved.enabled;
            if !state_enabled {
                log::warn!("tizim O'CHIRILGAN — signallar chalinmaydi");
            }
            app.manage(state);

            set_autostart(app.handle(), state_enabled);

            sleep::keep_awake(state_enabled);

            build_tray(app.handle())?;

            bluetooth::spawn(app.handle().clone());

            scheduler::spawn(app.handle().clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::schedule_list,
            commands::schedule_save,
            commands::settings_get,
            commands::settings_set,
            commands::sound_pick,
            commands::sound_toggle,
            commands::bt_status,
            commands::speaker_list,
            commands::volume_set,
            commands::app_quit,
            commands::app_tray,
        ])
        .build(tauri::generate_context!())
        .expect("ilovani ishga tushirib bo'lmadi")
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { .. } | tauri::RunEvent::Exit = event {
                app.state::<AppState>().audio.stop_all();
            }
        });
}
