//! Crawlable guides that answer a print search and end on the generator.

use resuma::prelude::*;
use serde_json::json;

use crate::landing::canonical_url;
use crate::tool::placaqr_tool;

struct Guide {
    path: &'static str,
    title: &'static str,
    description: &'static str,
    h1: &'static str,
    lead: &'static str,
    preset: &'static str,
    sections: &'static [(&'static str, &'static str)],
    faq: &'static [(&'static str, &'static str)],
    links: &'static [(&'static str, &'static str)],
}

const ALL: &[Guide] = &[PRINT, REVIEWS, WIFI, KEYCHAIN];

const PRINT: Guide = Guide {
    path: "/guides/3d-printable-qr",
    title: "3D-Printable QR Code | PlacaQR",
    description: "Make a dual-color 3MF QR for a stand, tile, keychain, or plaque. Filament 1 is the base, filament 2 is the code. No account.",
    h1: "3D-print a QR code",
    lead: "Paste the link, pick a shape, and download a 3MF. In the slicer, filament 1 is the base and filament 2 is the QR. Print at 0.16–0.20 mm with a 0.4 mm nozzle and no supports.",
    preset: "url",
    sections: &[
        (
            "Two filaments, not a sticker",
            "The file is a raised code in a second color. A single-color STL is there if you want to paint it yourself, but the 3MF is the one that scans without paint.",
        ),
        (
            "Module size",
            "If the preview says the modules are small, scale the object up. A phone needs a few millimeters per module.",
        ),
        (
            "What we keep",
            "The link is encoded into the file you download. We do not keep a gallery of your URLs.",
        ),
    ],
    faq: &[
        (
            "Which printers?",
            "Any slicer that opens 3MF. The file is aimed at Bambu, Prusa, and Creality style dual-color prints.",
        ),
        (
            "Do I need supports?",
            "No, at 0.16–0.20 mm, printed in the orientation the file uses.",
        ),
    ],
    links: &[
        ("/3d-printable-qr", "Open the 3D QR page"),
        ("/guides/google-reviews-qr", "A reviews stand"),
        ("/guides/wifi-qr", "A Wi-Fi plaque"),
    ],
};

const REVIEWS: Guide = Guide {
    path: "/guides/google-reviews-qr",
    title: "Google Reviews QR You Can 3D-Print | PlacaQR",
    description: "Turn a Google review link into a table stand or plaque. Dual-color 3MF. No account.",
    h1: "Print a Google reviews QR",
    lead: "Use the review link, not the maps homepage. A stand on the table should open the write-a-review screen. Pick the stand shape and download the 3MF.",
    preset: "google",
    sections: &[
        (
            "The link has to be the review URL",
            "A short maps link that only opens the listing makes people hunt for the stars. Paste the link that starts a review.",
        ),
        (
            "Stand, not a flat sticker",
            "The table stand sits on the counter. A flush tile is for a surface you can stick or screw.",
        ),
        (
            "Scan test before a batch",
            "Print one. Scan it with a phone in the room light you actually have. Then print the rest.",
        ),
    ],
    faq: &[
        (
            "Does this post the review for them?",
            "No. The code only opens the link.",
        ),
        (
            "Can I add a logo?",
            "A small center mark can still scan. A huge one will not. Watch the hint on the tool.",
        ),
    ],
    links: &[
        ("/google-reviews-qr", "Open reviews QR"),
        ("/guides/3d-printable-qr", "Print settings"),
        ("/guides/qr-keychain", "A smaller object"),
    ],
};

const WIFI: Guide = Guide {
    path: "/guides/wifi-qr",
    title: "3D-Print a Wi-Fi QR | PlacaQR",
    description: "Encode a guest Wi-Fi network into a printable QR. The password is in the file, not stored on the site. No account.",
    h1: "Print a Wi-Fi QR",
    lead: "Use a guest network. The code contains the SSID and password, so anyone who scans it joins. Prefer a guest SSID over the one that runs your house.",
    preset: "wifi",
    sections: &[
        (
            "Guest network",
            "A plaque on the wall is a published password. Put the guest SSID on it.",
        ),
        (
            "What gets encoded",
            "The standard Wi-Fi QR string: security type, SSID, and password. We encode it for the file. We do not keep it.",
        ),
        (
            "Where to put it",
            "A wall plaque or a flush tile. A keychain is a bad place for a network password.",
        ),
    ],
    faq: &[
        (
            "Is the password stored?",
            "No. It leaves with the file you download.",
        ),
        (
            "Hidden SSID?",
            "Phones are unreliable with hidden networks. Use a visible guest name.",
        ),
    ],
    links: &[
        ("/wifi-qr-print", "Open Wi-Fi QR"),
        ("/guides/3d-printable-qr", "How the 3MF is printed"),
        ("/guides/google-reviews-qr", "A reviews stand instead"),
    ],
};

const KEYCHAIN: Guide = Guide {
    path: "/guides/qr-keychain",
    title: "3D-Print a QR Keychain | PlacaQR",
    description: "A small dual-color QR keychain as 3MF. Scale up if the modules are too small to scan. No account.",
    h1: "Print a QR keychain",
    lead: "A keychain is small, so the code has to stay scannable. If the tool warns that modules are tiny, make the object larger before you export.",
    preset: "url",
    sections: &[
        (
            "Size before filament",
            "Phones need a few millimeters per module. A cute tiny code that does not scan is waste.",
        ),
        (
            "Print flat",
            "The hole does not need supports at 0.16–0.20 mm. Filament 1 is the body, filament 2 is the code.",
        ),
        (
            "What to encode",
            "A link, not a Wi-Fi password. A keychain gets lost.",
        ),
    ],
    faq: &[
        (
            "One color only?",
            "Export STL and paint it, or print the 3MF in one filament twice. Contrast is what makes it scan.",
        ),
        (
            "Will a logo still scan?",
            "A small center logo can. A large one covers the code.",
        ),
    ],
    links: &[
        ("/qr-keychain-3mf", "Open the keychain page"),
        ("/guides/3d-printable-qr", "Print settings"),
        ("/guides/wifi-qr", "Do not put Wi-Fi on a keychain"),
    ],
};

pub fn render(path: &str) -> View {
    let guide = ALL.iter().find(|g| g.path == path).unwrap_or(&PRINT);
    set_page_title(guide.title);
    set_page_description(guide.description);
    set_page_canonical(canonical_url(guide.path));
    set_page_json_ld(guide_json_ld(guide));
    let sections: Vec<View> = guide
        .sections
        .iter()
        .map(|(h, p)| {
            view! {
                <section class="guide-section">
                    <h2>{*h}</h2>
                    <p>{*p}</p>
                </section>
            }
        })
        .collect();
    let faq: Vec<View> = guide
        .faq
        .iter()
        .map(|(q, a)| {
            view! {
                <details>
                    <summary>{*q}</summary>
                    <p>{*a}</p>
                </details>
            }
        })
        .collect();
    let related: Vec<View> = guide
        .links
        .iter()
        .map(|(href, label)| {
            let href = (*href).to_string();
            view! {
                <li>
                    <NavLink href={href}>{*label}</NavLink>
                </li>
            }
        })
        .collect();
    let preset = guide.preset;
    view! {
        <main class="home-page guide-page" lang="en">
            <section class="hero tool-section tool-section--lead">
                <p class="eyebrow">"Guide"</p>
                <h1>{guide.h1}</h1>
                <p class="hero-lead">{guide.lead}</p>
                <div data-start-preset={preset}>
                    {placaqr_tool()}
                </div>
            </section>
            <article class="content-section guide-article">
                {sections}
                <h2>"Related"</h2>
                <ul class="guide-links">{related}</ul>
                <p class="guide-actions">
                    <NavLink href="/guides" class="btn btn-ghost">"All guides"</NavLink>
                </p>
            </article>
            <section class="faq" aria-labelledby="guide-faq">
                <h2 id="guide-faq">"FAQ"</h2>
                <div class="faq-list">{faq}</div>
            </section>
        </main>
    }
}

pub fn index_page() -> View {
    set_page_title("3D QR guides — print, reviews, Wi-Fi, keychain | PlacaQR");
    set_page_description(
        "How to 3D-print a QR, a Google reviews stand, a guest Wi-Fi plaque, and a keychain. Each guide includes the generator.",
    );
    set_page_canonical(canonical_url("/guides"));
    set_page_json_ld(index_json_ld());
    view! {
        <main class="content-section guide-page">
            <p class="eyebrow">"Learn"</p>
            <h1>"PlacaQR guides"</h1>
            <p class="hero-lead">"Short pages for the objects people search for. Each one includes the generator."</p>
            {home_strip()}
        </main>
    }
}

pub fn home_strip() -> View {
    let cards: Vec<View> = ALL
        .iter()
        .map(|g| {
            let href = g.path.to_string();
            view! {
                <li>
                    <a href={href} class="related-card" data-r-nav="true">
                        <strong>{g.h1}</strong>
                        <span>{g.description}</span>
                    </a>
                </li>
            }
        })
        .collect();
    view! { <ul class="related-grid">{cards}</ul> }
}

fn index_json_ld() -> String {
    let origin = crate::landing::public_origin();
    let items: Vec<_> = ALL
        .iter()
        .enumerate()
        .map(|(i, g)| {
            json!({
                "@type": "ListItem",
                "position": i + 1,
                "name": g.h1,
                "url": format!("{origin}{}", g.path)
            })
        })
        .collect();
    json!({
        "@context": "https://schema.org",
        "@type": "ItemList",
        "name": "PlacaQR guides",
        "itemListElement": items
    })
    .to_string()
}

fn guide_json_ld(g: &Guide) -> String {
    let origin = crate::landing::public_origin();
    let faq: Vec<_> = g
        .faq
        .iter()
        .map(|(q, a)| {
            json!({
                "@type": "Question",
                "name": q,
                "acceptedAnswer": {"@type": "Answer", "text": a}
            })
        })
        .collect();
    json!({
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "Article",
                "headline": g.h1,
                "description": g.description,
                "mainEntityOfPage": format!("{origin}{}", g.path),
                "author": {"@type": "Organization", "name": "PlacaQR"},
                "publisher": {"@type": "Organization", "name": "PlacaQR", "url": origin}
            },
            {"@type": "FAQPage", "mainEntity": faq}
        ]
    })
    .to_string()
}
