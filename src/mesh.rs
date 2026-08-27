//! Triangle mesh builder for printable QR objects.

use crate::design::DesignSpec;
use crate::logo;
use crate::qr_gen::QrMatrix;

#[derive(Clone, Copy, Debug)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tri {
    pub a: V3,
    pub b: V3,
    pub c: V3,
}

#[derive(Clone, Debug)]
pub struct MeshPart {
    pub name: String,
    /// #RRGGBB
    pub color: String,
    pub tris: Vec<Tri>,
}

#[derive(Clone, Debug)]
pub struct Model3d {
    pub parts: Vec<MeshPart>,
    #[allow(dead_code)]
    pub bounds: (V3, V3),
}

pub fn build_model(spec: &DesignSpec, qr: &QrMatrix) -> Model3d {
    let size = spec.size_mm.max(30.0).min(120.0);
    let relief = spec.relief_mm.max(0.4).min(2.5);
    let base_h = 2.2_f32;
    let quiet = 4usize; // modules of quiet zone baked into margin
    let n = qr.size + quiet * 2;
    let module = size / n as f32;
    let qr_origin = quiet as f32 * module;

    let mut base = MeshPart {
        name: "Base".into(),
        color: normalize_color(&spec.color_base),
        tris: Vec::new(),
    };
    let mut fg = MeshPart {
        name: "QR".into(),
        color: normalize_color(&spec.color_fg),
        tris: Vec::new(),
    };

    match spec.object.as_str() {
        "keychain" => build_keychain(spec, qr, &mut base, &mut fg, size, base_h, relief, module, qr_origin),
        "plaque" => build_plaque(spec, qr, &mut base, &mut fg, size, base_h, relief, module, qr_origin),
        "coin" => build_coin(spec, qr, &mut base, &mut fg, size, base_h, relief, module, qr_origin),
        "tile" => build_tile(spec, qr, &mut base, &mut fg, size, relief, module, qr_origin),
        _ => build_stand(spec, qr, &mut base, &mut fg, size, base_h, relief, module, qr_origin),
    }

    let bounds = compute_bounds(&[&base, &fg]);
    Model3d {
        parts: vec![base, fg],
        bounds,
    }
}

/// Side of the QR square (quiet zone included), in millimetres.
pub fn qr_extent_mm(spec: &DesignSpec) -> f32 {
    let size = spec.size_mm.max(30.0).min(120.0);
    if spec.object == "coin" {
        let r = size * 0.5 + 4.2;
        (r * std::f32::consts::SQRT_2 * 0.9).min(size)
    } else {
        size
    }
}

pub fn module_pitch_mm(spec: &DesignSpec, qr_size: usize) -> f32 {
    qr_extent_mm(spec) / (qr_size as f32 + 8.0)
}

pub fn preview_parts(model: &Model3d) -> Vec<crate::design::MeshPartPreview> {
    model
        .parts
        .iter()
        .filter(|p| !p.tris.is_empty())
        .map(|p| {
            let mut positions = Vec::with_capacity(p.tris.len() * 9);
            for t in &p.tris {
                for v in [t.a, t.b, t.c] {
                    positions.push((v.x * 1000.0).round() / 1000.0);
                    positions.push((v.y * 1000.0).round() / 1000.0);
                    positions.push((v.z * 1000.0).round() / 1000.0);
                }
            }
            crate::design::MeshPartPreview {
                color: p.color.clone(),
                positions,
            }
        })
        .collect()
}

fn normalize_color(c: &str) -> String {
    let c = c.trim();
    if c.starts_with('#') && c.len() >= 7 {
        c[..7].to_ascii_lowercase()
    } else {
        "#000000".into()
    }
}

fn build_stand(
    spec: &DesignSpec,
    qr: &QrMatrix,
    base: &mut MeshPart,
    fg: &mut MeshPart,
    size: f32,
    base_h: f32,
    relief: f32,
    module: f32,
    qr_origin: f32,
) {
    let depth = size * 0.55;
    let angle_deg = 65.0_f32;
    let rad = angle_deg.to_radians();
    let face_h = size + 14.0;
    let foot_d = depth;

    // Foot plate
    add_box(&mut base.tris, 0.0, 0.0, 0.0, size, base_h, foot_d);
    // Back brace wedge (approx as box)
    let brace_t = 3.0;
    add_box(
        &mut base.tris,
        size * 0.5 - brace_t * 0.5,
        0.0,
        foot_d * 0.15,
        brace_t,
        face_h * rad.sin() * 0.85,
        foot_d * 0.55,
    );

    // Vertical-ish face plate (standing along Z, upright in Y)
    let face_z = foot_d * 0.72;
    add_box(&mut base.tris, 0.0, base_h, face_z, size, face_h, 2.4);

    // QR modules on face (outward +Z)
    let z0 = face_z + 2.4;
    add_qr_with_logo(fg, spec, qr, module, relief, |x, y| {
        let px = qr_origin + x as f32 * module;
        let py = base_h + 8.0 + (qr.size - 1 - y) as f32 * module + qr_origin * 0.15;
        (px, py, z0)
    });

    // Label text band
    let label_y = base_h + 2.2;
    extrude_label(
        &mut fg.tris,
        &spec.label,
        3.0,
        label_y,
        z0 + relief * 0.2,
        size - 6.0,
        3.2,
        0.55,
    );
}

fn build_keychain(
    spec: &DesignSpec,
    qr: &QrMatrix,
    base: &mut MeshPart,
    fg: &mut MeshPart,
    size: f32,
    base_h: f32,
    relief: f32,
    module: f32,
    qr_origin: f32,
) {
    let caption = 8.0;
    let tab = 12.0;
    let total_h = caption + size + tab;
    let hole = 4.2;
    let hx = (size - hole) * 0.5;
    let hy = caption + size + (tab - hole) * 0.5;
    add_plate_with_rect_hole(
        &mut base.tris,
        0.0,
        0.0,
        0.0,
        size,
        total_h,
        base_h,
        hx,
        hy,
        hole,
        hole,
    );

    let oy = caption;
    add_qr_with_logo(fg, spec, qr, module, relief, |x, y| {
        let px = qr_origin + x as f32 * module;
        let py = oy + qr_origin + (qr.size - 1 - y) as f32 * module;
        (px, py, base_h)
    });
    extrude_label(
        &mut fg.tris,
        &spec.label,
        2.5,
        1.6,
        base_h + relief * 0.15,
        size - 5.0,
        4.6,
        0.5,
    );
}

fn build_plaque(
    spec: &DesignSpec,
    qr: &QrMatrix,
    base: &mut MeshPart,
    fg: &mut MeshPart,
    size: f32,
    base_h: f32,
    relief: f32,
    module: f32,
    qr_origin: f32,
) {
    let margin = 8.0;
    let hang = 12.0;
    let caption = 10.0;
    let w = size + margin * 2.0;
    let h = hang + size + margin + caption;
    let hole = 3.6;
    add_plate_with_two_holes(
        &mut base.tris,
        0.0,
        0.0,
        0.0,
        w,
        h,
        base_h,
        8.0,
        h - hang * 0.62,
        w - 8.0 - hole,
        h - hang * 0.62,
        hole,
        hole,
    );

    let ox = margin;
    let oy = caption;
    add_qr_with_logo(fg, spec, qr, module, relief, |x, y| {
        let px = ox + qr_origin + x as f32 * module;
        let py = oy + qr_origin + (qr.size - 1 - y) as f32 * module;
        (px, py, base_h)
    });
    extrude_label(
        &mut fg.tris,
        &spec.label,
        margin,
        2.2,
        base_h + relief * 0.15,
        w - margin * 2.0,
        5.2,
        0.55,
    );
}

fn build_coin(
    spec: &DesignSpec,
    qr: &QrMatrix,
    base: &mut MeshPart,
    fg: &mut MeshPart,
    size: f32,
    base_h: f32,
    relief: f32,
    _module: f32,
    _qr_origin: f32,
) {
    let pad = 4.2;
    let r = size * 0.5 + pad;
    let lug_w = 13.0;
    let lug_h = 11.0;
    let hole = 4.0;
    let cx = r;
    let cy = r;
    let qr_side = qr_extent_mm(spec);
    add_cylinder(&mut base.tris, cx, cy, 0.0, r, base_h, 28);
    let lug_x = cx - lug_w * 0.5;
    let lug_y = cy + r - 2.4;
    add_plate_with_rect_hole(
        &mut base.tris,
        lug_x,
        lug_y,
        0.0,
        lug_w,
        lug_h,
        base_h,
        lug_x + (lug_w - hole) * 0.5,
        lug_y + lug_h - hole - 2.0,
        hole,
        hole,
    );
    let ox = cx - qr_side * 0.5;
    let oy = cy - qr_side * 0.5;
    let n = (qr.size + 8) as f32;
    let module = qr_side / n;
    let qr_origin = 4.0 * module;
    add_qr_with_logo(fg, spec, qr, module, relief, |x, y| {
        let px = ox + qr_origin + x as f32 * module;
        let py = oy + qr_origin + (qr.size - 1 - y) as f32 * module;
        (px, py, base_h)
    });
}

/// Flush dual-color coaster: light and dark modules share one top plane.
fn build_tile(
    spec: &DesignSpec,
    qr: &QrMatrix,
    base: &mut MeshPart,
    fg: &mut MeshPart,
    size: f32,
    relief: f32,
    module: f32,
    qr_origin: f32,
) {
    let inlay = relief.clamp(0.4, 1.2);
    let magnet_depth = 2.2_f32;
    let magnet_roof = 0.9_f32;
    let floor = if spec.magnet {
        (magnet_depth + magnet_roof).max(2.4 - inlay)
    } else {
        (2.4 - inlay).max(1.2)
    };
    if spec.magnet {
        add_magnet_pocket(&mut base.tris, 0.0, 0.0, 0.0, size, size, floor);
    } else {
        add_box(&mut base.tris, 0.0, 0.0, 0.0, size, size, floor);
    }
    let q = qr_origin;
    add_box(&mut base.tris, 0.0, 0.0, floor, size, q, inlay);
    add_box(&mut base.tris, 0.0, size - q, floor, size, q, inlay);
    add_box(&mut base.tris, 0.0, q, floor, q, size - 2.0 * q, inlay);
    add_box(&mut base.tris, size - q, q, floor, q, size - 2.0 * q, inlay);

    let n = qr.size;
    let stamps = logo::logo_dark_cells(spec, n);
    let cut = !stamps.is_empty();
    let logo_dark: std::collections::HashSet<(usize, usize)> = stamps.into_iter().collect();
    for y in 0..n {
        for x in 0..n {
            let px = qr_origin + x as f32 * module;
            let py = qr_origin + (n - 1 - y) as f32 * module;
            let dark = if cut && logo::in_center(x, y, n) {
                logo_dark.contains(&(x, y))
            } else {
                qr.dark(x, y)
            };
            if dark {
                add_box(&mut fg.tris, px, py, floor, module, module, inlay);
            } else {
                add_box(&mut base.tris, px, py, floor, module, module, inlay);
            }
        }
    }
}

fn add_magnet_pocket(out: &mut Vec<Tri>, x: f32, y: f32, z: f32, w: f32, h: f32, t: f32) {
    let depth = 2.2_f32.min((t - 0.9).max(0.6));
    let hole = 6.4_f32;
    let hx = x + (w - hole) * 0.5;
    let hy = y + (h - hole) * 0.5;
    add_plate_with_rect_hole(out, x, y, z, w, h, depth, hx, hy, hole, hole);
    let roof = (t - depth).max(0.8);
    add_box(out, x, y, z + depth, w, h, roof);
}

fn add_qr_with_logo<F>(
    fg: &mut MeshPart,
    spec: &DesignSpec,
    qr: &QrMatrix,
    module: f32,
    relief: f32,
    to_world: F,
) where
    F: Fn(usize, usize) -> (f32, f32, f32),
{
    let stamps = logo::logo_dark_cells(spec, qr.size);
    let cut = !stamps.is_empty();
    for y in 0..qr.size {
        for x in 0..qr.size {
            if cut && logo::in_center(x, y, qr.size) {
                continue;
            }
            if !qr.dark(x, y) {
                continue;
            }
            let (px, py, pz) = to_world(x, y);
            add_box(&mut fg.tris, px, py, pz, module * 0.92, module * 0.92, relief);
        }
    }
    if !cut {
        return;
    }
    for (x, y) in stamps {
        let (px, py, pz) = to_world(x, y);
        add_box(&mut fg.tris, px, py, pz, module * 0.92, module * 0.92, relief);
    }
}

fn add_cylinder(out: &mut Vec<Tri>, cx: f32, cy: f32, z: f32, r: f32, h: f32, segs: usize) {
    let segs = segs.max(8);
    let tau = std::f32::consts::TAU;
    let mut bot = Vec::with_capacity(segs);
    let mut top = Vec::with_capacity(segs);
    for i in 0..segs {
        let a = i as f32 / segs as f32 * tau;
        let x = cx + r * a.cos();
        let y = cy + r * a.sin();
        bot.push(V3::new(x, y, z));
        top.push(V3::new(x, y, z + h));
    }
    let c0 = V3::new(cx, cy, z);
    let c1 = V3::new(cx, cy, z + h);
    for i in 0..segs {
        let j = (i + 1) % segs;
        out.push(Tri {
            a: c0,
            b: bot[j],
            c: bot[i],
        });
        out.push(Tri {
            a: c1,
            b: top[i],
            c: top[j],
        });
        quad(out, bot[i], bot[j], top[j], top[i]);
    }
}

fn add_box(out: &mut Vec<Tri>, x: f32, y: f32, z: f32, sx: f32, sy: f32, sz: f32) {
    let p000 = V3::new(x, y, z);
    let p100 = V3::new(x + sx, y, z);
    let p110 = V3::new(x + sx, y + sy, z);
    let p010 = V3::new(x, y + sy, z);
    let p001 = V3::new(x, y, z + sz);
    let p101 = V3::new(x + sx, y, z + sz);
    let p111 = V3::new(x + sx, y + sy, z + sz);
    let p011 = V3::new(x, y + sy, z + sz);
    // bottom
    quad(out, p000, p010, p110, p100);
    // top
    quad(out, p001, p101, p111, p011);
    // sides
    quad(out, p000, p100, p101, p001);
    quad(out, p100, p110, p111, p101);
    quad(out, p110, p010, p011, p111);
    quad(out, p010, p000, p001, p011);
}

fn quad(out: &mut Vec<Tri>, a: V3, b: V3, c: V3, d: V3) {
    out.push(Tri { a, b, c });
    out.push(Tri { a: a, b: c, c: d });
}

/// Axis-aligned plate with a rectangular through-hole (CSG-free tiling).
fn add_plate_with_rect_hole(
    out: &mut Vec<Tri>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    t: f32,
    hx: f32,
    hy: f32,
    hw: f32,
    hh: f32,
) {
    let hx = hx.clamp(x, x + w);
    let hy = hy.clamp(y, y + h);
    let hw = hw.max(0.2).min(x + w - hx);
    let hh = hh.max(0.2).min(y + h - hy);
    if hy > y {
        add_box(out, x, y, z, w, hy - y, t);
    }
    let top_y = hy + hh;
    if top_y < y + h {
        add_box(out, x, top_y, z, w, y + h - top_y, t);
    }
    if hx > x {
        add_box(out, x, hy, z, hx - x, hh, t);
    }
    let right_x = hx + hw;
    if right_x < x + w {
        add_box(out, right_x, hy, z, x + w - right_x, hh, t);
    }
}

/// Plate with two rectangular through-holes on the same Y band (hanging plaque).
fn add_plate_with_two_holes(
    out: &mut Vec<Tri>,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
    t: f32,
    hx1: f32,
    hy: f32,
    hx2: f32,
    _hy2: f32,
    hw: f32,
    hh: f32,
) {
    let hy = hy.clamp(y, y + h);
    let hh = hh.max(0.2).min(y + h - hy);
    let mut holes = [hx1, hx2];
    holes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if hy > y {
        add_box(out, x, y, z, w, hy - y, t);
    }
    let top_y = hy + hh;
    if top_y < y + h {
        add_box(out, x, top_y, z, w, y + h - top_y, t);
    }
    let mut cursor = x;
    for hx in holes {
        let hx = hx.clamp(x, x + w);
        if hx > cursor {
            add_box(out, cursor, hy, z, hx - cursor, hh, t);
        }
        cursor = (hx + hw).min(x + w);
    }
    if cursor < x + w {
        add_box(out, cursor, hy, z, x + w - cursor, hh, t);
    }
}

fn extrude_label(
    out: &mut Vec<Tri>,
    text: &str,
    x0: f32,
    y0: f32,
    z0: f32,
    max_w: f32,
    h: f32,
    depth: f32,
) {
    let t = text.trim();
    if t.is_empty() {
        return;
    }
    let chars: Vec<char> = t.chars().take(24).collect();
    let cell = (max_w / chars.len().max(1) as f32).min(h * 0.7);
    for (i, ch) in chars.iter().enumerate() {
        let glyph = glyph5x7(*ch);
        let gx = x0 + i as f32 * cell;
        for row in 0..7 {
            for col in 0..5 {
                if glyph[row] & (1 << (4 - col)) != 0 {
                    let px = gx + col as f32 * (cell / 5.5);
                    let py = y0 + (6 - row) as f32 * (h / 7.0);
                    add_box(out, px, py, z0, cell / 6.2, h / 8.0, depth);
                }
            }
        }
    }
}

fn glyph5x7(c: char) -> [u8; 7] {
    // Minimal uppercase/digit font
    let c = c.to_ascii_uppercase();
    match c {
        'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'B' => [0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E],
        'C' => [0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E],
        'D' => [0x1E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1E],
        'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        'F' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10],
        'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0E],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'I' => [0x0E, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E],
        'J' => [0x01, 0x01, 0x01, 0x01, 0x11, 0x11, 0x0E],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F],
        'M' => [0x11, 0x1B, 0x15, 0x11, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'P' => [0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10],
        'Q' => [0x0E, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0D],
        'R' => [0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11],
        'S' => [0x0F, 0x10, 0x10, 0x0E, 0x01, 0x01, 0x1E],
        'T' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x1B, 0x11],
        'X' => [0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1F],
        '0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
        '1' => [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
        '2' => [0x0E, 0x11, 0x01, 0x06, 0x08, 0x10, 0x1F],
        '3' => [0x1F, 0x01, 0x02, 0x06, 0x01, 0x11, 0x0E],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        '5' => [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
        '6' => [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E],
        '7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        '9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C],
        ' ' => [0; 7],
        '-' => [0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x0C],
        '!' | '¡' => [0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x04],
        '?' | '¿' => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x00, 0x04],
        'Ñ' | 'ñ' => [0x0A, 0x00, 0x11, 0x19, 0x15, 0x13, 0x11],
        'Á' | 'á' | 'À' | 'à' => [0x04, 0x0E, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'É' | 'é' => [0x04, 0x1F, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        'Í' | 'í' => [0x04, 0x0E, 0x04, 0x04, 0x04, 0x04, 0x0E],
        'Ó' | 'ó' => [0x04, 0x0E, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'Ú' | 'ú' => [0x04, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'Ü' | 'ü' => [0x0A, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        _ => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x00, 0x04],
    }
}

fn compute_bounds(parts: &[&MeshPart]) -> (V3, V3) {
    let mut min = V3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = V3::new(f32::MIN, f32::MIN, f32::MIN);
    for p in parts {
        for t in &p.tris {
            for v in [t.a, t.b, t.c] {
                min.x = min.x.min(v.x);
                min.y = min.y.min(v.y);
                min.z = min.z.min(v.z);
                max.x = max.x.max(v.x);
                max.y = max.y.max(v.y);
                max.z = max.z.max(v.z);
            }
        }
    }
    if min.x == f32::MAX {
        (V3::new(0.0, 0.0, 0.0), V3::new(1.0, 1.0, 1.0))
    } else {
        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::DesignSpec;
    use crate::qr_gen;
    use image::ImageEncoder;

    #[test]
    fn coin_module_pitch_matches_fitted_qr() {
        let qr = qr_gen::encode("https://example.com").unwrap();
        let mut coin = DesignSpec::default();
        coin.object = "coin".into();
        coin.size_mm = 42.0;
        let mut stand = DesignSpec::default();
        stand.object = "stand".into();
        stand.size_mm = 42.0;
        let c = module_pitch_mm(&coin, qr.size);
        let s = module_pitch_mm(&stand, qr.size);
        assert!(
            c < s,
            "coin dots sit in a smaller square than the disc: coin={c} stand={s}"
        );
    }

    #[test]
    fn coin_has_base_and_qr_triangles() {
        let qr = qr_gen::encode("https://g.page/r/example/review").unwrap();
        let mut spec = DesignSpec::default();
        spec.object = "coin".into();
        spec.with_3d = true;
        let model = build_model(&spec, &qr);
        assert_eq!(model.parts[0].name, "Base");
        assert_eq!(model.parts[1].name, "QR");
        assert!(model.parts[0].tris.len() > 40);
        assert!(model.parts[1].tris.len() > 40);
    }

    #[test]
    fn logo_stamps_dark_cells() {
        let img: image::RgbImage = image::ImageBuffer::from_pixel(12, 12, image::Rgb([10, 10, 10]));
        let mut buf = Vec::new();
        image::codecs::png::PngEncoder::new(&mut buf)
            .write_image(img.as_raw(), 12, 12, image::ExtendedColorType::Rgb8)
            .unwrap();
        let mut spec = DesignSpec::default();
        spec.logo_png = format!(
            "data:image/png;base64,{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &buf)
        );
        spec.emoji.clear();
        let qr = qr_gen::encode(&spec.payload).unwrap();
        let model = build_model(&spec, &qr);
        assert!(!crate::logo::logo_dark_cells(&spec, qr.size).is_empty());
        assert!(!model.parts[1].tris.is_empty());
        let svg = crate::preview::svg_qr_2d(&spec, &qr);
        assert!(svg.contains("<image href=\"data:image/png;base64,"));
    }

    #[test]
    fn flush_tile_has_two_parts() {
        let qr = qr_gen::encode("https://g.page/r/example/review").unwrap();
        let mut spec = DesignSpec::default();
        spec.object = "tile".into();
        spec.magnet = true;
        spec.emoji.clear();
        let model = build_model(&spec, &qr);
        assert_eq!(model.parts[0].name, "Base");
        assert_eq!(model.parts[1].name, "QR");
        assert!(model.parts[0].tris.len() > 20);
        assert!(model.parts[1].tris.len() > 20);
        let (min, max) = model.bounds;
        assert!(
            max.z - min.z >= 3.0,
            "magnet tile too thin for a 2 mm magnet: {}",
            max.z - min.z
        );
    }

    #[test]
    fn flush_tile_modules_fill_the_grid() {
        let qr = qr_gen::encode("https://example.com").unwrap();
        let mut spec = DesignSpec::default();
        spec.object = "tile".into();
        spec.size_mm = 50.0;
        spec.emoji.clear();
        spec.logo_png.clear();
        let model = build_model(&spec, &qr);
        let n = qr.size + 8;
        let module = 50.0 / n as f32;
        let mut min_x = f32::MAX;
        for t in &model.parts[1].tris {
            min_x = min_x.min(t.a.x).min(t.b.x).min(t.c.x);
        }
        let mut max_first = f32::MIN;
        for t in &model.parts[1].tris {
            for v in [t.a, t.b, t.c] {
                if v.x <= min_x + module * 1.05 {
                    max_first = max_first.max(v.x);
                }
            }
        }
        assert!(
            (max_first - min_x - module).abs() < 0.08,
            "flush tile modules must abut without 8% gaps: span {}, module {module}",
            max_first - min_x
        );
    }

    #[test]
    fn emoji_does_not_cut_printable_qr() {
        let qr = qr_gen::encode("https://example.com").unwrap();
        let mut with_e = DesignSpec::default();
        with_e.emoji = "⭐".into();
        with_e.logo_png.clear();
        let mut none = DesignSpec::default();
        none.emoji.clear();
        none.logo_png.clear();
        let a = build_model(&with_e, &qr);
        let b = build_model(&none, &qr);
        assert_eq!(a.parts[1].tris.len(), b.parts[1].tris.len());
    }

    #[test]
    fn coin_qr_stays_inside_disc() {
        let payload = format!(
            "https://example.com/reviews/{}",
            "place-id-and-extra-path-to-grow-the-matrix/".repeat(8)
        );
        let qr = qr_gen::encode(&payload).unwrap();
        let mut spec = DesignSpec::default();
        spec.object = "coin".into();
        spec.size_mm = 42.0;
        spec.emoji.clear();
        spec.logo_png.clear();
        spec.payload = payload;
        let model = build_model(&spec, &qr);
        let r = spec.size_mm * 0.5 + 4.2;
        let cx = r;
        let cy = r;
        let limit = (r + 0.2) * (r + 0.2);
        for t in &model.parts[1].tris {
            for v in [t.a, t.b, t.c] {
                let d2 = (v.x - cx) * (v.x - cx) + (v.y - cy) * (v.y - cy);
                assert!(
                    d2 <= limit,
                    "coin QR vertex outside disc: ({}, {}) r={}",
                    v.x,
                    v.y,
                    r
                );
            }
        }
    }

    #[test]
    fn light_logo_still_stamps() {
        let img: image::RgbImage = image::ImageBuffer::from_pixel(12, 12, image::Rgb([250, 250, 250]));
        let mut buf = Vec::new();
        image::codecs::png::PngEncoder::new(&mut buf)
            .write_image(img.as_raw(), 12, 12, image::ExtendedColorType::Rgb8)
            .unwrap();
        let mut spec = DesignSpec::default();
        spec.logo_png = format!(
            "data:image/png;base64,{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &buf)
        );
        spec.emoji.clear();
        let qr = qr_gen::encode(&spec.payload).unwrap();
        assert!(
            !crate::logo::logo_dark_cells(&spec, qr.size).is_empty(),
            "a light logo must still produce stamp cells"
        );
        let none = DesignSpec::default();
        let with_logo = build_model(&spec, &qr);
        let without = build_model(&none, &qr);
        assert_ne!(
            with_logo.parts[1].tris.len(),
            without.parts[1].tris.len(),
            "light logo should change the QR mesh, not leave an empty hole"
        );
    }
}
