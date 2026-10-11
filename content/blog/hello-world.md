---
title: Hello, World
date: 2026-01-15
description: First post on the new self-hosted blog — what this is and what to expect.
---

# Hello, World

Welcome to the blog. This page is served straight from the portfolio's own
Rust/Actix backend — no external platform, just markdown files rendered in the
same terminal aesthetic as the rest of the site.

## How posts work

Every post is a markdown file in `content/blog/`. The filename becomes the URL:

- `content/blog/hello-world.md` → `/blog/hello-world`

Each file can start with **front matter**:

```
---
title: My post
date: 2026-01-15
description: One-line summary shown on the index.
---
```

Then write the body in plain markdown — headings, `inline code`, fenced code
blocks, lists, links, tables, and blockquotes are all supported.

## Code

```rust
fn main() {
    println!("systems from the metal up");
}
```

## Lists

- Linux kernels
- BGP
- Backend services

> Document the journey.
