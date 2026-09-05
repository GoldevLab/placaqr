use resuma::prelude::*;

#[layout("/")]
fn RootLayout() -> View {
    crate::landing::chrome(view! { <Slot /> })
}
