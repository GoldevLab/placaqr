//! SEO landings, chrome helpers, and legal links.

use resuma::prelude::*;
use serde_json::json;

use crate::tool::placaqr_tool;

#[derive(Clone, Copy)]
pub enum Landing {
    Printable,
    Reviews,
    Wifi,
    Keychain,
}

impl Landing {
    pub fn path(self) -> &'static str {
        match self {
            Self::Printable => "/3d-printable-qr",
            Self::Reviews => "/google-reviews-qr",
            Self::Wifi => "/wifi-qr-print",
            Self::Keychain => "/qr-keychain-3mf",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Printable => "3D-printable QR code — 3MF and STL | PlacaQR",
            Self::Reviews => "Google reviews QR you can 3D print | PlacaQR",
            Self::Wifi => "Print a Wi-Fi QR stand or tile | PlacaQR",
            Self::Keychain => "QR keychain 3MF — dual-color print | PlacaQR",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Printable => {
                "Paste a link, pick a stand, tile, keychain, or plaque, download a dual-color 3MF. No account."
            }
            Self::Reviews => {
                "Turn a Google review link into a table stand or tile QR. Dual-color 3MF for Bambu, Prusa, Creality."
            }
            Self::Wifi => {
                "Print a Wi-Fi QR guests can scan. Stand or flush tile. No account."
            }
            Self::Keychain => {
                "Export a dual-color 3MF keychain QR. Hole is already in the mesh."
            }
        }
    }

    fn eyebrow(self) -> &'static str {
        match self {
            Self::Printable => "3D QR",
            Self::Reviews => "Reviews",
            Self::Wifi => "Wi-Fi",
            Self::Keychain => "Keychain",
        }
    }

    fn h1(self) -> &'static str {
        match self {
            Self::Printable => "Create a 3D-printable QR",
            Self::Reviews => "Print a Google reviews QR",
            Self::Wifi => "Print a Wi-Fi QR",
            Self::Keychain => "Print a QR keychain",
        }
    }

    fn lead(self) -> &'static str {
        match self {
            Self::Printable => {
                "Paste a URL, pick the object, download 3MF or STL. Filament 1 is the base, filament 2 is the QR."
            }
            Self::Reviews => {
                "Use the Reviews chip, paste the g.page link, print a stand for the table. Guests scan and leave a review."
            }
            Self::Wifi => {
                "Pick WiFi, enter the SSID and password. The payload is a WIFI: string phones already understand."
            }
            Self::Keychain => {
                "Switch the object to keychain. The ring hole is part of the mesh — no supports if you print flat."
            }
        }
    }

    fn preset(self) -> &'static str {
        match self {
            Self::Printable => "url",
            Self::Reviews => "google",
            Self::Wifi => "wifi",
            Self::Keychain => "url",
        }
    }

    fn howto(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Printable => &[
                ("Paste the link", "Web, Wi-Fi, reviews, Instagram, or plain text. We encode it, we do not store it."),
                ("Pick a shape", "Stand, flush tile (optional magnet), keychain, plaque, or coin."),
                ("Download 3MF", "Filament 1 = base, filament 2 = QR. 0.16–0.20 mm, no supports."),
            ],
            Self::Reviews => &[
                ("Copy the review URL", "From Google Maps or the review short link (g.page)."),
                ("Keep Reviews selected", "That chip is already on. Paste and preview the mesh."),
                ("Print a stand", "Table height is meant to sit by the check. Test-scan before a batch."),
            ],
            Self::Wifi => &[
                ("Pick WiFi", "SSID and password fields appear. WPA is the usual case."),
                ("Preview contrast", "Dark QR on a light base scans. Invert only if you print light-on-dark."),
                ("Leave it on the counter", "A stand or tile beats a paper card that walks away."),
            ],
            Self::Keychain => &[
                ("Any payload", "A menu URL or vCard both fit. Keep the module size large enough to scan."),
                ("Object = keychain", "The hole is already cut. Do not scale below the warning."),
                ("Dual color", "3MF maps two filaments. STL is one body if your slicer prefers that."),
            ],
        }
    }

    fn faq(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Printable => &[
                ("Do you keep my link?", "No. Generation is on this request. We do not store a gallery of payloads."),
                ("3MF or STL?", "3MF for AMS / dual color. STL if you assign colors in the slicer."),
                ("Will it scan?", "We warn when modules are too small or contrast is low. Test with a phone."),
                ("Is there an account?", "No. Ads may appear around the tool."),
            ],
            Self::Reviews => &[
                ("Does it open the review form?", "It opens whatever URL you paste. Use the official Google review link."),
                ("Table stand size?", "Default stand is meant for a restaurant table. Scale in the slicer if you need larger."),
                ("Do you store the place?", "No. The URL stays in the QR modules, not in our database."),
                ("Can I batch?", "This page is one QR at a time. Export, then print several copies of the same file."),
            ],
            Self::Wifi => &[
                ("Is the password in the QR?", "Yes. Anyone who scans joins the network. That is how WIFI: QR works."),
                ("Hidden SSID?", "Use the network name the phone should request. Hidden networks are hit-or-miss."),
                ("Guest network?", "Prefer a guest SSID so the printed plaque is not your admin password."),
                ("Do you log the password?", "No. It is encoded into the mesh for that download only."),
            ],
            Self::Keychain => &[
                ("How small can it be?", "If the warning says modules are small, scale up. Phones need a few millimeters per module."),
                ("Supports?", "Print the keychain flat. The hole does not need supports at 0.16–0.20 mm."),
                ("One color?", "Export STL and paint in the slicer, or print 3MF with the same filament twice."),
                ("Will a logo still scan?", "A small center logo is fine. A huge mark will fail — watch the hint."),
            ],
        }
    }
}

pub fn public_origin() -> String {
    std::env::var("SITE_URL")
        .ok()
        .map(|s| s.trim().trim_end_matches('/').to_string())
        .filter(|s| s.starts_with("http://") || s.starts_with("https://"))
        .unwrap_or_else(|| "https://placaqr.fly.dev".into())
}

pub fn canonical_url(path: &str) -> String {
    format!("{}{path}", public_origin())
}

pub fn seo_footer_links() -> View {
    view! {
        <nav class="seo-links" aria-label="PlacaQR pages">
            <NavLink href="/3d-printable-qr">"3D QR"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/google-reviews-qr">"Reviews"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/wifi-qr-print">"Wi-Fi"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/qr-keychain-3mf">"Keychain"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/privacy">"Privacy"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/terms">"Terms"</NavLink>
        </nav>
    }
}

/// Canonical family order. Each app skips itself in the footer and home cards.
const FAMILY: &[(&str, &str, &str)] = &[
    (
        "YouTubeForge",
        "YouTube transcript, MP3, SRT, and translation.",
        "https://youtubetotext.fly.dev",
    ),
    (
        "UnderKb",
        "Compress images to a real KB target. JPG, WebP, PNG.",
        "https://underkb.fly.dev",
    ),
    (
        "PDFForge",
        "Merge, split, compress PDFs. JPG ↔ PDF and extract text.",
        "https://pdfforge.fly.dev",
    ),
    (
        "PlacaQR",
        "3D-printable QR — stand, tile, keychain, or plaque.",
        "https://placaqr.fly.dev",
    ),
    (
        "Billloom",
        "Invoice, quote, and receipt PDFs. No account, no watermark.",
        "https://billloom.fly.dev",
    ),
];

const SELF: &str = "PlacaQR";

pub fn sister_apps() -> View {
    let cards = FAMILY
        .iter()
        .copied()
        .filter(|(name, _, _)| *name != SELF)
        .map(|(name, blurb, href)| {
            let name = name.to_string();
            let blurb = blurb.to_string();
            let href = href.to_string();
            view! {
                <li>
                    <a href={href} class="sister-card" rel="noopener">
                        <strong>{name}</strong>
                        <span>{blurb}</span>
                    </a>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <nav class="sister-apps" aria-label="Other apps from us">
            <p class="eyebrow">"Also from us"</p>
            <h2>"Free tools, same idea"</h2>
            <p class="hint">
                "No account. Paste, convert, download. Transcripts, images under a KB cap, PDFs, and invoices."
            </p>
            <ul class="sister-grid">{cards}</ul>
        </nav>
    }
}

pub fn sister_apps_links() -> View {
    let items = FAMILY
        .iter()
        .copied()
        .filter(|(name, _, _)| *name != SELF)
        .enumerate()
        .map(|(i, (name, _, href))| {
            let name = name.to_string();
            let href = href.to_string();
            if i == 0 {
                view! { <a href={href} rel="noopener">{name}</a> }
            } else {
                view! {
                    <span aria-hidden="true">" · "</span>
                    <a href={href} rel="noopener">{name}</a>
                }
            }
        })
        .collect::<Vec<_>>();

    view! {
        <nav class="sister-apps-links" aria-label="Also from us">
            <span>"Also from us:"</span>
            " "
            {items}
        </nav>
    }
}

pub fn theme_picker() -> View {
    view! {
        <div class="theme-wrap">
            <Popup id="themes" positions="bottom left top right" class="theme-popover">
                <button
                    slot="anchor"
                    type="button"
                    class="theme-btn"
                    aria-haspopup="dialog"
                    aria-label="Change site theme"
                    title="Themes"
                >
                    <span class="theme-btn-wheel" aria-hidden="true"></span>
                    <span class="theme-btn-label">"Theme"</span>
                </button>
                <div class="theme-popover-body" role="dialog" aria-label="Choose a theme">
                    <p class="theme-popover-kicker">"Live themes"</p>
                    <p class="theme-popover-lead">
                        "Filament is PlacaQR’s resin palette. The rest are official Resuma palettes."
                    </p>
                    <div class="theme-grid" role="group" aria-label="Site palettes">
                        <ThemeSwitch id="filament">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#12081c 0 120deg, #8b5cf6 120deg 240deg, #c4b5fd 240deg 360deg)"></span>
                                <span>"Filament"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="paper">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#eceff4 0 120deg, #2563eb 120deg 240deg, #0f172a 240deg 360deg)"></span>
                                <span>"Paper"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="slate">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#f4efe6 0 120deg, #c2410c 120deg 240deg, #1c1917 240deg 360deg)"></span>
                                <span>"Slate"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="midnight">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#0b1020 0 120deg, #818cf8 120deg 240deg, #e6e8ee 240deg 360deg)"></span>
                                <span>"Midnight"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="ember">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#1a100c 0 120deg, #f59e0b 120deg 240deg, #e8a87c 240deg 360deg)"></span>
                                <span>"Ember"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="aurora">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#0a1628 0 120deg, #22d3ee 120deg 240deg, #a78bfa 240deg 360deg)"></span>
                                <span>"Aurora"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="forest">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#0c1410 0 120deg, #34d399 120deg 240deg, #6ee7b7 240deg 360deg)"></span>
                                <span>"Forest"</span>
                            </button>
                        </ThemeSwitch>
                    </div>
                </div>
            </Popup>
        </div>
    }
}

pub fn chrome(body: View) -> View {
    view! {
        <div class="app">
            <div class="liquid-orbs" aria-hidden="true">
                <div class="liquid-blob liquid-blob-a"></div>
                <div class="liquid-blob liquid-blob-b"></div>
                <div class="liquid-blob liquid-blob-c"></div>
            </div>
            <header class="site-header">
                <div class="header-inner">
                    <NavLink href="/" class="brand" activeClass="is-active" exact=true>
                        <span class="brand-mark" aria-hidden="true"></span>
                        <span class="brand-name">"PlacaQR"</span>
                    </NavLink>
                    {theme_picker()}
                </div>
            </header>
            {body}
            <footer class="site-footer">
                {seo_footer_links()}
                {sister_apps_links()}
                <p>
                    <strong>"PlacaQR"</strong>
                    " — 3D-printable QR · no sign-up · we don’t keep your links"
                </p>
            </footer>
            <div id="toast-ad" class="toast-ad" popover="manual" role="status">
                {crate::ads::unit(crate::ads::Placement::Toast)}
                <button type="button" class="toast-close" aria-label="Close ad">"×"</button>
            </div>
        </div>
    }
}

pub fn seo_landing(kind: Landing) -> View {
    set_page_title(kind.title());
    set_page_description(kind.description());
    set_page_canonical(canonical_url(kind.path()));
    let faq_ld = json!({
        "@context": "https://schema.org",
        "@type": "FAQPage",
        "mainEntity": kind.faq().iter().map(|(q, a)| json!({
            "@type": "Question",
            "name": q,
            "acceptedAnswer": { "@type": "Answer", "text": a }
        })).collect::<Vec<_>>()
    });
    set_page_json_ld(faq_ld.to_string());
    let howto: Vec<View> = kind
        .howto()
        .iter()
        .map(|(h, p)| {
            view! {
                <li>
                    <h3>{*h}</h3>
                    <p>{*p}</p>
                </li>
            }
        })
        .collect();
    let faq: Vec<View> = kind
        .faq()
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
    let preset = kind.preset();
    view! {
        <main class="home-page landing-page" lang="en">
            <section class="hero tool-section tool-section--lead">
                <p class="eyebrow">{kind.eyebrow()}</p>
                <h1>{kind.h1()}</h1>
                <p class="hero-lead">{kind.lead()}</p>
                <div data-start-preset={preset}>
                    {placaqr_tool()}
                </div>
            </section>
            <section class="howto content-section" aria-labelledby="howto-title">
                <h2 id="howto-title">"How to use it"</h2>
                <ol class="howto-grid">{howto}</ol>
            </section>
            {crate::ads::unit(crate::ads::Placement::Banner)}
            <section class="faq content-section" aria-labelledby="faq-title">
                <h2 id="faq-title">"FAQ"</h2>
                <div class="faq-list">{faq}</div>
            </section>
        </main>
    }
}
