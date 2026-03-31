# CrAIte - Project Conventions

## Architecture
- **pnpm monorepo** with 2 apps:
  - `@app/desktop` — Tauri v2 desktop app (Rust backend + Vue 3/TS frontend)
  - `@app/landing` — Nuxt 3 landing page with @nuxt/ui
- Package manager: **pnpm** (workspaces)
- Desktop frontend: Vite + Vue 3 + TypeScript + Pinia
- Desktop backend: Rust with modular structure in `apps/desktop/src-tauri/src/`
- Landing: Nuxt 3 + @nuxt/ui

## File Structure
- Max **200 lines per file** - split into modules
- Rust modules follow `mod.rs` pattern
- Vue components use `<script setup lang="ts">`

## Naming
- Rust: snake_case for files, functions, variables
- Vue/TS: PascalCase for components, camelCase for functions/variables
- CSS: kebab-case for classes, CSS custom properties for theming

## Commands
- `pnpm dev` — Start Tauri dev mode (frontend + Rust backend)
- `pnpm dev:landing` — Start Nuxt dev server (landing page)
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
- Dark theme only (background: #0a0a0a)
- CSS custom properties in `apps/desktop/src/assets/styles/variables.css`
- Animations in `apps/desktop/src/assets/styles/animations.css`
- Accent color: orange (#ff6b35)
