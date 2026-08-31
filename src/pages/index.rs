use resuma::prelude::*;

use crate::ads::{self, Placement};
use crate::tool::placaqr_tool;

pub fn page(_req: FlowRequest) -> View {
    view! {
        <main>
            <section class="tool-section tool-section--lead" id="tool" aria-labelledby="tool-title">
                <div class="section-head section-head--compact">
                    <h1 id="tool-title">"Create your 3D QR"</h1>
                    <p>"Paste a link, pick a shape, download a file ready to print. No account needed."</p>
                </div>
                {placaqr_tool()}
                {ads::unit(Placement::Banner)}
            </section>
        </main>

        <dialog id="ad-download-dialog" closedby="any">
            <div class="dialog-ad">
                <form method="dialog">
                    <button type="submit" class="dialog-close" aria-label="Close">"×"</button>
                </form>
                {ads::unit(Placement::Download)}
                <p class="dialog-note">
                    "Your file is downloading. Open the 3MF: filament 1 = Base, filament 2 = QR. Print at 0.16–0.20 mm, no supports."
                </p>
            </div>
        </dialog>
    }
}
