//! Google AdSense slots. Layout is always reserved; live units render when
//! `ADSENSE_CLIENT` + per-placement slot IDs are set.

use resuma::prelude::*;
use resuma::server::{CspConfig, GOOGLE_ADSENSE_ORIGINS};

const CLIENT_ENV: &str = "ADSENSE_CLIENT";

#[derive(Clone, Copy)]
pub enum Placement {
    Banner,
    #[allow(dead_code)]
    Footer,
    Download,
    Toast,
}

impl Placement {
    fn slot_env(self) -> &'static str {
        match self {
            Self::Banner => "ADSENSE_SLOT_BANNER",
            Self::Footer => "ADSENSE_SLOT_FOOTER",
            Self::Download => "ADSENSE_SLOT_DOWNLOAD",
            Self::Toast => "ADSENSE_SLOT_TOAST",
        }
    }

    fn classes(self, live: bool) -> &'static str {
        match (self, live) {
            (Self::Banner, true) => "ad-unit ad-unit--banner is-live",
            (Self::Banner, false) => "ad-unit ad-unit--banner is-placeholder",
            (Self::Footer, true) => "ad-unit ad-unit--footer is-live",
            (Self::Footer, false) => "ad-unit ad-unit--footer is-placeholder",
            (Self::Download, true) => "ad-unit ad-unit--download is-live",
            (Self::Download, false) => "ad-unit ad-unit--download is-placeholder",
            (Self::Toast, true) => "ad-unit ad-unit--toast is-live",
            (Self::Toast, false) => "ad-unit ad-unit--toast is-placeholder",
        }
    }

    fn hint(self) -> &'static str {
        match self {
            Self::Banner => "728 × 90 · responsive",
            Self::Footer => "Footer · responsive",
            Self::Download => "300 × 250",
            Self::Toast => "320 × 100",
        }
    }

    fn lazy(self) -> bool {
        matches!(self, Self::Download | Self::Toast)
    }
}

pub fn client_id() -> Option<String> {
    let raw = std::env::var(CLIENT_ENV).ok()?;
    sanitize_client(&raw)
}

fn sanitize_client(raw: &str) -> Option<String> {
    let s = raw.trim();
    let digits = s.strip_prefix("ca-pub-")?;
    if digits.len() >= 10 && digits.bytes().all(|b| b.is_ascii_digit()) {
        Some(s.to_string())
    } else {
        None
    }
}

fn sanitize_slot(raw: &str) -> Option<String> {
    let s = raw.trim();
    if !s.is_empty() && s.len() <= 22 && s.bytes().all(|b| b.is_ascii_digit()) {
        Some(s.to_string())
    } else {
        None
    }
}

fn slot_id(kind: Placement) -> Option<String> {
    std::env::var(kind.slot_env())
        .ok()
        .as_deref()
        .and_then(sanitize_slot)
}

/// `<script>` tag for the document head (empty when no publisher ID).
pub fn head_snippet() -> String {
    match client_id() {
        Some(id) => format!(
            r#"<script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client={id}" crossorigin="anonymous"></script>
<script type="module" src="/js/placaqr-ads.js"></script>"#
        ),
        None => r#"<script type="module" src="/js/placaqr-ads.js"></script>"#.into(),
    }
}

/// `/ads.txt` body, or `None` when AdSense is not configured.
pub fn ads_txt() -> Option<String> {
    let client = client_id()?;
    let pub_id = client.strip_prefix("ca-")?;
    Some(format!(
        "google.com, {pub_id}, DIRECT, f08c47fec0942fa0\n"
    ))
}

pub fn apply_csp(csp: &mut CspConfig) {
    if client_id().is_none() {
        return;
    }
    // `frame-src` is required for AdSense iframes; without it `default-src 'self'`
    // blocks the units. Keep CSP enforcing (do not ship `RESUMA_CSP=0`).
    for origin in GOOGLE_ADSENSE_ORIGINS {
        push_unique(&mut csp.script_src, origin);
        push_unique(&mut csp.img_src, origin);
        push_unique(&mut csp.connect_src, origin);
        push_unique(&mut csp.frame_src, origin);
    }
}

fn push_unique(list: &mut Vec<String>, origin: &str) {
    if !list.iter().any(|s| s == origin) {
        list.push(origin.to_string());
    }
}

/// Reserved slot. Renders a live `<ins class="adsbygoogle">` when both
/// publisher and unit IDs are set; otherwise a sized placeholder.
pub fn unit(kind: Placement) -> View {
    let live = client_id().zip(slot_id(kind));
    let class = kind.classes(live.is_some());
    let hint = kind.hint();
    match live {
        Some((client, slot)) if kind.lazy() => view! {
            <aside class={class} aria-label="Advertisement" data-ad-lazy="">
                <ins
                    class="adsbygoogle"
                    data-ad-client={client}
                    data-ad-slot={slot}
                    data-ad-format="auto"
                    data-full-width-responsive="true"
                ></ins>
            </aside>
        },
        Some((client, slot)) => view! {
            <aside class={class} aria-label="Advertisement">
                <ins
                    class="adsbygoogle"
                    data-ad-client={client}
                    data-ad-slot={slot}
                    data-ad-format="auto"
                    data-full-width-responsive="true"
                ></ins>
            </aside>
        },
        None => view! {
            <aside class={class} aria-label="Advertisement">
                <span class="ad-unit__label">"Ad"</span>
                <small class="ad-unit__hint">{hint}</small>
            </aside>
        },
    }
}
