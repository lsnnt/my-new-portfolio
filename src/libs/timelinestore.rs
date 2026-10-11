use std::fs;
use std::path::Path;

use crate::libs::frontmatter::split_front_matter;
use crate::models::timeline::TimelineEntry;

const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];

struct Raw {
    sort: String,
    display: String,
    head: String,
    title: String,
    body: String,
    metric: String,
}

/// Load every `*.md` file in `dir` as a "commit history" (timeline) entry.
///
/// Each file is a markdown file with optional front matter:
///
/// ```text
/// ---
/// date: 2026-01            # sortable; "YYYY-MM" renders as "January 2026"
/// title: AtCoder Journey Begins
/// metric: atcoder          # optional live value: codeforces | leetcode | atcoder
/// head: HEAD~2             # optional; defaults to position (newest = HEAD)
/// ---
/// Free-text description of what you did.
/// ```
///
/// Adding an entry = drop in a file. Removing one = delete the file.
/// Entries are returned oldest-first.
pub fn load_timeline(dir: &Path) -> Vec<TimelineEntry> {
    let mut raws: Vec<Raw> = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("timeline: cannot read {}: {e}", dir.display());
            return Vec::new();
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(stem) => stem.to_string(),
            None => continue,
        };
        let raw = match fs::read_to_string(&path) {
            Ok(raw) => raw,
            Err(e) => {
                eprintln!("timeline: cannot read {}: {e}", path.display());
                continue;
            }
        };

        let (meta, body) = split_front_matter(&raw);
        let date = meta.get("date").cloned().unwrap_or_default();
        let title = meta.get("title").cloned().unwrap_or_else(|| stem.clone());
        let metric = meta.get("metric").cloned().unwrap_or_default();
        let head = meta.get("head").cloned().unwrap_or_default();
        let body = body.split_whitespace().collect::<Vec<_>>().join(" ");

        raws.push(Raw {
            sort: format!("{date}\u{0}{stem}"),
            display: display_date(&date),
            head,
            title,
            body,
            metric,
        });
    }

    raws.sort_by(|a, b| a.sort.cmp(&b.sort));

    let n = raws.len();
    raws.into_iter()
        .enumerate()
        .map(|(i, r)| TimelineEntry {
            date: r.display,
            head: if !r.head.is_empty() {
                r.head
            } else if i + 1 == n {
                "HEAD".to_string()
            } else {
                format!("HEAD~{}", n - 1 - i)
            },
            title: r.title,
            body: r.body,
            metric: r.metric,
        })
        .collect()
}

/// `2026-01` -> `January 2026`; anything else is passed through unchanged.
fn display_date(date: &str) -> String {
    if let Some((year, month)) = date.split_once('-') {
        if let Ok(m) = month.trim_start_matches('0').parse::<usize>() {
            if (1..=12).contains(&m) {
                return format!("{} {}", MONTHS[m - 1], year.trim());
            }
        }
    }
    date.to_string()
}
