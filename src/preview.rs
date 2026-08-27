//! SVG / PNG 2D exports (Rust). 3D preview is the printable mesh + Three.js.

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use image::{ImageBuffer, Rgb, RgbImage};

use crate::design::{parse_rgb, DesignSpec};
use crate::logo;
use crate::qr_gen::QrMatrix;

pub fn svg_qr_2d(spec: &DesignSpec, qr: &QrMatrix) -> String {
    let n = qr.size as f32;
    let quiet = 4.0;
    let dim = n + quiet * 2.0;
    let px = 512.0 / dim;
    let shape = module_kind(&spec.module_shape);
    let mut out = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512" width="512" height="512" role="img" aria-label="QR preview">
  <rect width="512" height="512" fill="{base}"/>
"##,
        base = xml_color(&spec.color_base)
    );

    let logo_href = if logo::has_logo(spec) {
        logo::data_url(spec)
    } else {
        None
    };
    let draw_emoji = logo_href.is_none() && !spec.emoji.trim().is_empty();
    let punch = logo_href.is_some() || draw_emoji;

    for y in 0..qr.size {
        for x in 0..qr.size {
            if !qr.dark(x, y) {
                continue;
            }
            if punch && logo::in_center(x, y, qr.size) {
                continue;
            }
            let px0 = (quiet + x as f32) * px;
            let py0 = (quiet + y as f32) * px;
            let s = px * 0.92;
            let kind = if qr.is_functional(x, y) { "square" } else { shape };
            match kind {
                "dots" => {
                    out.push_str(&format!(
                        r##"  <circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="{fg}"/>"##,
                        px0 + px * 0.5,
                        py0 + px * 0.5,
                        s * 0.48,
                        fg = xml_color(&spec.color_fg)
                    ));
                }
                "rounded" => {
                    out.push_str(&format!(
                        r##"  <rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" rx="{:.2}" fill="{fg}"/>"##,
                        px0 + px * 0.04,
                        py0 + px * 0.04,
                        s,
                        s,
                        s * 0.28,
                        fg = xml_color(&spec.color_fg)
                    ));
                }
                _ => {
                    out.push_str(&format!(
                        r##"  <rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="{fg}"/>"##,
                        px0 + px * 0.04,
                        py0 + px * 0.04,
                        s,
                        s,
                        fg = xml_color(&spec.color_fg)
                    ));
                }
            }
            out.push('\n');
        }
    }

    // Center badge: uploaded logo, or emoji — same window the 3D stamp uses.
    let (cx, cy, badge) = center_badge_px(qr.size, quiet, px);
    if let Some(href) = logo_href {
        let pad = badge * 0.1;
        out.push_str(&format!(
            r##"  <rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" rx="8" fill="{base}" stroke="{fg}" stroke-width="3"/>
  <image href="{href}" xlink:href="{href}" x="{ix:.2}" y="{iy:.2}" width="{iw:.2}" height="{ih:.2}" preserveAspectRatio="xMidYMid meet"/>
"##,
            cx - badge * 0.5,
            cy - badge * 0.5,
            badge,
            badge,
            base = xml_color(&spec.color_base),
            fg = xml_color(&spec.color_fg),
            href = xml_attr(&href),
            ix = cx - badge * 0.5 + pad,
            iy = cy - badge * 0.5 + pad,
            iw = badge - pad * 2.0,
            ih = badge - pad * 2.0,
        ));
    } else if draw_emoji {
        out.push_str(&format!(
            r##"  <rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" rx="8" fill="{base}" stroke="{fg}" stroke-width="3"/>
  <text x="{cx}" y="{cy}" text-anchor="middle" dominant-baseline="central" font-size="{fs:.0}" font-family="Apple Color Emoji, Segoe UI Emoji, Noto Color Emoji, sans-serif">{emoji}</text>
"##,
            cx - badge * 0.5,
            cy - badge * 0.5,
            badge,
            badge,
            base = xml_color(&spec.color_base),
            fg = xml_color(&spec.color_fg),
            cx = cx,
            cy = cy,
            fs = badge * 0.62,
            emoji = xml_text(spec.emoji.trim())
        ));
    }

    out.push_str("</svg>");
    out
}

pub fn png_base64(spec: &DesignSpec, qr: &QrMatrix) -> Result<(String, String), String> {
    let n = qr.size;
    let quiet = 4;
    let dim = n + quiet * 2;
    let scale = (512 / dim).max(4);
    let w = dim * scale;
    let (br, bg, bb) = parse_rgb(&spec.color_base);
    let (fr, fg, fb) = parse_rgb(&spec.color_fg);
    let fg_px = Rgb([fr, fg, fb]);
    let shape = module_kind(&spec.module_shape);
    let logo_img = if logo::has_logo(spec) {
        logo::decode_logo(&spec.logo_png)
    } else {
        None
    };
    let punch = logo_img.is_some();
    let mut img: RgbImage = ImageBuffer::from_pixel(w as u32, w as u32, Rgb([br, bg, bb]));
    for y in 0..n {
        for x in 0..n {
            if !qr.dark(x, y) {
                continue;
            }
            if punch && logo::in_center(x, y, n) {
                continue;
            }
            let x0 = (quiet + x) * scale;
            let y0 = (quiet + y) * scale;
            let kind = if qr.is_functional(x, y) { "square" } else { shape };
            paint_module(&mut img, x0, y0, scale, kind, fg_px);
        }
    }
    if let Some(logo_img) = logo_img {
        let (o, win) = logo::center_window(n);
        let x0 = ((quiet + o) * scale) as u32;
        let y0 = ((quiet + o) * scale) as u32;
        let side = (win * scale) as u32;
        for dy in 0..side {
            for dx in 0..side {
                img.put_pixel(x0 + dx, y0 + dy, Rgb([br, bg, bb]));
            }
        }
        let inner = ((side as f32 * 0.82).round().max(8.0) as u32).min(side);
        let resized = logo_img
            .resize(inner, inner, image::imageops::FilterType::Triangle)
            .to_rgba8();
        let ox = x0 + side.saturating_sub(resized.width()) / 2;
        let oy = y0 + side.saturating_sub(resized.height()) / 2;
        for (px, py, pix) in resized.enumerate_pixels() {
            let a = pix[3] as u16;
            if a == 0 {
                continue;
            }
            let x = ox.saturating_add(px);
            let y = oy.saturating_add(py);
            if x >= img.width() || y >= img.height() {
                continue;
            }
            let dest = img.get_pixel_mut(x, y);
            dest[0] = ((pix[0] as u16 * a + dest[0] as u16 * (255 - a)) / 255) as u8;
            dest[1] = ((pix[1] as u16 * a + dest[1] as u16 * (255 - a)) / 255) as u8;
            dest[2] = ((pix[2] as u16 * a + dest[2] as u16 * (255 - a)) / 255) as u8;
        }
    }
    let mut buf = Vec::new();
    let enc = image::codecs::png::PngEncoder::new(&mut buf);
    use image::ImageEncoder;
    enc.write_image(img.as_raw(), w as u32, w as u32, image::ExtendedColorType::Rgb8)
        .map_err(|e| e.to_string())?;
    Ok(("placaqr.png".into(), B64.encode(buf)))
}

fn xml_color(c: &str) -> String {
    let (r, g, b) = parse_rgb(c);
    format!("#{r:02x}{g:02x}{b:02x}")
}

fn paint_module(img: &mut RgbImage, x0: usize, y0: usize, scale: usize, kind: &str, color: Rgb<u8>) {
    let s = scale as f32;
    let cx = x0 as f32 + s * 0.5;
    let cy = y0 as f32 + s * 0.5;
    match kind {
        "dots" => {
            let r = s * 0.92 * 0.48;
            let r2 = r * r;
            let min_x = (cx - r).floor().max(0.0) as usize;
            let max_x = (cx + r).ceil() as usize;
            let min_y = (cy - r).floor().max(0.0) as usize;
            let max_y = (cy + r).ceil() as usize;
            for py in min_y..=max_y.min(img.height() as usize - 1) {
                for px in min_x..=max_x.min(img.width() as usize - 1) {
                    let dx = px as f32 + 0.5 - cx;
                    let dy = py as f32 + 0.5 - cy;
                    if dx * dx + dy * dy <= r2 {
                        img.put_pixel(px as u32, py as u32, color);
                    }
                }
            }
        }
        "rounded" => {
            let pad = s * 0.04;
            let w = s * 0.92;
            let rx = (w * 0.28).min(w * 0.5);
            let left = x0 as f32 + pad;
            let top = y0 as f32 + pad;
            let right = left + w;
            let bottom = top + w;
            let min_x = left.floor().max(0.0) as usize;
            let max_x = right.ceil() as usize;
            let min_y = top.floor().max(0.0) as usize;
            let max_y = bottom.ceil() as usize;
            for py in min_y..=max_y.min(img.height() as usize - 1) {
                for px in min_x..=max_x.min(img.width() as usize - 1) {
                    let fx = px as f32 + 0.5;
                    let fy = py as f32 + 0.5;
                    if in_rounded_rect(fx, fy, left, top, right, bottom, rx) {
                        img.put_pixel(px as u32, py as u32, color);
                    }
                }
            }
        }
        _ => {
            let pad = ((s * 0.04).round() as usize).min(scale / 4);
            let x1 = (x0 + scale.saturating_sub(pad)).min(img.width() as usize);
            let y1 = (y0 + scale.saturating_sub(pad)).min(img.height() as usize);
            for py in (y0 + pad)..y1 {
                for px in (x0 + pad)..x1 {
                    img.put_pixel(px as u32, py as u32, color);
                }
            }
        }
    }
}

fn in_rounded_rect(x: f32, y: f32, l: f32, t: f32, r: f32, b: f32, rx: f32) -> bool {
    if x < l || x > r || y < t || y > b {
        return false;
    }
    let cx = if x < l + rx {
        l + rx
    } else if x > r - rx {
        r - rx
    } else {
        return true;
    };
    let cy = if y < t + rx {
        t + rx
    } else if y > b - rx {
        b - rx
    } else {
        return true;
    };
    let dx = x - cx;
    let dy = y - cy;
    dx * dx + dy * dy <= rx * rx
}

fn xml_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn xml_attr(s: &str) -> String {
    xml_text(s).replace('"', "&quot;")
}

fn module_kind(shape: &str) -> &'static str {
    match shape.trim().to_ascii_lowercase().as_str() {
        "dots" | "dot" | "circle" | "circles" => "dots",
        "rounded" | "round" => "rounded",
        _ => "square",
    }
}

pub fn svg_file_base64(svg: &str) -> (String, String) {
    ("placaqr.svg".into(), B64.encode(svg.as_bytes()))
}

pub fn svg_data_uri(svg: &str) -> String {
    format!("data:image/svg+xml;base64,{}", B64.encode(svg.as_bytes()))
}

fn center_badge_px(qr_size: usize, quiet: f32, px: f32) -> (f32, f32, f32) {
    let (origin, w) = logo::center_window(qr_size);
    let badge = w as f32 * px;
    let c = (quiet + origin as f32 + w as f32 * 0.5) * px;
    (c, c, badge)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::DesignSpec;
    use crate::qr_gen;

    #[test]
    fn empty_payload_is_rejected() {
        let err = qr_gen::encode("  ").unwrap_err();
        assert!(err.contains("Enter a link"));
    }

    #[test]
    fn svg_badge_matches_center_window() {
        let qr = qr_gen::encode("https://example.com/this/path/is/long/enough/for/version-2")
            .unwrap();
        let mut spec = DesignSpec::default();
        spec.emoji = "⭐".into();
        spec.logo_png.clear();
        let svg = svg_qr_2d(&spec, &qr);
        let quiet = 4.0_f32;
        let px = 512.0 / (qr.size as f32 + quiet * 2.0);
        let (_, _, badge) = center_badge_px(qr.size, quiet, px);
        assert!(
            svg.contains(&format!("width=\"{:.2}\"", badge)),
            "badge size should follow center_window, not a fixed 7 modules"
        );
    }

    #[test]
    fn svg_omits_object_label() {
        let qr = qr_gen::encode("https://example.com").unwrap();
        let mut spec = DesignSpec::default();
        spec.label = "Scan and review".into();
        spec.emoji.clear();
        let svg = svg_qr_2d(&spec, &qr);
        assert!(
            !svg.contains("Scan and review"),
            "2D QR should not include the 3D object label"
        );
    }

    #[test]
    fn svg_circles_when_dots_selected() {
        let qr = qr_gen::encode("https://example.com").unwrap();
        let mut spec = DesignSpec::default();
        spec.module_shape = "circles".into();
        spec.emoji.clear();
        let svg = svg_qr_2d(&spec, &qr);
        assert!(svg.contains("<circle"), "dot shape Circles must draw circle modules");
        assert!(
            svg.contains("<rect x="),
            "finder / timing modules must stay square so phones can lock on"
        );
        assert!(qr.is_functional(0, 0));
        assert!(!qr.is_functional(qr.size / 2, qr.size / 2));
    }

    #[test]
    fn svg_colors_are_safe_hex() {
        let qr = qr_gen::encode("https://example.com").unwrap();
        let mut spec = DesignSpec::default();
        spec.color_base = r#"#f4f7f5"/><script>alert(1)</script>"#.into();
        spec.color_fg = "not-a-color".into();
        spec.emoji.clear();
        let svg = svg_qr_2d(&spec, &qr);
        assert!(!svg.contains("<script"));
        assert!(svg.contains("fill=\"#f4f7f5\""));
        assert!(svg.contains("fill=\"#000000\""));
    }
}
