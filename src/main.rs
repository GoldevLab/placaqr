//! PlacaQR — 3D-printable QR generator in pure Rust (Resuma Flow).

mod actions;
mod ads;
mod landing;
mod design;
mod export3d;
mod logo;
mod mesh;
mod pages;
mod payload;
mod preview;
mod qr_gen;
mod site;
mod tool;

use pages::PagesRegistry;
use resuma::prelude::*;
use resuma::SeoKit;
use serde_json::json;

fn placa_not_found() -> View {
    crate::landing::chrome_with_ads(view! {
        <main class="content-section">
            <h1>"Page not found"</h1>
            <p class="hero-lead">"That path does not exist on PlacaQR."</p>
            <p>
                <NavLink href="/" class="btn btn-primary">"Back to home"</NavLink>
            </p>
        </main>
    }, false)
}

const HEAD: &str = r##"
<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
<link rel="icon" href="/icon.svg" type="image/svg+xml" />
<link rel="icon" href="/icons/favicon-32.png" type="image/png" sizes="32x32" />
<link rel="apple-touch-icon" href="/icons/apple-touch-icon.png" sizes="180x180" />
<link rel="preconnect" href="https://fonts.googleapis.com" />
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
<link href="https://fonts.googleapis.com/css2?family=Figtree:ital,wght@0,400;0,500;0,600;0,700;0,800;1,400&family=Outfit:wght@600;700;800&display=swap" rel="stylesheet" />
<script type="module" src="/js/placaqr-ui.js?v=2"></script>
<script type="module" src="/js/placaqr-fx.js?v=2"></script>
"##;

fn seo_kit() -> SeoKit {
    let origin = crate::landing::public_origin();
    let mut kit = SeoKit::new("PlacaQR", &origin)
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
            "url": crate::landing::public_origin(),
            "applicationCategory": "DesignApplication",
            "operatingSystem": "Web",
            "offers": {"@type": "Offer", "price": "0", "priceCurrency": "EUR"},
            "description": "Generator for 3D-printable QR codes (3MF and STL)"
        }));
    kit.theme_color = Some("#12081c".into());
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
        (
            "Pages".into(),
            "/3d-printable-qr /google-reviews-qr /wifi-qr-print /qr-keychain-3mf. /privacy /terms.".into(),
        ),
    ];
    kit
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // `with_seo_kit` owns keywords/author/theme-color meta, JSON-LD, and the
    // `/robots.txt` + `/llms.txt` routes (AI crawler policy included).
    let head = format!("{HEAD}{}{}", ads::head_snippet(), crate::site::head_extras());
    let ads_txt = ads::ads_txt().map(|s| -> &'static [u8] {
        Box::leak(s.into_bytes().into_boxed_slice())
    });
    const ICON: &[u8] = include_bytes!("icon.svg");
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
        .with_site_url(crate::landing::public_origin())
        .with_og_image("/og.png")
        .with_head(head)
        .with_seo_kit(seo_kit())
        .with_html_theme(
            HtmlTheme::new(["filament"])
                .dark(["filament"])
                .cookie("placaqr_theme")
                .storage_key("placaqr-theme"),
        )
        .with_stylesheet("/css/placaqr.css?v=r1")
        .static_asset("/icon.svg", ICON, "image/svg+xml");
    if let Some(body) = ads_txt {
        app = app.static_asset("/ads.txt", body, "text/plain; charset=utf-8");
    }
    app.with_public_dir(public)
        .with_pwa(FlowPwaConfig {
            name: "PlacaQR".into(),
            short_name: "PlacaQR".into(),
            description: "Make a dual-color 3MF or STL QR for stands, tiles, keychains, and plaques."
                .into(),
            theme_color: "#8b5cf6".into(),
            background_color: "#12081c".into(),
            start_url: "/".into(),
            scope: "/".into(),
            cache_version: "pqr-8".into(),
            display: "standalone".into(),
            orientation: "any".into(),
            lang: "en".into(),
            icon_char: Some("Q".into()),
            precache_paths: vec![
                "/themes.css".into(),
                "/css/placaqr.css?v=r1".into(),
                "/js/placaqr-ui.js?v=2".into(),
                "/js/placaqr-fx.js?v=2".into(),
                "/js/placaqr-ads.js?v=2".into(),
                "/icon.svg".into(),
                "/icons/icon-192.png".into(),
                "/icons/icon-512.png".into(),
                "/icons/apple-touch-icon.png".into(),
            ],
            shortcuts: vec![PwaShortcut {
                name: "New QR".into(),
                short_name: "Home".into(),
                url: "/".into(),
            }],
            offline_title: "You're offline".into(),
            offline_message:
                "PlacaQR needs a connection to build and download your QR. Reconnect and try again."
                    .into(),
            manifest_icons: Vec::new(),
        })
        .not_found(placa_not_found)
        .auto_pages(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages"),
            PagesRegistry,
        )
        .serve(serve)
        .await
}
