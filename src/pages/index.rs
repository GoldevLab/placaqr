use resuma::prelude::*;

use crate::ads::{self, Placement};
use crate::landing::canonical_url;
use crate::tool::placaqr_tool;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("PlacaQR — 3D-printable QR | Stand, tile, keychain, plaque");
    set_page_description(
        "Make a dual-color 3MF QR: table stand, flush tile with magnet pocket, keychain, or wall plaque. No sign-up.",
    );
    set_page_canonical(canonical_url("/"));
    view! {
        <main>
            <section class="tool-section tool-section--lead" id="tool" aria-labelledby="tool-title">
                <div class="hero-wrap">
                    {crate::landing::hero_particles()}
                    <div class="section-head section-head--compact">
                        <h1 id="tool-title">"Create your 3D QR"</h1>
                        <p>"Paste a link, pick a shape, download a file ready to print. No account needed."</p>
                    </div>
                </div>
                {placaqr_tool()}
                {ads::unit(Placement::Banner)}
            </section>
            {crate::landing::sister_apps()}
        </main>

        // Opened from the island via `__resuma.showModal("ad-download")` after export.
        <Modal id="ad-download" closedBy="any" class="ad-download-dialog">
            <div class="dialog-ad">
                <form method="dialog">
                    <button type="submit" class="dialog-close" aria-label="Close">"×"</button>
                </form>
                {ads::unit(Placement::Download)}
                <p class="dialog-note">
                    "Your file is downloading. Open the 3MF: filament 1 = Base, filament 2 = QR. Print at 0.16–0.20 mm, no supports."
                </p>
            </div>
        </Modal>
    }
}
