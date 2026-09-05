use resuma::prelude::*;

use crate::landing::canonical_url;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Privacy | PlacaQR");
    set_page_description(
        "How PlacaQR handles the links you paste, generated meshes, and ads.",
    );
    set_page_canonical(canonical_url("/privacy"));
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
                "Reserved ad slots may show Google AdSense when a publisher id is configured. See "
                <a href="https://policies.google.com/technologies/ads" rel="noopener">"Google’s advertising policies"</a>
                "."
            </p>
            <h2>"Contact"</h2>
            <p>
                "Questions: "
                <a href="https://github.com/GoldevLab/placaqr/issues" rel="noopener">"open an issue on GitHub"</a>
                " (set CONTACT_EMAIL on the server to show a mailbox). "
                <NavLink href="/terms">"Terms"</NavLink>
                "."
            </p>
        </main>
    }
}
