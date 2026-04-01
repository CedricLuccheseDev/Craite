# CrAIte

**Stop digging through 40 folders for your samples.**

CrAIte scans your sample folders, classifies sounds by type (kick, snare, pad, vocal...), and creates an organized mirror library via hardlinks — directly accessible in your DAW's file browser.

- **Free and open source** (AGPL v3)
- **100% local** — no cloud, no account, no data leaves your PC
- **Works with any DAW** — FL Studio, Ableton, Logic, Bitwig, Cubase, REAPER, and more
- **Zero extra disk space** — uses hardlinks, your originals stay untouched
- **Windows, macOS, Linux**

## How it works

1. **Install CrAIte** — auto-detects your sample folders
2. **Run the scan** — classifies samples by filename + audio analysis (WAV, MP3, FLAC, OGG, AIFF)
3. **Open your DAW** — add the CrAIte Library folder to your search paths. Done.

## Download

**[Download CrAIte](https://craite.clhub.fr)** — Free, no account required.

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop | Tauri v2 |
| Frontend | Vue 3 + TypeScript + Vite + Pinia |
| Backend | Rust |
| Database | SQLite (rusqlite) |
| Audio | rodio + symphonia |
| Classifier | AhoCorasick pattern matching + audio feature analysis |
| Landing | Nuxt 3 + @nuxt/ui |

---

## Development

### Prerequisites

- **Node.js >= 18** — `nvm install 18`
- **pnpm >= 8** — `npm install -g pnpm`
- **Rust >= 1.70** — [rustup.rs](https://rustup.rs)

**Linux only:**
```bash
sudo apt-get install -y pkg-config build-essential libwebkit2gtk-4.1-dev \
  libssl-dev libayatana-appindicator3-dev librsvg2-dev libasound2-dev
```

**macOS only:** `xcode-select --install`

**Windows only:** [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with C++ workload + [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### Setup

```bash
git clone git@github.com:CedricLuccheseDev/Craite.git
cd Craite
cp .env.example .env  # add your PostHog key if needed
pnpm install
```

### Commands

```bash
pnpm dev            # Tauri dev mode (frontend + Rust backend)
pnpm dev:landing    # Nuxt dev server (landing page)
pnpm build          # Build desktop distributable
pnpm build:landing  # Build landing for production
pnpm lint           # ESLint
pnpm format         # Prettier
pnpm test           # Vitest
pnpm typecheck      # Type-check desktop + landing
```

### Project Structure

```
apps/
  desktop/              # Tauri v2 desktop app
    src/                # Vue 3 / TypeScript frontend
    src-tauri/src/      # Rust backend
      commands/         # Tauri IPC commands
      scanner/          # Filesystem scanning
      classifier/       # Filename + audio classification
      linker/           # Hardlink / copy strategies
      watcher/          # File system watcher
      audio/            # Audio preview (rodio)
      db/               # SQLite database

  landing/              # Nuxt 3 landing page
```

### Release

1. Bump version in `apps/desktop/src-tauri/tauri.conf.json` and `Cargo.toml`
2. Commit and merge to `main` via PR
3. Push tag: `git tag v0.x.x && git push origin v0.x.x`
4. CI builds for all platforms, uploads to S3, auto-update is live

## License

[AGPL-3.0-only](LICENSE)
