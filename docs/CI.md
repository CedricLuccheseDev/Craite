# CI/CD Pipeline — CrAIte

## Overview

The CI/CD pipeline is split into **3 GitHub Actions workflows**:

| Workflow | Trigger | Purpose |
|---|---|---|
| **CI** | Pull request on `main` | Validate code quality (lint, types, build, Rust checks) |
| **Release Please** | Push on `main` | Automate versioning, changelog, and trigger builds |
| **Build Desktop** | Called by Release Please (or manual) | Build Tauri app for Windows, macOS, Linux |

There is also a **Dockerfile** for the landing page deployment.

---

## 1. CI Workflow (`.github/workflows/ci.yml`)

**Trigger:** Every pull request targeting `main`.

This workflow runs **2 jobs in parallel**:

### Job: Frontend

Validates the Vue/TypeScript frontend and the Nuxt landing page.

| Step | What it does |
|---|---|
| Checkout | Clone the repo |
| Setup Node.js 22 | Install Node runtime |
| Setup pnpm | Install pnpm package manager |
| Install dependencies | `pnpm install --frozen-lockfile` (strict, no lockfile changes) |
| Lint | `pnpm lint` — ESLint on all `apps/` |
| Typecheck desktop | `vue-tsc --noEmit` on `@app/desktop` |
| Build desktop frontend | `pnpm build:desktop` — Vite build |
| Build landing | `pnpm build:landing` — Nuxt build |

### Job: Backend (Rust)

Validates the Tauri/Rust backend. Runs from `apps/desktop/src-tauri/`.

| Step | What it does |
|---|---|
| Checkout | Clone the repo |
| Install system deps | WebKit, AppIndicator, librsvg, patchelf (Linux GTK deps for Tauri) |
| Setup Rust stable | Install Rust toolchain + clippy |
| Rust cache | Cache `target/` directory for faster builds |
| `cargo check` | Verify compilation without producing binaries |
| `cargo clippy -- -D warnings` | Lint Rust code, **fail on any warning** |
| `cargo test` | Run Rust unit tests |

### What you need to do

- Write [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `refactor:`, etc.)
- Open a PR targeting `main` — CI runs automatically
- Fix any lint/type/build/clippy errors before merging
- **No secrets needed** — this workflow only reads code

---

## 2. Release Please (`.github/workflows/release-please.yml`)

**Trigger:** Every push to `main` (i.e., after merging a PR).

### How it works

1. **Release Please** scans commit messages since the last release
2. If there are `feat:` or `fix:` commits, it opens/updates a **Release PR** that:
   - Bumps the version in `package.json` (root)
   - Updates the CHANGELOG.md
   - Syncs the version to all extra files:
     - `apps/desktop/package.json`
     - `apps/landing/package.json`
     - `apps/desktop/src-tauri/tauri.conf.json`
     - `apps/desktop/src-tauri/Cargo.toml`
3. When you **merge the Release PR**, it:
   - Creates a **GitHub Release** with a git tag (e.g., `v0.5.0`)
   - Sets `release_created = true`
   - Triggers the **Build Desktop** workflow

### Version bump rules

| Commit type | Version bump | Example |
|---|---|---|
| `fix:` | Patch | `0.4.0` -> `0.4.1` |
| `feat:` | Minor | `0.4.0` -> `0.5.0` |
| `feat!:` or `BREAKING CHANGE:` | Minor (pre-1.0) | `0.4.0` -> `0.5.0` |

> `bump-minor-pre-major: true` means breaking changes bump minor (not major) while version < 1.0.

### Changelog sections

| Commit prefix | Changelog section | Visible |
|---|---|---|
| `feat:` | Features | Yes |
| `fix:` | Bug Fixes | Yes |
| `perf:` | Performance | Yes |
| `refactor:` | Refactoring | Yes |
| `docs:` | Documentation | Yes |
| `chore:` | Miscellaneous | Hidden |

### What you need to do

- **Just merge PRs** with conventional commit messages — Release Please does the rest
- When a Release PR appears, review the changelog and merge it to cut a release
- **No manual version bumping** — everything is automated

---

## 3. Build Desktop (`.github/workflows/build-desktop.yml`)

**Trigger:** Called automatically by Release Please after a release, or manually via `workflow_dispatch`.

### Build matrix

Builds run **in parallel** on 4 targets:

| Platform | OS | Rust target | Output |
|---|---|---|---|
| Windows | `windows-latest` | default (x86_64) | `.msi`, `.exe` |
| macOS ARM | `macos-latest` | `aarch64-apple-darwin` | `.dmg`, `.app` |
| macOS Intel | `macos-latest` | `x86_64-apple-darwin` | `.dmg`, `.app` |
| Linux | `ubuntu-22.04` | default (x86_64) | `.deb`, `.AppImage` |

### Steps per platform

1. **Checkout** the repo
2. **Install system deps** (Linux only: WebKit, AppIndicator, etc.)
3. **Setup Node.js 22 + pnpm** — install frontend dependencies
4. **Setup Rust** with the target architecture
5. **Rust cache** — speed up compilation
6. **Build with `tauri-action`** — builds frontend + Rust backend, packages the app, and uploads artifacts to the GitHub Release

### Signing & auto-update

The build uses Tauri's signing system:

- `TAURI_SIGNING_PRIVATE_KEY` — signs the update bundles
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — key password
- `includeUpdaterJson: true` — generates the JSON manifest for in-app auto-updates

### What you need to do

- **Nothing for automatic releases** — triggered by merging the Release PR
- **Manual build**: Go to Actions > Build Desktop > Run workflow, provide `tag_name` and `version`
- **Secrets to configure** (in GitHub repo Settings > Secrets):

| Secret | Description | How to generate |
|---|---|---|
| `TAURI_SIGNING_PRIVATE_KEY` | Ed25519 private key for signing updates | `pnpm tauri signer generate -w ~/.tauri/craite.key` |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Password for the signing key | Set during key generation |

> `GITHUB_TOKEN` is automatically provided by GitHub Actions.

---

## 4. Dockerfile (Landing Page)

The Dockerfile builds the Nuxt landing page for production deployment.

### Multi-stage build

```
Stage 1: base        → Node 22 Alpine + pnpm
Stage 2: deps        → Install only @app/landing dependencies
Stage 3: build       → Copy deps + source, run `pnpm build`
Stage 4: production  → Minimal image with only .output/
```

### Usage

```bash
# Build the image
pnpm docker:landing:build

# Run on port 3000
pnpm docker:landing:run
```

The container exposes port **3000** and runs the Nuxt server (`node .output/server/index.mjs`).

---

## Complete Flow: From Code to Release

```
1. Create branch (feat/my-feature)
         │
         ▼
2. Write code + conventional commits
         │
         ▼
3. Open PR → main
         │
         ▼
4. CI runs ──────────────────────────────────┐
   ├─ Frontend: lint, typecheck, build       │
   └─ Backend: check, clippy, test           │
         │                                    │
         ▼                                    │
5. Fix issues if CI fails ◄──────────────────┘
         │
         ▼
6. Merge PR into main
         │
         ▼
7. Release Please runs
   ├─ No release commits? → Nothing happens
   └─ feat/fix commits? → Opens/updates Release PR
         │
         ▼
8. Review & merge Release PR
         │
         ▼
9. Release Please creates GitHub Release + tag
         │
         ▼
10. Build Desktop runs (4 platforms in parallel)
          │
          ▼
11. Signed artifacts uploaded to GitHub Release
    (users can download + auto-update works)
```

---

## Secrets Checklist

| Secret | Required for | Status |
|---|---|---|
| `GITHUB_TOKEN` | All workflows | Automatic |
| `TAURI_SIGNING_PRIVATE_KEY` | Build Desktop (signing) | Must configure |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Build Desktop (signing) | Must configure |

## Troubleshooting

| Problem | Solution |
|---|---|
| CI lint fails | Run `pnpm lint` locally and fix errors |
| Typecheck fails | Run `pnpm --filter @app/desktop exec vue-tsc --noEmit` locally |
| Clippy fails | Run `cargo clippy -- -D warnings` in `apps/desktop/src-tauri/` |
| Release PR not created | Check commit messages follow conventional commits format |
| Build fails on Linux | Ensure system deps are listed in the workflow |
| Signing error | Verify `TAURI_SIGNING_PRIVATE_KEY` secret is set correctly |
