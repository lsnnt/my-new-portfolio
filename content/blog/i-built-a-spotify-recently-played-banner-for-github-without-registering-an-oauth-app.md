---
title: "I built a Spotify recently-played banner for GitHub — without registering an OAuth app"
date: 2026-05-29
description: "I run most of my own infrastructure, so depending on someone else's hosted service for a README widget felt wrong. And most \"Spotify for GitHub README\" proje..."
---

I run most of my own infrastructure, so depending on someone else's hosted service for a README widget felt wrong. And most "Spotify for GitHub README" projects share the same setup story: go to the Spotify developer dashboard, register an app, grab a client ID and secret, plug them into some hosted service, authorize it, and hope the maintainer keeps the server running.

I wanted something self-hosted, with no developer app registration at all. So I dug into how the Spotify web player authenticates — and it turns out there's a cleaner path.

## The trick: `sp_dc` + PKCE

When you log into [open.spotify.com](https://open.spotify.com), your browser gets an `sp_dc` session cookie. The web player uses this cookie to silently drive the full [PKCE (Proof Key for Code Exchange)](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow) authorization flow and obtain a short-lived bearer token — without any client secret.

The key endpoint is:

```plaintext
GET https://accounts.spotify.com/oauth2/v2/auth
  ?response_type=code
  &client_id=<spotify_web_player_client_id>
  &scope=user-read-recently-played ...
  &redirect_uri=https://developer.spotify.com
  &code_challenge=<sha256_of_verifier>
  &code_challenge_method=S256
  &response_mode=web_message
  &prompt=none
Cookie: sp_dc=<your_cookie>
```

With `prompt=none` and a valid `sp_dc`, Spotify returns an authorization code directly in the response body — no browser redirect, no user interaction. You then exchange that code (plus the PKCE verifier) for a bearer token, and you're in.

The whole auth chain in one line:

```plaintext
sp_dc cookie → PKCE flow → bearer token → /v1/me/player/recently-played → SVG
```

## The implementation

The server is written in Go. A few things worth pointing out:

**Token caching with mutex safety**

Hitting the auth endpoint on every request would be slow and rate-limitable. The token is cached globally and protected with a `sync.Mutex`:

```go
var (
    cachedToken string
    tokenMu     sync.Mutex
)

func getCachedToken() (string, error) {
    tokenMu.Lock()
    defer tokenMu.Unlock()
    if cachedToken == "" {
        token, err := getToken()
        if err != nil {
            return "", fmt.Errorf("getting token: %w", err)
        }
        cachedToken = token
    }
    return cachedToken, nil
}
```

**Auto-refresh on non-200**

Bearer tokens expire. Rather than tracking expiry times, the server just invalidates the cache whenever the Spotify API returns a non-200 and retries once:

```go
for attempt := 0; attempt < 2; attempt++ {
    token, err := getCachedToken()
    // ... make the API call ...
    if res.StatusCode != http.StatusOK {
        res.Body.Close()
        invalidateToken()
        continue
    }
    // decode and return
}
```

Simple and works well in practice.

**Bypassing GitHub's Camo proxy cache**

GitHub proxies all images through [Camo](https://github.blog/2014-01-28-proxying-user-images/), its image CDN, which aggressively caches responses. Without the right headers, your banner would show stale data for hours. The fix is straightforward:

```go
w.Header().Add("Cache-Control", "max-age=0, no-cache, no-store, must-revalidate")
```

This tells Camo not to cache the response, so every README load fetches a fresh SVG.

## What it looks like

The current SVG design is intentionally minimal — a dark Spotify-green card listing your 20 most recently played tracks with numbered rows.

![banner preview](https://spotify.fustin.top/?temp=foo)

There's room to make it much better: album art, artist names, play counts, theme variants. Contributions welcome.

## Running it yourself

```bash
git clone https://github.com/lsnnt/spotify-banner-for-github
cd spotify-banner-for-github
```

Get your `sp_dc` cookie from DevTools → Application → Cookies on open.spotify.com, then:

```bash
echo 'SPDC="your_cookie_here"' > .env
go build . && ./spotify-banner-for-github
```

Visit `http://localhost:8080/` — you should see your recently played tracks rendered as an SVG.

To embed it in your GitHub README, deploy it to any publicly reachable server and add:

```markdown
![Spotify recently played](https://your-server.example.com/)
```

## A note on `sp_dc`

The `sp_dc` cookie is a long-lived session credential. Treat it like a password — don't commit your `.env`, and rotate it by logging out and back into the web player. This approach is unofficial and intended for personal, non-commercial use.

* * *

The full source is on GitHub: [**lsnnt/spotify-banner-for-github**](https://github.com/lsnnt/spotify-banner-for-github)

If you like it do star the Repo.

If you want to improve the SVG design or add album art support, open a PR — that's the part that needs the most work right now.
