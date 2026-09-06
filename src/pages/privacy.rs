use resuma::prelude::*;

use crate::landing::canonical_url;
use crate::site;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Privacy | PlacaQR");
    set_page_description(
        "How PlacaQR handles the links you paste, generated meshes, and ads.",
    );
    set_page_canonical(canonical_url("/privacy"));
    let contact = match site::contact_email() {
        Some(email) => {
            let href = format!("mailto:{email}");
            view! {
                <p>
                    "Questions: "
                    <a href={href}>{email}</a>
                    " or "
                    <a href="https://github.com/GoldevLab/placaqr/issues" rel="noopener">"GitHub"</a>
                    ". "
                    <NavLink href="/terms">"Terms"</NavLink>
                    "."
                </p>
            }
        }
        None => view! {
            <p>
                "Questions: "
                <a href="https://github.com/GoldevLab/placaqr/issues" rel="noopener">"open an issue on GitHub"</a>
                " (set CONTACT_EMAIL on the server to show a mailbox). "
                <NavLink href="/terms">"Terms"</NavLink>
                "."
            </p>
        },
    };
    view! {
        <main class="content-section privacy-page">
            <p class="eyebrow">"Legal"</p>
            <h1>"Privacy"</h1>
            <p class="hero-lead">
                "PlacaQR is a no-account tool. We do not keep a gallery of the URLs you encode."
            </p>
            <h2>"What we process"</h2>
            <p>
                "The payload you type is encoded into a QR mesh for that request so you can download 3MF, STL, PNG, or SVG. We do not store the link after the response."
            </p>
            <h2>"Advertising"</h2>
            <p>
                "Reserved ad slots may show Google AdSense when a publisher id is configured. Google and its partners may set cookies or use similar identifiers to show ads and measure them. See "
                <a href="https://policies.google.com/technologies/ads" rel="noopener">"Google’s advertising policies"</a>
                " and "
                <a href="https://adssettings.google.com/" rel="noopener">"Ad Settings"</a>
                ". Publisher ads.txt is served at "
                <a href="/ads.txt">"/ads.txt"</a>
                " when a publisher id is configured."
            </p>
            <h2>"Browser extension"</h2>
            <p>
                "The optional Chrome extension only opens this website with the current page URL filled in. It does not show Google AdSense inside the popup. Ads stay on these pages."
            </p>
            <h2>"Contact"</h2>
            {contact}
        </main>
    }
}
