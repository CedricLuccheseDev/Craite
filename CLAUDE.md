# CrAIte - Project Conventions

## Architecture
- **pnpm monorepo** with 2 apps:
  - `@app/desktop` — Tauri v2 desktop app (Rust backend + Vue 3/TS frontend)
  - `@app/landing` — TanStack Start landing page (React 19)
- Package manager: **pnpm** (workspaces)
- Desktop frontend: Vite + Vue 3 + TypeScript + Pinia
- Desktop backend: Rust with modular structure in `apps/desktop/src-tauri/src/`
- Landing: TanStack Start + Tailwind v4, on the shared identity in `~/Perso/CLAUDE.md`

## File Structure
- Max **200 lines per file** - split into modules
- Rust modules follow `mod.rs` pattern
- Vue components (desktop) use `<script setup lang="ts">`
- Landing: file-based routes in `apps/landing/src/routes/`, copy in
  `src/i18n/` (fr is the source dictionary, en translates it), structural data
  in `src/content/`. `/api/release` proxies the S3 manifest that feeds the
  download buttons.

## Naming
- Rust: snake_case for files, functions, variables
- Vue/TS: PascalCase for components, camelCase for functions/variables
- CSS: kebab-case for classes, CSS custom properties for theming

## Commands
- `pnpm dev` — Start Tauri dev mode (frontend + Rust backend)
- `pnpm dev:landing` — Start the landing dev server (vite)
- `pnpm build` — Build Tauri distributable
- `pnpm build:landing` — Build landing for production
- `pnpm lint` — Run ESLint on all apps
- `pnpm lint:fix` — Lint and auto-fix
- `pnpm format` — Format all files with Prettier
- `pnpm format:check` — Check formatting
- `pnpm test` — Run tests (Vitest)
- `pnpm typecheck` — Typecheck desktop + landing

## Key Paths
- Desktop frontend: `apps/desktop/src/`
- Desktop backend: `apps/desktop/src-tauri/src/`
- Landing page: `apps/landing/`
- Types shared between Rust/TS: keep in sync manually
  - Rust: `apps/desktop/src-tauri/src/db/models.rs`
  - TS: `apps/desktop/src/types/sample.ts`

## Tauri Commands
- All Tauri commands live in `apps/desktop/src-tauri/src/commands/`
- Frontend invokes via `@tauri-apps/api/core` `invoke()`
- Wrapper composable: `apps/desktop/src/composables/useTauri.ts`

## Project Locations
- **CrAIte repo (WSL)**: `/home/quyver/Perso/CrAIte/`
- **CrAIte repo (Windows)**: `G:\Mon Drive\1 - Projects\CrAIte\`
- **Documentation & Design**: `G:\Mon Drive\1 - Projects\CrAIte\` — contains the landing page mockup and other design/spec files

## Commits
- Always use the project `/commit` skill (`.claude/commands/commit.md`), never the global one
- `/commit` — bump patch + commit
- `/commit minor` — bump minor + commit
- `/commit major` — bump major + commit
- Version must be bumped in all 5 files: `package.json` (root, desktop, landing), `tauri.conf.json`, `Cargo.toml`

## Styling

The two apps no longer share a theme, on purpose.

**Desktop** — dark only (background: #0a0a0a), accent orange #ff6b35.
- CSS custom properties in `apps/desktop/src/assets/styles/variables.css`
- Animations in `apps/desktop/src/assets/styles/animations.css`

**Landing** — the shared identity from `~/Perso/CLAUDE.md`: paper ground, print
ink, hairline rules, square corners, light only. Accent is #b03c12, a darkened
#ff6b35: the original only reaches 2.57:1 on paper and is unreadable as text.
Tokens in `apps/landing/src/styles/app.css`.
