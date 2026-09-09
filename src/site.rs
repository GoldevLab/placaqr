//! Optional env: CONTACT_EMAIL, GSC_VERIFICATION, GA4_ID, PLAUSIBLE_DOMAIN.

pub fn contact_email() -> Option<String> {
    std::env::var("CONTACT_EMAIL")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| s.contains('@') && !s.contains(' '))
}

pub fn head_extras() -> String {
    let mut out = String::new();
    if let Ok(v) = std::env::var("GSC_VERIFICATION") {
        let v = v.trim();
        if !v.is_empty()
            && v.len() < 120
            && v.bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            out.push_str(&format!(
                r#"<meta name="google-site-verification" content="{v}" />"#
            ));
        }
    }
    if let Ok(id) = std::env::var("GA4_ID") {
        let id = id.trim();
        if id.starts_with("G-")
            && id.len() < 20
            && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
                        out.push_str(&format!(
                r#"<link rel="dns-prefetch" href="https://www.googletagmanager.com" />
<script>
(function(){{
  var id={id:?};
  function load(){{
    if(window.__goldevGaLoaded)return;
    window.__goldevGaLoaded=1;
    var s=document.createElement('script');
    s.async=true;
    s.fetchPriority='low';
    s.src='https://www.googletagmanager.com/gtag/js?id='+id;
    s.onload=function(){{
      window.dataLayer=window.dataLayer||[];
      function gtag(){{dataLayer.push(arguments);}}
      window.gtag=gtag;
      gtag('js',new Date());
      gtag('config',id,{{send_page_view:true}});
    }};
    document.head.appendChild(s);
  }}
  function arm(){{
    var start=function(){{load();}};
    ['pointerdown','keydown','touchstart','scroll'].forEach(function(e){{
      window.addEventListener(e,start,{{once:true,passive:true}});
    }});
    if('requestIdleCallback' in window)requestIdleCallback(start,{{timeout:12000}});
    else window.addEventListener('load',function(){{setTimeout(start,8000);}},{{once:true}});
  }}
  if(document.readyState==='loading')document.addEventListener('DOMContentLoaded',arm,{{once:true}});
  else arm();
}})();
</script>"#
            ));
        }
    }
    if let Ok(domain) = std::env::var("PLAUSIBLE_DOMAIN") {
        let domain = domain.trim();
        if !domain.is_empty() && domain.len() < 80 && !domain.contains('<') {
            out.push_str(&format!(
                r#"<script defer data-domain="{domain}" src="https://plausible.io/js/script.js"></script>"#
            ));
        }
    }
    out
}
