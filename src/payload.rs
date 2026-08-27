//! WIFI: / vCard payload helpers (spec-compatible escaping).

/// Escape a WIFI QR field (ZXing / Android convention).
/// `\ ; , " :` must be prefixed with a backslash.
/// Keep in sync with `escWifi` in `tool.rs`.
#[allow(dead_code)]
pub fn escape_wifi_field(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if matches!(c, '\\' | ';' | ',' | '"' | ':') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// `WIFI:T:<auth>;S:<ssid>;P:<password>;;` — omit `P` on open networks.
#[allow(dead_code)]
pub fn wifi_payload(auth: &str, ssid: &str, password: &str) -> String {
    let s = escape_wifi_field(ssid);
    if auth.eq_ignore_ascii_case("nopass") {
        format!("WIFI:T:nopass;S:{s};;")
    } else {
        let p = escape_wifi_field(password);
        format!("WIFI:T:{auth};S:{s};P:{p};;")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_specials_in_password() {
        let q = wifi_payload("WPA", "Cafe;Guest", r#"p@ss;word,:"#);
        assert_eq!(q, r#"WIFI:T:WPA;S:Cafe\;Guest;P:p@ss\;word\,\:;;"#);
    }

    #[test]
    fn open_network_omits_password() {
        assert_eq!(wifi_payload("nopass", "Free", "ignored"), "WIFI:T:nopass;S:Free;;");
    }
}
