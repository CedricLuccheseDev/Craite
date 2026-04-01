# CrAIte — TODO

## Pre-Launch (Tier 1)

### Manual tasks (can't be done by code)

- [ ] **Screenshots** — Take real before/after screenshots of the app and replace placeholders in `apps/landing/components/landing/HeroSection.vue` (lines 68-86). Before: messy folder tree. After: CrAIte organized library. Optimize for web (~1200px wide, WebP or PNG)
- [ ] **OG image** — Create `apps/landing/public/og-image.png` (1200x630px). Should contain: CrAIte logo, tagline, and a mini screenshot. This image appears when sharing links on Discord/Reddit/Twitter
- [ ] **Plausible account** — Create a free account on [plausible.io](https://plausible.io) (or self-host). Add your domain. Verify that `data-domain` in `apps/landing/nuxt.config.ts` matches your actual domain
- [ ] **Discord server** — Create a Discord with channels: #general, #bug-reports, #feature-requests, #showcase. Add invite link to landing footer and app settings

### Code tasks (next session)

- [ ] **Landing i18n** — Install `@nuxtjs/i18n`, extract all hardcoded French strings from landing components, create EN (default) + FR locales
- [ ] **Merge Dependabot PRs** — 13 pending PRs to review and merge

## Post-Launch (Tier 2)

- [ ] **Audio classification multi-format** — Extend `classifier/audio.rs` beyond WAV (use symphonia for MP3/FLAC/OGG)
- [ ] **"Correct category" button** — Add correction UI in `SampleList.vue` + `corrections` table in SQLite
- [ ] **DAW integration guide in-app** — Contextual tips per detected DAW in `LibraryConfig.vue` (guide user to add CrAIte folder to DAW browser)
- [ ] **README rewrite** — User-facing section on top (screenshots, value prop, download) + dev docs below

## UX / Onboarding

- [ ] Guide user to remove source folders (Splice, etc.) from DAW browser and only add the CrAIte folder — avoids duplicates
