//! Center logo: decode a small PNG and stamp it into 2D / 3D QR output.

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use image::{DynamicImage, Rgba};

use crate::design::DesignSpec;

pub const MAX_LOGO_CHARS: usize = 100_000;

pub fn has_logo(spec: &DesignSpec) -> bool {
    !spec.logo_png.trim().is_empty()
}

/// Inclusive origin and width in QR modules for the center badge.
pub fn center_window(qr_size: usize) -> (usize, usize) {
    let max_w = qr_size.saturating_sub(8).max(5);
    let w = ((qr_size as f32) * 0.30).round() as usize;
    let w = w.clamp(5, max_w);
    let origin = (qr_size - w) / 2;
    (origin, w)
}

pub fn in_center(x: usize, y: usize, qr_size: usize) -> bool {
    let (o, w) = center_window(qr_size);
    x >= o && x < o + w && y >= o && y < o + w
}

pub fn check_logo(data: &str) -> Result<(), String> {
    if data.len() > MAX_LOGO_CHARS {
        return Err("That logo is too large. Use a simpler PNG or SVG.".into());
    }
    Ok(())
}

pub fn data_url(spec: &DesignSpec) -> Option<String> {
    let s = spec.logo_png.trim();
    if s.is_empty() {
        return None;
    }
    if s.starts_with("data:image/png") {
        Some(s.to_string())
    } else if s.starts_with("data:") {
        None
    } else {
        Some(format!("data:image/png;base64,{s}"))
    }
}

pub fn decode_logo(data: &str) -> Option<DynamicImage> {
    let s = data.trim();
    if s.is_empty() {
        return None;
    }
    let b64 = s.split(',').next_back().unwrap_or(s);
    let bytes = B64.decode(b64.as_bytes()).ok()?;
    image::load_from_memory(&bytes).ok()
}

/// Luma 0–255 as if composited on white (transparent → light).
fn luma_on_white(px: Rgba<u8>) -> u8 {
    let a = px[3] as f32 / 255.0;
    let r = px[0] as f32 * a + 255.0 * (1.0 - a);
    let g = px[1] as f32 * a + 255.0 * (1.0 - a);
    let b = px[2] as f32 * a + 255.0 * (1.0 - a);
    (0.2126 * r + 0.7152 * g + 0.0722 * b).round() as u8
}

/// Dark cells of the logo, sampled onto the inner center window (1-module inset).
/// Transparent pixels are ignored. A light/white mark with no dark pixels is inverted
/// so 3D prints still get a stamp instead of an empty hole.
pub fn logo_dark_cells(spec: &DesignSpec, qr_size: usize) -> Vec<(usize, usize)> {
    let img = match decode_logo(&spec.logo_png) {
        Some(img) => img,
        None => return Vec::new(),
    };
    let (o, w) = center_window(qr_size);
    let inset = 1usize;
    let gw = w.saturating_sub(inset * 2).max(3);
    let rgba = img
        .resize_exact(gw as u32, gw as u32, image::imageops::FilterType::Triangle)
        .to_rgba8();
    let mut dark = Vec::new();
    let mut light = Vec::new();
    for j in 0..gw {
        for i in 0..gw {
            let px = *rgba.get_pixel(i as u32, j as u32);
            if px[3] < 32 {
                continue;
            }
            let cell = (o + inset + i, o + inset + j);
            let y = luma_on_white(px);
            if y <= 168 {
                dark.push(cell);
            } else {
                light.push(cell);
            }
        }
    }
    if dark.is_empty() {
        light
    } else {
        dark
    }
}
