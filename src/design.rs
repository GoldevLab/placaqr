//! Design DTO shared by UI and generators.

use resuma::prelude::*;

#[data]
#[derive(Debug)]
pub struct DesignSpec {
    /// Encoded QR payload (URL, WIFI:…, plain text, …).
    pub payload: String,
    /// `stand` | `keychain` | `plaque` | `coin` | `tile`
    pub object: String,
    pub label: String,
    pub emoji: String,
    /// Small PNG as a `data:image/png;base64,…` URL (empty if none). Shown in the QR center.
    pub logo_png: String,
    pub size_mm: f32,
    pub relief_mm: f32,
    pub color_base: String,
    pub color_fg: String,
    /// `square` | `rounded` | `dots` (affects 2D; 3D uses square modules for scan reliability)
    pub module_shape: String,
    /// Yaw degrees (kept for compatibility; 3D orbit is client-side).
    pub rot_y: f32,
    /// When true, server also returns the printable mesh for the WebGL preview.
    pub with_3d: bool,
    /// Pocket on the back of flush tiles for a 6×2 mm fridge magnet.
    pub magnet: bool,
}

impl Default for DesignSpec {
    fn default() -> Self {
        Self {
            payload: "https://g.page/r/example/review".into(),
            object: "stand".into(),
            label: "Scan and review".into(),
            emoji: "".into(),
            logo_png: String::new(),
            size_mm: 55.0,
            relief_mm: 0.8,
            color_base: "#fafafa".into(),
            color_fg: "#12111a".into(),
            module_shape: "square".into(),
            rot_y: 35.0,
            with_3d: false,
            magnet: false,
        }
    }
}

#[data]
#[derive(Debug)]
pub struct MeshPartPreview {
    pub color: String,
    pub positions: Vec<f32>,
}

#[data]
#[derive(Debug)]
pub struct PreviewResult {
    pub svg_2d: String,
    pub mesh: Vec<MeshPartPreview>,
    pub module_mm: f32,
    pub contrast: String,
    pub hint: String,
    pub modules: u32,
}

#[data]
#[derive(Debug)]
pub struct ExportResult {
    pub filename: String,
    pub mime: String,
    pub base64: String,
}

pub fn contrast_label(base: &str, fg: &str) -> (&'static str, &'static str) {
    let (br, bg, bb) = parse_hex(base);
    let (fr, fg_, fb) = parse_hex(fg);
    let lb = relative_luminance(br, bg, bb);
    let lf = relative_luminance(fr, fg_, fb);
    let ratio = if lb > lf {
        (lb + 0.05) / (lf + 0.05)
    } else {
        (lf + 0.05) / (lb + 0.05)
    };
    if ratio >= 7.0 {
        ("high", "Looks good to scan")
    } else if ratio >= 3.0 {
        ("medium", "Contrast is OK — test with a phone")
    } else {
        ("low", "Low contrast — may not scan")
    }
}

fn parse_hex(s: &str) -> (f32, f32, f32) {
    let s = s.trim().trim_start_matches('#');
    if s.len() >= 6 {
        let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0) as f32 / 255.0;
        let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0) as f32 / 255.0;
        let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0) as f32 / 255.0;
        (r, g, b)
    } else {
        (0.0, 0.0, 0.0)
    }
}

fn relative_luminance(r: f32, g: f32, b: f32) -> f32 {
    fn lin(c: f32) -> f32 {
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b)
}

pub fn parse_rgb(s: &str) -> (u8, u8, u8) {
    let s = s.trim().trim_start_matches('#');
    if s.len() >= 6 {
        (
            u8::from_str_radix(&s[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&s[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&s[4..6], 16).unwrap_or(0),
        )
    } else {
        (0, 0, 0)
    }
}
