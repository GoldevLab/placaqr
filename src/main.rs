//! PlacaQR — 3D-printable QR generator in pure Rust (Resuma Flow).

mod actions;
mod ads;
mod design;
mod export3d;
mod logo;
mod mesh;
mod pages;
mod payload;
mod preview;
mod qr_gen;
mod tool;

use pages::PagesRegistry;
use resuma::prelude::*;
use resuma::SeoKit;
use serde_json::json;

fn placa_not_found() -> View {
    view! {
        <main class="content-section">
            <h1>"Page not found"</h1>
            <p class="hero-lead">"That path does not exist on PlacaQR."</p>
            <p>
                <a class="btn btn-primary" href="/">"Back to home"</a>
            </p>
        </main>
    }
}

const HEAD: &str = r##"
<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link href="https://fonts.googleapis.com/css2?family=Figtree:ital,wght@0,400;0,500;0,600;0,700;0,800;1,400&family=Outfit:wght@600;700;800&display=swap" rel="stylesheet" />
<link rel="canonical" href="https://placaqr.fly.dev/" />
<meta property="og:title" content="PlacaQR — 3D-printable QR codes" />
<meta property="og:description" content="Make a dual-color 3MF or STL QR object. No sign-up." />
<meta property="og:type" content="website" />
<script type="module" src="/js/placaqr-ui.js"></script>
"##;

fn seo_kit() -> SeoKit {
    let mut kit = SeoKit::new("PlacaQR", "https://placaqr.fly.dev")
        .with_locale("en_US")
        .with_keywords(
            "3D printable QR code, QR keychain 3D, QR 3MF, 3D QR plaque, \
             restaurant table stand QR, Google reviews QR 3D, QR STL generator, \
             WiFi QR 3D print, Bambu Lab QR, AMS dual color QR",
        )
        .with_llms_summary(
            "PlacaQR generates 3D-printable QR codes. Paste a link (web, WiFi, \
             Google reviews, Instagram) and download a dual-color 3MF or STL ready for \
             Bambu Lab, Prusa, or Creality. No sign-up.",
        )
        .with_default_json_ld()
        .push_json_ld(json!({
            "@context": "https://schema.org",
            "@type": "WebApplication",
            "name": "PlacaQR",
            "applicationCategory": "DesignApplication",
            "operatingSystem": "Web",
            "offers": {"@type": "Offer", "price": "0", "priceCurrency": "EUR"},
            "description": "Generator for 3D-printable QR codes (3MF and STL)"
        }));
    kit.theme_color = Some("#f4f3f8".into());
    kit.author = "PlacaQR".into();
    kit.llms_sections = vec![
        (
            "Tool".into(),
            "Preview in 2D/3D and export dual-color 3MF, STL, PNG, and SVG.".into(),
        ),
        (
            "Objects".into(),
            "Table stand, flush tile (optional magnet), keychain, wall plaque, and coin.".into(),
        ),
    ];
    kit
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let kit = seo_kit();
    let head = format!("{HEAD}{}{}", kit.head_extras(), ads::head_snippet());
    let json_ld = serde_json::to_string(&kit.json_ld_blocks).unwrap_or_else(|_| "[]".into());
    let llms: &'static [u8] = Box::leak(kit.llms_txt().into_bytes().into_boxed_slice());
    let ads_txt = ads::ads_txt().map(|s| -> &'static [u8] {
        Box::leak(s.into_bytes().into_boxed_slice())
    });
    let public = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("public");

    let mut serve = FlowServeOptions::default();
    // Island JS runs via eval; Chrome then blocks `import()` under 'strict-dynamic'.
    // Same-origin module scripts still need `'self'` to load the Three.js viewer.
    serve.security.csp.strict_dynamic = false;
    ads::apply_csp(&mut serve.security.csp);

    let mut app = FlowApp::new()
        .with_title("PlacaQR — 3D-printable QR | Stand, tile, keychain, plaque")
        .with_description(
            "Make a dual-color 3MF QR: table stand, flush tile with magnet pocket, keychain, or wall plaque. \
             No sign-up. Built for Google reviews, Wi-Fi, and menus.",
        )
        .with_site_url("https://placaqr.fly.dev")
        .with_og_image("/og.svg")
        .with_json_ld(json_ld)
        .with_head(head)
        .with_stylesheet("/css/placaqr.css")
        .static_asset("/llms.txt", llms, "text/plain; charset=utf-8");
    if let Some(body) = ads_txt {
        app = app.static_asset("/ads.txt", body, "text/plain; charset=utf-8");
    }
    app.with_public_dir(public)
        .without_pwa()
        .not_found(placa_not_found)
        .auto_pages(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages"),
            PagesRegistry,
        )
        .serve(serve)
        .await
}
