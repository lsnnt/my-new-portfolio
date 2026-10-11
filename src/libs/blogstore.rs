use std::collections::HashMap;
use std::fs;
use std::path::Path;

use pulldown_cmark::{html, Options, Parser};

use crate::models::blogpost::BlogPost;

/// Load every `*.md` file in `dir` as a blog post.
///
/// Each file may optionally start with a front-matter block:
///
/// ```text
/// ---
/// title: My first post
/// date: 2026-01-15
/// description: A short summary shown on the index.
/// ---
/// # Markdown body
/// ```
///
/// Posts are returned newest-first (sorted by `date`, then `slug`).
/// A missing/unreadable directory or file is logged and skipped rather
/// than panicking, so a bad post never takes the server down.
pub fn load_posts(dir: &Path) -> Vec<BlogPost> {
    let mut posts = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("blog: cannot read {}: {e}", dir.display());
            return posts;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let slug = match path.file_stem().and_then(|s| s.to_str()) {
            Some(slug) => slug.to_string(),
            None => continue,
        };
        let raw = match fs::read_to_string(&path) {
            Ok(raw) => raw,
            Err(e) => {
                eprintln!("blog: cannot read {}: {e}", path.display());
                continue;
            }
        };

        let (meta, body) = split_front_matter(&raw);
        let title = meta
            .get("title")
            .cloned()
            .unwrap_or_else(|| slug.clone());
        let date = meta.get("date").cloned().unwrap_or_default();
        let description = meta.get("description").cloned().unwrap_or_default();

        posts.push(BlogPost {
            slug,
            title,
            date,
            description,
            html: render_markdown(&body),
        });
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.slug.cmp(&b.slug)));
    posts
}

/// Split an optional `---`-delimited front-matter block from the body.
/// Returns the parsed key/value metadata (keys lower-cased) and the body.
fn split_front_matter(raw: &str) -> (HashMap<String, String>, String) {
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

fn render_markdown(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(md, options);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}
