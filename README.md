# PlacaQR

3D-printable QR code generator — **100% Rust** with [Resuma](https://resuma-docs.fly.dev/).

## What it does

- Customize a QR (URL, WiFi, Google reviews, Instagram, text)
- Objects: table stand, keychain, wall plaque
- Export **dual-color 3MF**, STL, PNG, and SVG
- 2D preview and interactive 3D preview (mesh from the server, Three.js in the browser)

## Development

```bash
cd qr3d
cargo run
# or: resuma dev
```

Open http://127.0.0.1:3000

Flow already serves `/robots.txt`, `/sitemap.xml`, `/favicon.svg`, and `/og.svg`. Do not put those files in `public/` — Axum panics if the route exists twice.

## Deploy (Fly.io)

Pushes to `main` deploy automatically via GitHub Actions (`.github/workflows/fly.yml`).

```bash
# first time only, if the Fly app does not exist yet:
fly apps create placaqr
fly tokens create deploy -x 999999h -a placaqr
# paste the token as the GitHub Actions secret FLY_API_TOKEN
```

## Architecture

| Module | Role |
|--------|------|
| `qr_gen` | QR matrix (`qrcode`) |
| `mesh` | 3D object geometry |
| `export3d` | Binary STL + colored 3MF |
| `preview` | 2D SVG + PNG |
| `actions` | `#[server]` preview + export |
| `tool` | Resuma `#[island]` UI |
