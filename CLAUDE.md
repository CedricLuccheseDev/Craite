# CrAIte - Project Conventions

## Architecture
- **Tauri v2** desktop app: Rust backend + Vue 3/TS frontend
- Package manager: **pnpm**
- Frontend: Vite + Vue 3 + TypeScript + Pinia
- Backend: Rust with modular structure in `src-tauri/src/`

## File Structure
- Max **200 lines per file** - split into modules
- Rust modules follow `mod.rs` pattern
- Vue components use `<script setup lang="ts">`

## Naming
- Rust: snake_case for files, functions, variables
- Vue/TS: PascalCase for components, camelCase for functions/variables
- CSS: kebab-case for classes, CSS custom properties for theming

## Commands
- `pnpm dev` — Start Vite dev server
- `pnpm tauri dev` — Start full Tauri dev mode
- `pnpm build` — Build frontend
- `pnpm tauri build` — Build distributable
- `pnpm lint` — Run ESLint

## Key Paths
- Frontend: `src/`
- Backend: `src-tauri/src/`
- Types shared between Rust/TS: keep in sync manually
  - Rust: `src-tauri/src/db/models.rs`
  - TS: `src/types/sample.ts`

## Tauri Commands
- All Tauri commands live in `src-tauri/src/commands/`
- Frontend invokes via `@tauri-apps/api/core` `invoke()`
- Wrapper composable: `src/composables/useTauri.ts`

## Styling
- Dark theme only (background: #0a0a0a)
- CSS custom properties in `src/assets/styles/variables.css`
- Animations in `src/assets/styles/animations.css`
- Accent color: orange (#ff6b35)
