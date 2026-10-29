//! Completed PlacaQR downloads. Totals stay in `RESUMA_DATA_DIR/activity.json`.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

static STATE: std::sync::LazyLock<Mutex<Activity>> =
    std::sync::LazyLock::new(|| Mutex::new(Activity::load_file()));
static NOTES: std::sync::LazyLock<Mutex<HashMap<String, (u64, usize)>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy)]
pub enum Kind {
    ThreeMf,
    Stl,
    Png,
    Svg,
    Stand,
    Tile,
    Keychain,
    Plaque,
    Coin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub three_mf: u64,
    pub stl: u64,
    pub png: u64,
    pub svg: u64,
    pub stand: u64,
    pub tile: u64,
    pub keychain: u64,
    pub plaque: u64,
    pub coin: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Activity {
    three_mf: u64,
    stl: u64,
    png: u64,
    svg: u64,
    stand: u64,
    tile: u64,
    keychain: u64,
    plaque: u64,
    coin: u64,
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            three_mf: 0,
            stl: 0,
            png: 0,
            svg: 0,
            stand: 0,
            tile: 0,
            keychain: 0,
            plaque: 0,
            coin: 0,
        }
    }
}

impl Activity {
    fn load_file() -> Self {
        let path = store_path();
        let Ok(raw) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&raw).unwrap_or_default()
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            three_mf: self.three_mf,
            stl: self.stl,
            png: self.png,
            svg: self.svg,
            stand: self.stand,
            tile: self.tile,
            keychain: self.keychain,
            plaque: self.plaque,
            coin: self.coin,
        }
    }

    fn add(&mut self, kind: Kind) {
        match kind {
            Kind::ThreeMf => self.three_mf = self.three_mf.saturating_add(1),
            Kind::Stl => self.stl = self.stl.saturating_add(1),
            Kind::Png => self.png = self.png.saturating_add(1),
            Kind::Svg => self.svg = self.svg.saturating_add(1),
            Kind::Stand => self.stand = self.stand.saturating_add(1),
            Kind::Tile => self.tile = self.tile.saturating_add(1),
            Kind::Keychain => self.keychain = self.keychain.saturating_add(1),
            Kind::Plaque => self.plaque = self.plaque.saturating_add(1),
            Kind::Coin => self.coin = self.coin.saturating_add(1),
        }
    }

    fn flush(&self) {
        let path = store_path();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let body = serde_json::json!({
            "three_mf": self.three_mf,
            "stl": self.stl,
            "png": self.png,
            "svg": self.svg,
            "stand": self.stand,
            "tile": self.tile,
            "keychain": self.keychain,
            "plaque": self.plaque,
            "coin": self.coin,
            "updated_at": now_secs(),
        });
        let tmp = path.with_extension("json.tmp");
        if std::fs::write(&tmp, body.to_string()).is_ok() {
            let _ = std::fs::rename(&tmp, &path);
        }
    }
}

fn lock() -> std::sync::MutexGuard<'static, Activity> {
    STATE.lock().unwrap_or_else(|err| err.into_inner())
}

fn store_path() -> PathBuf {
    PathBuf::from(std::env::var("RESUMA_DATA_DIR").unwrap_or_else(|_| ".resuma".into()))
        .join("activity.json")
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn record(kind: Kind) {
    let mut guard = lock();
    guard.add(kind);
    guard.flush();
}

pub fn record_export(format: &str, object: &str) {
    let mut guard = lock();
    let mut wrote = false;
    if let Some(kind) = format_kind(format) {
        guard.add(kind);
        wrote = true;
    }
    if let Some(kind) = object_kind(object) {
        guard.add(kind);
        wrote = true;
    }
    if wrote {
        guard.flush();
    }
}

pub fn format_kind(format: &str) -> Option<Kind> {
    match format {
        "3mf" => Some(Kind::ThreeMf),
        "stl" => Some(Kind::Stl),
        "png" => Some(Kind::Png),
        "svg" => Some(Kind::Svg),
        _ => None,
    }
}

pub fn object_kind(object: &str) -> Option<Kind> {
    match object {
        "stand" => Some(Kind::Stand),
        "tile" => Some(Kind::Tile),
        "keychain" => Some(Kind::Keychain),
        "plaque" => Some(Kind::Plaque),
        "coin" => Some(Kind::Coin),
        _ => None,
    }
}

/// Browser-side PNG saves are not a server job. Cap how often one IP can note them.
pub fn allow_local_note(ip: &str) -> bool {
    const MAX: usize = 40;
    const WINDOW: u64 = 600;
    let now = now_secs();
    let mut map = NOTES.lock().unwrap_or_else(|err| err.into_inner());
    if map.len() > 4_000 {
        map.retain(|_, (start, _)| now.saturating_sub(*start) <= WINDOW);
    }
    let hit = map.entry(ip.to_string()).or_insert((now, 0));
    if now.saturating_sub(hit.0) > WINDOW {
        *hit = (now, 0);
    }
    if hit.1 >= MAX {
        return false;
    }
    hit.1 += 1;
    true
}

pub fn snapshot() -> Snapshot {
    lock().snapshot()
}

pub fn grouped(n: u64) -> String {
    let raw = n.to_string();
    let mut out = String::with_capacity(raw.len() + raw.len() / 3);
    for (i, ch) in raw.chars().enumerate() {
        if i > 0 && (raw.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out
}
