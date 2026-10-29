use resuma::prelude::*;
use resuma::FlowPageRegistry;

pub struct PagesRegistry;

impl FlowPageRegistry for PagesRegistry {
    fn routes(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("/", "index"),
            ("/3d-printable-qr", "printable_qr"),
            ("/google-reviews-qr", "reviews_qr"),
            ("/wifi-qr-print", "wifi_qr"),
            ("/qr-keychain-3mf", "keychain_qr"),
            ("/privacy", "privacy"),
            ("/terms", "terms"),
            ("/extension", "extension"),
            ("/admin", "admin"),
            ("/guides", "guides"),
            ("/guides/3d-printable-qr", "guide_print"),
            ("/guides/google-reviews-qr", "guide_reviews"),
            ("/guides/wifi-qr", "guide_wifi"),
            ("/guides/qr-keychain", "guide_keychain"),
        ]
    }

    fn layout_for(&self, _pattern: &str) -> &'static [&'static str] {
        &["/"]
    }

    fn render(&self, module: &str, req: FlowRequest) -> Option<View> {
        match module {
            "index" => Some(super::index::page(req)),
            "printable_qr" => Some(super::printable_qr::page(req)),
            "reviews_qr" => Some(super::reviews_qr::page(req)),
            "wifi_qr" => Some(super::wifi_qr::page(req)),
            "keychain_qr" => Some(super::keychain_qr::page(req)),
            "privacy" => Some(super::privacy::page(req)),
            "terms" => Some(super::terms::page(req)),
            "extension" => Some(super::extension::page(req)),
            "admin" => Some(super::admin::page(req)),
            "guides" => Some(super::guides::page(req)),
            "guide_print" => Some(super::guide_print::page(req)),
            "guide_reviews" => Some(super::guide_reviews::page(req)),
            "guide_wifi" => Some(super::guide_wifi::page(req)),
            "guide_keychain" => Some(super::guide_keychain::page(req)),
            _ => None,
        }
    }
}
