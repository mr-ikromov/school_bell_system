use std::path::{Path, PathBuf};
use std::sync::RwLock;

use crate::audio::{Audio, Kind};
use crate::db::Db;
use crate::models::Bell;

const EXTS: [&str; 4] = ["mp3", "wav", "ogg", "flac"];

const BUILT_IN: [(&str, &[u8]); 2] = [
    ("anthem.mp3", include_bytes!("../assets/anthem.mp3")),
    ("alarm.mp3", include_bytes!("../assets/alarm.mp3")),
];

pub struct AppState {
    pub db: Db,
    pub audio: Audio,

    pub sounds: PathBuf,

    pub bells: RwLock<Vec<Bell>>,
}

impl AppState {
    pub fn new(db: Db, audio: Audio, sounds: PathBuf) -> Self {
        let bells = db.bells().unwrap_or_default();
        Self {
            db,
            audio,
            sounds,
            bells: RwLock::new(bells),
        }
    }

    pub fn install_built_in(&self) {
        if let Err(e) = std::fs::create_dir_all(&self.sounds) {
            log::error!("ovozlar katalogi yaratilmadi: {e}");
            return;
        }
        for (name, data) in BUILT_IN {
            let path = self.sounds.join(name);
            if path.is_file() {
                continue;
            }
            match std::fs::write(&path, data) {
                Ok(()) => log::info!("statik ovoz yozildi: {name} ({} KB)", data.len() / 1024),
                Err(e) => log::error!("{name} yozilmadi: {e}"),
            }
        }
    }

    pub fn sound_file(&self, kind: Kind) -> Option<PathBuf> {
        EXTS.iter()
            .map(|e| self.sounds.join(format!("{}.{e}", kind.as_str())))
            .find(|p| p.is_file())
    }

    pub fn install_sound(&self, kind: Kind, src: &Path) -> Result<(), String> {
        install_into(&self.sounds, kind.as_str(), src)
    }

    pub fn volume(&self) -> f32 {
        self.db.settings().volume as f32
    }

    pub fn enabled(&self) -> bool {
        self.db.settings().enabled
    }

    pub fn snapshot(&self) -> Vec<Bell> {
        self.bells.read().map(|g| g.clone()).unwrap_or_default()
    }

    pub fn set_bells(&self, items: Vec<Bell>) {
        if let Ok(mut g) = self.bells.write() {
            *g = items;
        }
    }
}

fn install_into(dir: &Path, name: &str, src: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .filter(|e| EXTS.contains(&e.as_str()))
        .ok_or("unsupported-format")?;

    let tmp = dir.join(format!("{name}.yangi"));
    let _ = std::fs::remove_file(&tmp);
    std::fs::copy(src, &tmp).map_err(|e| e.to_string())?;

    let dst = dir.join(format!("{name}.{ext}"));
    if let Err(e) = std::fs::rename(&tmp, &dst) {
        let _ = std::fs::remove_file(&tmp);
        return Err(e.to_string());
    }

    for e in EXTS {
        let old = dir.join(format!("{name}.{e}"));
        if old != dst {
            let _ = std::fs::remove_file(old);
        }
    }
    Ok(())
}
