use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use rusqlite::{params, Connection};

use crate::models::{Bell, Settings};

#[derive(PartialEq, Eq)]
struct Row {
    mins: i64,
    days: String,
    label: String,
    enabled: bool,
}

impl From<&Bell> for Row {
    fn from(b: &Bell) -> Self {
        Self {
            mins: i64::from(b.mins()),
            days: b
                .days
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(","),
            label: b.label.clone(),
            enabled: b.enabled,
        }
    }
}

pub struct Db(Mutex<Connection>);

pub type Result<T> = std::result::Result<T, String>;

fn err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

impl Db {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(err)?;
        }
        let conn = Connection::open(path).map_err(err)?;

        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(err)?;
        conn.pragma_update(None, "synchronous", "FULL")
            .map_err(err)?;
        conn.pragma_update(None, "foreign_keys", "ON")
            .map_err(err)?;

        conn.pragma_update(None, "journal_size_limit", 65_536)
            .map_err(err)?;
        conn.pragma_update(None, "wal_autocheckpoint", 64)
            .map_err(err)?;

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS bells (
                id      TEXT    PRIMARY KEY,
                mins    INTEGER NOT NULL UNIQUE,   -- soat*60+minut, takrorlanmaydi
                days    TEXT    NOT NULL,          -- "1,2,3,4,5,6"
                label   TEXT    NOT NULL DEFAULT '',
                enabled INTEGER NOT NULL DEFAULT 1
            );

            CREATE TABLE IF NOT EXISTS settings (
                k TEXT PRIMARY KEY,
                v TEXT NOT NULL
            );
            "#,
        )
        .map_err(err)?;

        tozalash(&conn)?;
        Ok(Self(Mutex::new(conn)))
    }

    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn bells(&self) -> Result<Vec<Bell>> {
        let conn = self.conn();
        let mut st = conn
            .prepare("SELECT id, mins, days, label, enabled FROM bells ORDER BY mins")
            .map_err(err)?;

        let rows = st
            .query_map([], |r| {
                let mins: u16 = r.get(1)?;
                let days: String = r.get(2)?;
                Ok(Bell {
                    id: r.get(0)?,
                    hour: (mins / 60) as u8,
                    minute: (mins % 60) as u8,
                    days: days.split(',').filter_map(|d| d.parse().ok()).collect(),
                    label: r.get(3)?,
                    enabled: r.get::<_, i64>(4)? != 0,
                })
            })
            .map_err(err)?;

        rows.collect::<std::result::Result<_, _>>().map_err(err)
    }

    pub fn replace_bells(&self, items: &[Bell]) -> Result<usize> {
        let hozir = self.bells_raw()?;
        let mut conn = self.conn();
        let tx = conn.transaction().map_err(err)?;

        let kerak: Vec<(String, Row)> = items
            .iter()
            .filter(|b| {
                b.is_valid() || {
                    log::warn!("noto'g'ri yozuv tashlab yuborildi: {b:?}");
                    false
                }
            })
            .map(|b| (b.id.clone(), Row::from(b)))
            .collect();

        let mut ozgargan = 0usize;

        {
            let mut del = tx.prepare("DELETE FROM bells WHERE id = ?1").map_err(err)?;
            for id in hozir.keys() {
                if !kerak.iter().any(|(k, _)| k == id) {
                    ozgargan += del.execute(params![id]).map_err(err)?;
                }
            }
        }

        {
            let mut up = tx
                .prepare(
                    "INSERT INTO bells (id, mins, days, label, enabled) VALUES (?1,?2,?3,?4,?5)
                     ON CONFLICT(id) DO UPDATE SET
                       mins    = excluded.mins,
                       days    = excluded.days,
                       label   = excluded.label,
                       enabled = excluded.enabled",
                )
                .map_err(err)?;

            for (id, r) in &kerak {
                if hozir.get(id) == Some(r) {
                    continue;
                }
                match up.execute(params![id, r.mins, r.days, r.label, r.enabled as i64]) {
                    Ok(n) => ozgargan += n,
                    Err(e) => log::warn!("'{id}' saqlanmadi (takroriy vaqt bo'lishi mumkin): {e}"),
                }
            }
        }

        tx.commit().map_err(err)?;
        Ok(ozgargan)
    }

    fn bells_raw(&self) -> Result<HashMap<String, Row>> {
        let conn = self.conn();
        let mut q = conn
            .prepare("SELECT id, mins, days, label, enabled FROM bells")
            .map_err(err)?;
        let rows = q
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    Row {
                        mins: r.get(1)?,
                        days: r.get(2)?,
                        label: r.get(3)?,
                        enabled: r.get::<_, i64>(4)? != 0,
                    },
                ))
            })
            .map_err(err)?;
        rows.collect::<std::result::Result<_, _>>().map_err(err)
    }

    pub fn settings(&self) -> Settings {
        let conn = self.conn();
        let get = |k: &str| -> Option<String> {
            conn.query_row("SELECT v FROM settings WHERE k = ?1", [k], |r| r.get(0))
                .ok()
        };

        let mut s = Settings::default();
        if let Some(v) = get("volume").and_then(|v| v.parse().ok()) {
            s.volume = v;
        }
        if let Some(v) = get("bell_file") {
            s.bell_file = v;
        }
        s.speaker = get("speaker").filter(|v| !v.is_empty());
        if let Some(v) = get("language").filter(|v| !v.is_empty()) {
            s.language = v;
        }
        if let Some(v) = get("enabled") {
            s.enabled = v != "0";
        }
        s
    }

    pub fn set_settings(&self, s: &Settings) -> Result<()> {
        let yangi = [
            ("volume", s.volume.to_string()),
            ("bell_file", s.bell_file.clone()),
            ("speaker", s.speaker.clone().unwrap_or_default()),
            ("language", s.language.clone()),
            ("enabled", if s.enabled { "1" } else { "0" }.to_string()),
        ];

        let conn = self.conn();
        let mut hozir = conn
            .prepare("SELECT v FROM settings WHERE k = ?1")
            .map_err(err)?;
        let mut put = conn
            .prepare(
                "INSERT INTO settings (k,v) VALUES (?1,?2) ON CONFLICT(k) DO UPDATE SET v = ?2",
            )
            .map_err(err)?;

        for (k, v) in &yangi {
            let eski: Option<String> = hozir.query_row([k], |r| r.get(0)).ok();
            if eski.as_deref() == Some(v.as_str()) {
                continue;
            }
            put.execute(params![k, v]).map_err(err)?;
        }
        Ok(())
    }
}

fn tozalash(conn: &Connection) -> Result<()> {
    let bor: bool = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='ring_log'",
            [],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if bor {
        conn.execute_batch("DROP TABLE IF EXISTS ring_log;")
            .map_err(err)?;
        conn.execute_batch("VACUUM;").map_err(err)?;
        log::info!("eski 'ring_log' jadvali o'chirildi — baza ixchamlashtirildi");
    }

    let _ = conn.pragma_update(None, "wal_checkpoint", "TRUNCATE");
    Ok(())
}
