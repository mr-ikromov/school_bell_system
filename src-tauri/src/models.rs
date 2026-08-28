use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bell {
    pub id: String,
    pub hour: u8,
    pub minute: u8,

    pub days: Vec<u8>,
    #[serde(default)]
    pub label: String,
    #[serde(default = "yes")]
    pub enabled: bool,
}

fn yes() -> bool {
    true
}

impl Bell {
    pub fn mins(&self) -> u16 {
        self.hour as u16 * 60 + self.minute as u16
    }

    pub fn is_valid(&self) -> bool {
        self.hour < 24
            && self.minute < 60
            && !self.days.is_empty()
            && self.days.iter().all(|d| (1..=7).contains(d))
    }

    pub fn rings_on(&self, iso_day: u8) -> bool {
        self.enabled && self.days.contains(&iso_day)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub volume: f64,

    pub bell_file: String,

    #[serde(default)]
    pub speaker: Option<String>,

    #[serde(default = "default_language")]
    pub language: String,

    #[serde(default = "yes")]
    pub enabled: bool,
}

fn default_language() -> String {
    "uz".into()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            bell_file: "bell.mp3".into(),
            speaker: None,
            language: default_language(),
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct BtStatus {
    pub connected: bool,

    pub muted: bool,
    pub device: Option<String>,

    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Speaker {
    pub id: String,
    pub name: String,
    pub selected: bool,

    pub usable: bool,
}
