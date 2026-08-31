use resuma::prelude::*;

use crate::ads::{self, Placement};

#[layout("/")]
fn RootLayout() -> View {
    view! {
        <div class="app">
            <header class="site-header">
                <a class="brand" href="/">
                    <span class="brand-mark" aria-hidden="true"></span>
                    <span class="brand-name">"PlacaQR"</span>
                </a>
            </header>
            <Slot />
            <footer class="site-footer">
                {ads::unit(Placement::Footer)}
                <p>
                    <strong>"PlacaQR"</strong>
                    " — 3D-printable QR · no sign-up · we don’t keep your links"
                </p>
            </footer>
            <div id="toast-ad" class="toast-ad" popover="manual" role="status">
                {ads::unit(Placement::Toast)}
                <button type="button" class="toast-close" aria-label="Close ad">"×"</button>
            </div>
        </div>
    }
}
