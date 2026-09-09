//! Google AdSense slots. Layout is always reserved; live units render when
//! `ADSENSE_CLIENT` + per-placement slot IDs are set.

use resuma::prelude::*;
use resuma::server::CspConfig;

const CLIENT_ENV: &str = "ADSENSE_CLIENT";

const ADSENSE_ORIGINS: &[&str] = &[
    "https://pagead2.googlesyndication.com",
    "https://googleads.g.doubleclick.net",
    "https://tpc.googlesyndication.com",
    "https://www.google.com",
    "https://www.gstatic.com",
    "https://www.googleadservices.com",
    "https://adservice.google.com",
    "https://www.googletagservices.com",
    "https://partner.googleadservices.com",
    "https://ep1.adtrafficquality.google",
    "https://ep2.adtrafficquality.google",
    "https://fundingchoicesmessages.google.com",
];

#[derive(Clone, Copy)]
pub enum Placement {
    Banner,
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
        .or_else(|| {
            std::env::var("ADSENSE_SLOT")
                .ok()
                .as_deref()
                .and_then(sanitize_slot)
        })
}

/// `<script>` tag for the document head (empty when no publisher ID).
pub fn head_snippet() -> String {
    // Tiny deferred loader — AdSense after engagement / long idle (monitor TBT).
    match client_id() {
        Some(id) => format!(
            r#"<link rel="dns-prefetch" href="https://pagead2.googlesyndication.com" />
<link rel="dns-prefetch" href="https://googleads.g.doubleclick.net" />
<meta name="pqr-adsense-client" content="{id}" />
<script>
(function(){{
  function load(){{
    if(window.__pqrAdsJs)return;
    window.__pqrAdsJs=1;
    var s=document.createElement('script');
    s.type='module';
    s.src='/js/placaqr-ads.js?v=3';
    s.fetchPriority='low';
    document.head.appendChild(s);
  }}
  function arm(){{
    var start=function(){{load();}};
    ['pointerdown','keydown','touchstart','scroll'].forEach(function(e){{
      window.addEventListener(e,start,{{once:true,passive:true}});
    }});
    if('requestIdleCallback' in window)requestIdleCallback(start,{{timeout:12000}});
    else window.addEventListener('load',function(){{setTimeout(start,8000);}},{{once:true}});
  }}
  if(document.readyState==='loading')document.addEventListener('DOMContentLoaded',arm,{{once:true}});
  else arm();
}})();
</script>"#
        ),
        None => String::new(),
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
    csp.strict_dynamic = false;
    csp.report_only = true;
    for origin in ADSENSE_ORIGINS {
        push_unique(&mut csp.script_src, origin);
        push_unique(&mut csp.img_src, origin);
        push_unique(&mut csp.connect_src, origin);
        push_unique(&mut csp.style_src, origin);
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
