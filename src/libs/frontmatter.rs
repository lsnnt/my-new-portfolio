use std::collections::HashMap;

/// Split an optional `---`-delimited front-matter block from the body.
/// Returns the parsed key/value metadata (keys lower-cased) and the body.
pub fn split_front_matter(raw: &str) -> (HashMap<String, String>, String) {
    let raw = raw.strip_prefix('\u{feff}').unwrap_or(raw);
    let mut meta = HashMap::new();

    if let Some(rest) = raw.strip_prefix("---") {
        if let Some(end) = rest.find("\n---") {
            for line in rest[..end].lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Some((key, value)) = line.split_once(':') {
                    meta.insert(key.trim().to_lowercase(), strip_quotes(value.trim()).to_string());
                }
            }
            let body = rest[end + 4..].trim_start_matches('\n').to_string();
            return (meta, body);
        }
    }

    (meta, raw.to_string())
}

fn strip_quotes(s: &str) -> &str {
    let bytes = s.as_bytes();
    if bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''))
    {
        &s[1..s.len() - 1]
    } else {
        s
    }
}
