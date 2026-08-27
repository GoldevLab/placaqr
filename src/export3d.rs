//! Binary STL + dual-color 3MF exporters (pure Rust).

use std::io::{Cursor, Write};

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

use crate::mesh::{Model3d, Tri, V3};

pub fn export_stl_base64(model: &Model3d) -> Result<(String, String), String> {
    let bytes = write_stl_binary(model)?;
    Ok(("placaqr.stl".into(), B64.encode(bytes)))
}

pub fn export_3mf_base64(model: &Model3d) -> Result<(String, String), String> {
    let bytes = write_3mf(model)?;
    Ok(("placaqr.3mf".into(), B64.encode(bytes)))
}

fn write_stl_binary(model: &Model3d) -> Result<Vec<u8>, String> {
    let mut tris: Vec<Tri> = Vec::new();
    for p in &model.parts {
        tris.extend(p.tris.iter().copied());
    }
    let mut out = Vec::with_capacity(84 + tris.len() * 50);
    out.extend_from_slice(&[0u8; 80]); // header
    out.extend_from_slice(&(tris.len() as u32).to_le_bytes());
    for t in tris {
        let n = normal(t.a, t.b, t.c);
        write_f32(&mut out, n.x);
        write_f32(&mut out, n.y);
        write_f32(&mut out, n.z);
        for v in [t.a, t.b, t.c] {
            write_f32(&mut out, v.x);
            write_f32(&mut out, v.y);
            write_f32(&mut out, v.z);
        }
        out.extend_from_slice(&0u16.to_le_bytes());
    }
    Ok(out)
}

fn write_f32(out: &mut Vec<u8>, v: f32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn normal(a: V3, b: V3, c: V3) -> V3 {
    let u = V3::new(b.x - a.x, b.y - a.y, b.z - a.z);
    let v = V3::new(c.x - a.x, c.y - a.y, c.z - a.z);
    let n = V3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    );
    let len = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt().max(1e-8);
    V3::new(n.x / len, n.y / len, n.z / len)
}

fn write_3mf(model: &Model3d) -> Result<Vec<u8>, String> {
    let buf = Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(buf);
    let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    zip.start_file("[Content_Types].xml", opts)
        .map_err(|e| e.to_string())?;
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/>
</Types>"#,
    )
    .map_err(|e| e.to_string())?;

    zip.start_file("_rels/.rels", opts)
        .map_err(|e| e.to_string())?;
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Target="/3D/3dmodel.model" Id="rel0" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/>
</Relationships>"#,
    )
    .map_err(|e| e.to_string())?;

    let model_xml = build_3mf_model_xml(model)?;
    zip.start_file("3D/3dmodel.model", opts)
        .map_err(|e| e.to_string())?;
    zip.write_all(model_xml.as_bytes())
        .map_err(|e| e.to_string())?;

    let cursor = zip.finish().map_err(|e| e.to_string())?;
    Ok(cursor.into_inner())
}

fn build_3mf_model_xml(model: &Model3d) -> Result<String, String> {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<model unit="millimeter" xml:lang="en-US" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02">
  <resources>
"#,
    );

    // basematerials
    xml.push_str(r#"    <basematerials id="1">"#);
    xml.push('\n');
    for (i, part) in model.parts.iter().enumerate() {
        let (r, g, b) = crate::design::parse_rgb(&part.color);
        xml.push_str(&format!(
            r##"      <base name="{name}" displaycolor="#{r:02X}{g:02X}{b:02X}FF"/>"##,
            name = xml_escape(&part.name),
            r = r,
            g = g,
            b = b
        ));
        xml.push('\n');
        let _ = i;
    }
    xml.push_str("    </basematerials>\n");

    let mut object_ids = Vec::new();
    for (i, part) in model.parts.iter().enumerate() {
        let oid = (i + 2) as u32;
        object_ids.push(oid);
        xml.push_str(&format!(
            r#"    <object id="{oid}" name="{name}" type="model" pid="1" pindex="{pi}">
      <mesh>
        <vertices>
"#,
            oid = oid,
            name = xml_escape(&part.name),
            pi = i
        ));

        // Dedup vertices lightly
        let mut verts: Vec<V3> = Vec::new();
        let mut indices: Vec<(usize, usize, usize)> = Vec::new();
        for t in &part.tris {
            let ia = push_v(&mut verts, t.a);
            let ib = push_v(&mut verts, t.b);
            let ic = push_v(&mut verts, t.c);
            indices.push((ia, ib, ic));
        }
        for v in &verts {
            xml.push_str(&format!(
                r#"          <vertex x="{:.4}" y="{:.4}" z="{:.4}"/>"#,
                v.x, v.y, v.z
            ));
            xml.push('\n');
        }
        xml.push_str("        </vertices>\n        <triangles>\n");
        for (a, b, c) in indices {
            xml.push_str(&format!(
                r#"          <triangle v1="{a}" v2="{b}" v3="{c}"/>"#
            ));
            xml.push('\n');
        }
        xml.push_str("        </triangles>\n      </mesh>\n    </object>\n");
    }

    xml.push_str("  </resources>\n  <build>\n");
    for oid in object_ids {
        xml.push_str(&format!(r#"    <item objectid="{oid}"/>"#));
        xml.push('\n');
    }
    xml.push_str("  </build>\n</model>\n");
    Ok(xml)
}

fn push_v(verts: &mut Vec<V3>, v: V3) -> usize {
    // Exact match only (fast enough for our sizes)
    for (i, e) in verts.iter().enumerate() {
        if (e.x - v.x).abs() < 1e-4 && (e.y - v.y).abs() < 1e-4 && (e.z - v.z).abs() < 1e-4 {
            return i;
        }
    }
    verts.push(v);
    verts.len() - 1
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
