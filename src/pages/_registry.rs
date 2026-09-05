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
            _ => None,
        }
    }
}
