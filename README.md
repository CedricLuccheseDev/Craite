# CrAIte

AI-powered sample organizer for producers. Scans your sample folders, classifies sounds by type (kick, snare, pad...), and creates a mirror folder structure via hardlinks/symlinks for use in browser.

## Prerequisites

Install the following tools before anything else.

**Node.js >= 18**
```bash
# via nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
nvm install 18
```

**pnpm >= 8**
```bash
npm install -g pnpm
```

**Rust >= 1.70**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**System dependencies (Linux only)**
```bash
sudo apt-get install -y \
  pkg-config \
  build-essential \
  libwebkit2gtk-4.1-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libasound2-dev
```

**System dependencies (macOS only)**
```bash
xcode-select --install
```

**System dependencies (Windows only)**

Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with C++ workload and [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).

## Setup

```bash
git clone git@github.com:CedricLuccheseDev/Craite.git
cd Craite
pnpm install
```

## Development

**Desktop app (Tauri + Vue)**
```bash
pnpm dev
```
Ouvre la fenêtre Tauri avec hot reload. Vite tourne sur `localhost:1420`.

**Landing page**

Créer `apps/landing/.env` :
```
GITHUB_TOKEN=<ton fine-grained token — voir section Deploy>
```

```bash
pnpm dev:landing
```
Accessible sur `http://localhost:3000`.

## Build

```bash
# Desktop — distributable dans apps/desktop/src-tauri/target/release/bundle/
pnpm build

# Landing
pnpm build:landing
```

## Other commands

```bash
pnpm lint           # ESLint on all apps
pnpm lint:fix       # ESLint with auto-fix
pnpm format         # Prettier on all files
pnpm format:check   # Check formatting without writing
pnpm test           # Run Vitest tests
pnpm test:coverage  # Run tests with coverage report
pnpm typecheck      # Type-check desktop + landing
```

## Project Structure

```
apps/
  desktop/                # Tauri v2 desktop app
    src/                  # Vue 3 / TypeScript frontend
      components/         # UI components (onboarding, library, common)
      composables/        # Vue composables (useTauri, useOnboarding)
      stores/             # Pinia stores (scan, library)
      views/              # Route views (Onboarding, Library)
      types/              # TypeScript interfaces
    src-tauri/src/        # Rust backend
      commands/           # Tauri IPC commands
      scanner/            # Filesystem scanning + Splice DB reader
      classifier/         # Filename-based classification
      linker/             # Hardlink / symlink / copy strategies
      watcher/            # File system watcher (notify)
      audio/              # Audio preview (rodio)
      db/                 # SQLite database (rusqlite)

  landing/                # Nuxt 3 landing page
    pages/                # Nuxt pages
    composables/          # useGithubRelease
    server/api/           # Proxy routes (release, download)
    app.vue               # Root app component
```

## Tech Stack

| Layer    | Technology                        |
|----------|-----------------------------------|
| Desktop  | Tauri v2                          |
| Frontend | Vue 3 + TypeScript + Vite + Pinia |
| Backend  | Rust                              |
| Database | SQLite (rusqlite)                 |
| Audio    | rodio                             |
| Watcher  | notify                            |
| Landing  | Nuxt 3 + @nuxt/ui                 |

## Release (desktop)

La release est **manuelle** : tu bumpes la version, tu pousses un tag, le CI fait le reste.

### Étapes dans l'ordre

**1. Bumper la version** (ex: `0.4.0` → `0.5.0`)

Fichiers critiques — l'app et l'auto-updater ne fonctionneront pas correctement sans :

| Fichier | Clé |
|---------|-----|
| `apps/desktop/src-tauri/tauri.conf.json` | `"version"` |
| `apps/desktop/src-tauri/Cargo.toml` | `version = "..."` |

Fichiers optionnels — cohérence uniquement, aucun impact fonctionnel :

| Fichier | Clé |
|---------|-----|
| `package.json` | `"version"` |
| `apps/desktop/package.json` | `"version"` |
| `apps/landing/package.json` | `"version"` |

**2. Committer et merger dans `main`**

```bash
git add -p
git commit -m "chore: release v0.5.0"
# ouvrir une PR dev → main et la merger
```

**3. Créer et pousser le tag depuis `main`**

```bash
git checkout main && git pull
git tag v0.5.0
git push origin v0.5.0
```

**→ Le CI se déclenche automatiquement**, build pour Windows / macOS / Linux, crée la GitHub Release avec les assets signés et le `latest.json` → l'auto-update est actif.

> Ne jamais pousser un tag depuis `dev` ou une branche feature.

## Deploy

### Créer le token GitHub (fine-grained)

1. Aller sur [github.com/settings/personal-access-tokens/new](https://github.com/settings/personal-access-tokens/new)
2. **Token name** : `craite-landing`
3. **Resource owner** : `CedricLuccheseDev`
4. **Repository access** : Only selected repositories → `CedricLuccheseDev/Craite`
5. **Permissions** → Repository permissions → **Contents** : `Read-only`
6. Générer et copier le token

### Déployer la landing

Ajouter la variable d'environnement suivante sur l'hébergeur :
```
GITHUB_TOKEN=<token généré ci-dessus>
```

**Via Docker :**
```bash
docker build -t craite-landing .
docker run -p 3000:3000 -e GITHUB_TOKEN=<token> craite-landing
```

## Auto-update

Au démarrage, l'app vérifie `https://craite.app/api/update`. Si une nouvelle version est disponible, une popup s'affiche.

**Ce qui est déjà en place :**
- Rust : `commands/updater.rs` — commandes `check_for_update` et `download_and_install_update`
- Vue : `composables/useUpdater.ts` + `components/UpdateNotification.vue`
- Nuxt : `server/api/update.ts` + `server/api/download/[id].ts` (proxy GitHub avec token)
- Clés de signature configurées dans `tauri.conf.json`

**Seul prérequis CI — secret GitHub Actions :**

GitHub → `CedricLuccheseDev/Craite` → Settings → Secrets and variables → Actions → New repository secret :
- Name : `TAURI_SIGNING_PRIVATE_KEY`
- Value : la clé privée Tauri (demander à @CedricLuccheseDev)

Le workflow passe ensuite ce secret à `tauri-action` :
```yaml
env:
  TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
```

Cela génère un `.sig` pour chaque asset de release, requis pour valider la mise à jour.
