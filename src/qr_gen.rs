//! QR matrix generation (pure Rust via `qrcode`).

use qrcode::types::{Color, EcLevel, Version};
use qrcode::QrCode;

#[derive(Debug)]
pub struct QrMatrix {
    pub size: usize,
    /// row-major, true = dark module
    pub modules: Vec<bool>,
    /// Finder, timing, alignment, format — keep these square when styling dots.
    functional: Vec<bool>,
}

pub fn encode(payload: &str) -> Result<QrMatrix, String> {
    let data = payload.trim();
    if data.is_empty() {
        return Err("Enter a link or text to encode.".into());
    }
    // High EC so center emoji/logo in 2D doesn't break scanning; also helps print defects.
    let code = QrCode::with_error_correction_level(data.as_bytes(), EcLevel::H)
        .or_else(|_| QrCode::with_version(data.as_bytes(), Version::Normal(6), EcLevel::M))
        .map_err(|e| format!("Could not generate the QR: {e}"))?;

    let w = code.width();
    let mut modules = Vec::with_capacity(w * w);
    let mut functional = Vec::with_capacity(w * w);
    for y in 0..w {
        for x in 0..w {
            modules.push(code[(x, y)] == Color::Dark);
            functional.push(code.is_functional(x, y));
        }
    }
    Ok(QrMatrix {
        size: w,
        modules,
        functional,
    })
}

impl QrMatrix {
    pub fn dark(&self, x: usize, y: usize) -> bool {
        self.modules[y * self.size + x]
    }

    pub fn is_functional(&self, x: usize, y: usize) -> bool {
        self.functional[y * self.size + x]
    }
}
