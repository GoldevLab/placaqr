use resuma::prelude::*;

use crate::landing::canonical_url;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Terms | PlacaQR");
    set_page_description("What PlacaQR generates and what you may encode.");
    set_page_canonical(canonical_url("/terms"));
    view! {
        <main class="content-section privacy-page">
            <p class="eyebrow">"Legal"</p>
            <h1>"Terms of use"</h1>
            <p class="hero-lead">
                "PlacaQR turns a payload into a 3D-printable QR. We are not a print farm or a link shortener."
            </p>
            <h2>"Your responsibility"</h2>
            <p>
                "Only encode links and Wi-Fi credentials you are allowed to publish. A printed Wi-Fi QR contains the password. Test-scan before you batch a restaurant."
            </p>
            <h2>"Availability"</h2>
            <p>
                "The tool is free. Ads may appear. We do not promise every phone will scan every size — watch the contrast hint."
            </p>
            <p>
                <NavLink href="/privacy">"Privacy"</NavLink>
            </p>
        </main>
    }
}
