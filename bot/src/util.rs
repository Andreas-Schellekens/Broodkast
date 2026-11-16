//! Kleine hulpfuncties voor tekst en veilige HTTP-requests naar door gebruikers opgegeven URL's.

use std::net::IpAddr;

use anyhow::{Context as _, bail};
use reqwest::Url;

/// Kort tekst in tot `max` tekens (niet bytes) en voegt `…` toe als er iets afgeknipt is.
pub fn truncate(text: &str, max: usize) -> String {
    let mut chars = text.chars();
    let cut: String = chars.by_ref().take(max).collect();
    if chars.next().is_some() {
        format!("{}…", cut.trim_end())
    } else {
        cut
    }
}

/// Haalt HTML-tags weg en zet de meest voorkomende entities om. Genoeg voor feed-samenvattingen.
pub fn strip_html(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => {
                in_tag = false;
                out.push(' ');
            }
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    let decoded = out
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&amp;", "&");
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Controleert dat een URL naar het publieke internet wijst, zodat gebruikers de bot geen
/// interne diensten (localhost, privénetwerk, Railway-intern) kunnen laten opvragen.
/// DNS-namen die naar privé-IP's resolven worden hier niet afgevangen.
pub fn check_public_url(raw: &str) -> anyhow::Result<Url> {
    let url = Url::parse(raw.trim()).context("dat is geen geldige URL")?;
    if !matches!(url.scheme(), "http" | "https") {
        bail!("alleen http- en https-URL's zijn toegestaan");
    }
    let host = url
        .host_str()
        .context("URL heeft geen host")?
        .to_ascii_lowercase();
    let host_trimmed = host.trim_start_matches('[').trim_end_matches(']');
    if host == "localhost"
        || host.ends_with(".localhost")
        || host.ends_with(".internal")
        || host.ends_with(".local")
    {
        bail!("die host is niet toegestaan");
    }
    if let Ok(ip) = host_trimmed.parse::<IpAddr>() {
        let private = match ip {
            IpAddr::V4(v4) => {
                v4.is_private()
                    || v4.is_loopback()
                    || v4.is_link_local()
                    || v4.is_unspecified()
                    || v4.is_broadcast()
                    || v4.octets()[0] == 100 && (v4.octets()[1] & 0xc0) == 64 // CGNAT
            }
            IpAddr::V6(v6) => {
                v6.is_loopback()
                    || v6.is_unspecified()
                    || (v6.segments()[0] & 0xfe00) == 0xfc00 // unique local
                    || (v6.segments()[0] & 0xffc0) == 0xfe80 // link local
            }
        };
        if private {
            bail!("interne IP-adressen zijn niet toegestaan");
        }
    }
    Ok(url)
}

/// Haalt een URL op met een maximale grootte, zodat een gigantisch bestand de bot niet opblaast.
pub async fn fetch_limited(
    http: &reqwest::Client,
    url: Url,
    max_bytes: usize,
) -> anyhow::Result<Vec<u8>> {
    let mut resp = http
        .get(url)
        .send()
        .await
        .context("kon de URL niet bereiken")?
        .error_for_status()
        .context("de server gaf een foutcode")?;
    if resp
        .content_length()
        .is_some_and(|len| len as usize > max_bytes)
    {
        bail!("bestand is te groot");
    }
    let mut body = Vec::new();
    while let Some(chunk) = resp.chunk().await? {
        body.extend_from_slice(&chunk);
        if body.len() > max_bytes {
            bail!("bestand is te groot");
        }
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_telt_tekens() {
        assert_eq!(truncate("héééé", 3), "héé…");
        assert_eq!(truncate("kort", 10), "kort");
    }

    #[test]
    fn strip_html_werkt() {
        assert_eq!(
            strip_html("<p>Hallo&nbsp;<b>wereld</b> &amp; co</p>"),
            "Hallo wereld & co"
        );
    }

    #[test]
    fn interne_urls_geweigerd() {
        for bad in [
            "http://localhost/feed",
            "http://127.0.0.1/x",
            "http://10.0.0.5/x",
            "http://192.168.1.1/x",
            "http://[::1]/x",
            "http://postgres.railway.internal/x",
            "ftp://example.com/x",
            "geen url",
        ] {
            assert!(
                check_public_url(bad).is_err(),
                "{bad} zou geweigerd moeten worden"
            );
        }
        assert!(check_public_url("https://example.com/feed.xml").is_ok());
    }
}
