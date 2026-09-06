use resuma::prelude::*;

use crate::landing::{canonical_url, chrome_store_url};

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Chrome extension | PlacaQR");
    set_page_description(
        "Turn the current page into a 3D-printable QR. Opens PlacaQR with the URL filled in.",
    );
    set_page_canonical(canonical_url("/extension"));
    let store_block = match chrome_store_url() {
        Some(url) => view! {
            <p>
                <a class="btn btn-primary" href={url} rel="noopener">"Add from Chrome Web Store"</a>
            </p>
        },
        None => view! {
            <p class="hint">
                "The Web Store listing is not public yet. In chrome://extensions turn on Developer mode and Load unpacked from the GitHub folder extension/."
            </p>
            <p>
                <a class="btn btn-primary" href="https://github.com/GoldevLab/placaqr/tree/main/extension" rel="noopener">
                    "extension/ on GitHub"
                </a>
            </p>
        },
    };
    view! {
        <main class="content-section privacy-page">
            <p class="eyebrow">"Browser"</p>
            <h1>"PlacaQR extension"</h1>
            <p class="hero-lead">
                "QR this page opens PlacaQR with the tab URL already in the link field. Download 3MF or STL there. Ads stay on the website, which Google AdSense allows."
            </p>
            {store_block}
            <h2>"What it does"</h2>
            <ol class="howto-grid">
                <li>
                    <h3>"Toolbar"</h3>
                    <p>"QR this page — encodes https://… of the active tab."</p>
                </li>
                <li>
                    <h3>"Right-click"</h3>
                    <p>"3D QR of this page, or of a link."</p>
                </li>
            </ol>
            <p>
                <NavLink href="/" class="btn btn-ghost">"Use the website"</NavLink>
            </p>
        </main>
    }
}
