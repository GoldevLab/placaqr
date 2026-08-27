//! #[server] actions — preview + export fully in Rust.

use resuma::prelude::*;

use crate::design::{contrast_label, DesignSpec, ExportResult, PreviewResult};
use crate::export3d::{export_3mf_base64, export_stl_base64};
use crate::mesh::{build_model, module_pitch_mm, preview_parts};
use crate::preview::{png_base64, svg_file_base64, svg_qr_2d};
use crate::qr_gen;

pub fn preview_spec(spec: &DesignSpec) -> Result<PreviewResult> {
    preview_spec_ex(spec, false)
}

const MAX_PAYLOAD: usize = 2048;

fn check_payload(payload: &str) -> Result<()> {
    if payload.len() > MAX_PAYLOAD {
        return Err(ResumaError::Validation(
            "That text is too long for a scannable print. Use a shorter link.".into(),
        ));
    }
    Ok(())
}

fn check_spec(spec: &DesignSpec) -> Result<()> {
    check_payload(&spec.payload)?;
    crate::logo::check_logo(&spec.logo_png).map_err(ResumaError::Validation)?;
    Ok(())
}

pub fn preview_spec_ex(spec: &DesignSpec, with_3d: bool) -> Result<PreviewResult> {
    check_spec(spec)?;
    let qr = qr_gen::encode(&spec.payload).map_err(ResumaError::Validation)?;
    let module = module_pitch_mm(spec, qr.size);
    let (level, msg) = contrast_label(&spec.color_base, &spec.color_fg);
    let n = spec.payload.len();
    let hint = if module < 1.2 {
        format!("Too small to scan — increase width or use a shorter link ({n} chars).")
    } else {
        format!("{msg} · {module:.1} mm dots")
    };
    let mesh = if with_3d {
        preview_parts(&build_model(spec, &qr))
    } else {
        Vec::new()
    };
    Ok(PreviewResult {
        svg_2d: svg_qr_2d(spec, &qr),
        mesh,
        module_mm: module,
        contrast: level.into(),
        hint,
        modules: qr.size as u32,
    })
}

#[server]
async fn preview_design(spec: DesignSpec) -> Result<PreviewResult> {
    preview_spec_ex(&spec, spec.with_3d)
}

#[server]
async fn export_design(spec: DesignSpec, format: String) -> Result<ExportResult> {
    check_spec(&spec)?;
    let qr = qr_gen::encode(&spec.payload).map_err(ResumaError::Validation)?;
    match format.as_str() {
        "3mf" => {
            let model = build_model(&spec, &qr);
            let (filename, base64) = export_3mf_base64(&model).map_err(ResumaError::Validation)?;
            Ok(ExportResult {
                filename,
                mime: "model/3mf".into(),
                base64,
            })
        }
        "stl" => {
            let model = build_model(&spec, &qr);
            let (filename, base64) = export_stl_base64(&model).map_err(ResumaError::Validation)?;
            Ok(ExportResult {
                filename,
                mime: "model/stl".into(),
                base64,
            })
        }
        "png" => {
            let (filename, base64) = png_base64(&spec, &qr).map_err(ResumaError::Validation)?;
            Ok(ExportResult {
                filename,
                mime: "image/png".into(),
                base64,
            })
        }
        "svg" => {
            let svg = svg_qr_2d(&spec, &qr);
            let (filename, base64) = svg_file_base64(&svg);
            Ok(ExportResult {
                filename,
                mime: "image/svg+xml".into(),
                base64,
            })
        }
        other => Err(ResumaError::Validation(format!("Unknown format: {other}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_preview_has_svg() {
        let r = preview_spec(&DesignSpec::default()).expect("preview");
        assert!(r.svg_2d.contains("<svg"));
        assert!(r.mesh.is_empty());
        assert!(r.modules >= 21);
        assert!(r.module_mm > 0.0);
        let r3 = preview_spec_ex(&DesignSpec::default(), true).expect("preview 3d");
        assert!(!r3.mesh.is_empty(), "expected 3d mesh parts");
        let verts: usize = r3.mesh.iter().map(|p| p.positions.len()).sum();
        assert!(verts > 200, "3d mesh too small: {verts}");
    }

    #[test]
    fn exports_3mf_and_stl() {
        let spec = DesignSpec::default();
        let qr = qr_gen::encode(&spec.payload).unwrap();
        let model = build_model(&spec, &qr);
        assert_eq!(model.parts[0].name, "Base");
        assert_eq!(model.parts[1].name, "QR");
        let (n3, b3) = export_3mf_base64(&model).unwrap();
        let (ns, bs) = export_stl_base64(&model).unwrap();
        assert!(n3.ends_with(".3mf"));
        assert!(ns.ends_with(".stl"));
        assert!(b3.len() > 80);
        assert!(bs.len() > 80);
    }

    #[test]
    fn wifi_special_chars_stay_in_one_field() {
        let p = crate::payload::wifi_payload("WPA", "Cafe;Guest", "a;b,c:d");
        assert_eq!(p, r#"WIFI:T:WPA;S:Cafe\;Guest;P:a\;b\,c\:d;;"#);
    }
}
