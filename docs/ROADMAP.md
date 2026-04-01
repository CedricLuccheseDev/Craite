# CrAIte - Roadmap & Go-to-Market

## Strategic Decisions

- **License**: AGPL v3 (open source + protection against commercial forks)
- **Monetization**: Launch 100% free. Monetization model TBD after user feedback (Phase 2, 500+ users)
- **Positioning**: "Open source. Free. 100% local." — vs Sononym ($89), Atlas ($99), Splice/Loopcloud subscriptions

---

## Tier 1: Pre-Launch (Required)

| # | Task | Effort |
|---|------|--------|
| 1.0 | Add AGPL v3 license (`LICENSE` + `package.json` + `Cargo.toml`) | S |
| 1.1 | Replace hero screenshot placeholders with real app screenshots | S |
| 1.2 | Add OG tags + Twitter Card meta (`pages/index.vue`) | S |
| 1.3 | Landing page i18n — EN default + FR (`@nuxtjs/i18n`) | M |
| 1.4 | Create `/privacy` page + footer link | S |
| 1.5 | Add Plausible analytics + download click events | S |
| 1.6 | Update messaging to match positioning | S |

## Tier 2: Post-Launch (Weeks 3-4)

| # | Task | Effort |
|---|------|--------|
| 2.1 | Extend audio classification beyond WAV (symphonia) | M |
| 2.2 | "Correct category" button in SampleList + corrections table | M |
| 2.3 | Create Discord community | S |
| 2.4 | Rewrite README — user-facing top + dev docs below | M |
| 2.5 | In-app DAW integration guide (per detected DAW) | M |
| 2.6 | Merge Dependabot PRs + ship v0.5.0 | S-M |

## Tier 3: Backlog (Data-Driven)

- Vue component tests (L)
- ML classification model (XL)
- Content marketing / SEO (L, ongoing)
- Enriched subcategories + data-driven rules (M)
- Opt-in anonymous telemetry (L)

---

## Launch Channels

- Reddit: r/edmproduction, r/WeAreTheMusicMakers, r/makinghiphop, r/FL_Studio, r/ableton
- KVR Audio forum
- Discord: producer communities
- GitHub: tags (sample-organizer, music-production, audio, tauri, rust)

## Competitive Landscape

| Tool | Price | Model | Differentiator |
|------|-------|-------|----------------|
| Sononym | $89 | One-time | ML timbre classification |
| Atlas 2 | $99 | One-time | Drums only + beat maker |
| XO | $148 | One-time | Visual exploration + sequencer |
| Cosmos | $30 | Freemium | Waves ecosystem |
| ADSR | Free | Freemium | Basic browser + ADSR cloud |
| Loopcloud | $8-12/mo | Subscription | Loopmasters library |
| Splice | $10-30/mo | Subscription | Marketplace + browser |
| SoundMiner | $199-899 | One-time | Enterprise (Hollywood/AAA) |
| **CrAIte** | **Free** | **Open source** | **Only: AGPL + local + all DAWs + Linux** |
