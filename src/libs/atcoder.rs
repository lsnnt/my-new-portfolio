use reqwest::Client;
use regex::Regex;

/// Fetch the AtCoder rating and highest rating by scraping the public profile
/// page (https://atcoder.jp/users/lsnnt). Mirrors the other `libs::*` fetchers.
pub async fn atcoder(client: &Client) -> Result<(u16, u16), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let body = client
        .get("https://atcoder.jp/users/lsnnt")
        .header("User-Agent", "Mozilla/5.0 (compatible; lsnnt-portfolio/0.1)")
        .send()
        .await?
        .text()
        .await?;

    Ok((stat(&body, "Rating")?, stat(&body, "Highest Rating")?))
}

/// Pull a numeric stat out of the profile's `<th>Label</th><td>…<span>N</span>…</td>` row.
fn stat(html: &str, label: &str) -> Result<u16, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // `(?s)` = DOTALL: the profile markup has newlines inside the <td>, so the
    // cell capture must be allowed to span lines.
    let row = Regex::new(&format!(
        r"(?s)<th[^>]*>\s*{}\s*</th>\s*<td[^>]*>(.*?)</td>",
        regex::escape(label)
    ))?;
    let cell = row
        .captures(html)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .ok_or_else(|| format!("atcoder: '{label}' row not found"))?;

    let value = Regex::new(r"<span[^>]*>\s*(\d+)\s*</span>")?;
    let digits = value
        .captures(cell)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .ok_or_else(|| format!("atcoder: no value in '{label}'"))?;

    Ok(digits.parse::<u16>()?)
}
