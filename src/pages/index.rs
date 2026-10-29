use resuma::prelude::*;
use serde_json::json;

use crate::ads::{self, Placement};
use crate::landing::canonical_url;
use crate::tool::placaqr_tool;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("PlacaQR — 3D-printable QR | Stand, tile, keychain, plaque");
    set_page_description(
        "Make a dual-color 3MF QR: table stand, flush tile with magnet pocket, keychain, or wall plaque. No sign-up.",
    );
    set_page_canonical(canonical_url("/"));
    set_page_json_ld(home_json_ld());
    view! {
        <main>
            <section class="tool-section tool-section--lead" id="tool" aria-labelledby="tool-title">
                {crate::landing::hero_particles()}
                <div class="section-head section-head--compact">
                    <h1 id="tool-title">"Create your 3D QR"</h1>
                    <p>"Paste a link, pick a shape, download a file ready to print. No account needed."</p>
                </div>
                {placaqr_tool()}
                {ads::unit(Placement::Banner)}
            </section>
            <section class="content-section" aria-labelledby="guides-title">
                <h2 id="guides-title">"Guides"</h2>
                {crate::guides::home_strip()}
            </section>
            {crate::landing::sister_apps()}
        </main>
    }
}

fn home_json_ld() -> String {
    let origin = canonical_url("");
    json!({
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "Organization",
                "name": "PlacaQR",
                "url": origin
            },
            {
                "@type": "WebSite",
                "name": "PlacaQR",
                "url": origin
            },
            {
                "@type": "WebApplication",
                "name": "PlacaQR",
                "url": origin,
                "applicationCategory": "DesignApplication",
                "operatingSystem": "Web",
                "offers": {"@type": "Offer", "price": "0", "priceCurrency": "USD"},
                "featureList": [
                    "Dual-color 3MF QR",
                    "STL, PNG, and SVG export",
                    "Table stand, tile, keychain, and plaque",
                    "Google reviews QR",
                    "Guest Wi-Fi QR"
                ],
                "description": "Generator for 3D-printable QR codes. Filament 1 is the base, filament 2 is the code. No account."
            }
        ]
    })
    .to_string()
}
