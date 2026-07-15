pub fn normalize_url(input: &str) -> String {
    let mut url = input.trim().to_string();

    if !url.contains("://") {
        url = format!("https://{url}");
    }

    let scheme_end = url.find("://").unwrap() + 3;
    let after_scheme = &url[scheme_end..];
    let host_end = after_scheme
        .find(['/', '?', '#'])
        .unwrap_or(after_scheme.len());
    let host = &after_scheme[..host_end];

    if !host.starts_with("www.") && host.matches('.').count() == 1 {
        let scheme = &url[..scheme_end];
        let rest = &after_scheme[host_end..];
        url = format!("{scheme}www.{host}{rest}");
    }

    url
}

pub fn resolve_href(base: &str, href: &str) -> Option<String> {
    if href.starts_with('#') || href.starts_with("javascript:") || href.starts_with("mailto:") {
        return None;
    }

    let base = url::Url::parse(base).ok()?;
    let resolved = base.join(href).ok()?;

    Some(resolved.to_string())
}
