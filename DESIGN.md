# CrAIte — Design Direction

Shared design system for `@app/desktop` and `@app/landing`.

## Philosophy

Inspired by **Linear**, **Notion**, **Framer** — ultra-clean, generous whitespace, large typography, flat surfaces. Every element earns its place. If it's not essential, remove it.

## Color Palette (Dark)

Both apps share the same dark palette based on Tailwind **zinc** scale + orange accent.

| Token              | Value     | Tailwind   | Usage                        |
|--------------------|-----------|------------|------------------------------|
| `bg`               | `#0a0a0a` | —          | Page background              |
| `surface`          | `#18181b` | zinc-900   | Cards, panels                |
| `surface-hover`    | `#27272a` | zinc-800   | Hover states                 |
| `border`           | `#27272a` | zinc-800   | Dividers, card borders       |
| `text`             | `#ffffff` | white      | Primary text                 |
| `text-muted`       | `#a1a1aa` | zinc-400   | Secondary text, labels       |
| `accent-orange`    | `#ff6b35` | orange-500 | Brand accent, CTAs           |
| `accent-blue`      | `#3b82f6` | blue-500   | Info, links                  |
| `accent-green`     | `#22c55e` | green-500  | Success states               |
| `danger`           | `#ef4444` | red-500    | Errors, destructive actions  |

Landing also uses: `bg-zinc-900/30`, `bg-zinc-900/50` for muted card backgrounds, `bg-[#ff6b35]/10` for soft orange tint on icons.

### Category Colors (samples)

| Category | Color     |
|----------|-----------|
| Kick     | `#ff6b35` |
| Snare    | `#3b82f6` |
| Hihat    | `#eab308` |
| Clap     | `#8b5cf6` |
| Pad      | `#22c55e` |
| Lead     | `#ec4899` |
| Bass     | `#ef4444` |
| FX       | `#06b6d4` |
| Vocal    | `#f97316` |
| Loop     | `#6366f1` |

## Typography

- **Font family:** Inter (sans), JetBrains Mono (mono)
- **Font smoothing:** antialiased on all platforms

| Role       | Size    | Weight | Tracking     |
|------------|---------|--------|--------------|
| Hero title | 104px   | 900    | -6px         |
| Page title | 40px    | 800    | -1.5px       |
| Section    | 24px    | 700    | -0.5px       |
| Body       | 16-18px | 400    | normal       |
| Label      | 13px    | 500-600| 0.12em upper |
| Caption    | 12px    | 400    | normal       |
| Mono data  | 14px    | 400    | tabular-nums |

## Spacing Scale

Generous whitespace is non-negotiable.

| Token  | Value | Usage                   |
|--------|-------|-------------------------|
| `xs`   | 4px   | Tight inline gaps       |
| `sm`   | 8px   | Compact element spacing |
| `md`   | 16px  | Default component gap   |
| `lg`   | 24px  | Section padding         |
| `xl`   | 32px  | Card padding            |
| `2xl`  | 48px  | Section gaps            |
| `3xl`  | 64px  | Major section gaps      |
| `4xl`  | 96px  | Hero spacing            |

**Rule:** When in doubt, add more space. Crowded UI is the enemy.

## Border Radius

| Token  | Value  |
|--------|--------|
| `sm`   | 6px    |
| `md`   | 10px   |
| `lg`   | 16px   |
| `full` | 9999px |

## Component Library

Both apps use **Nuxt UI v4** components:
- Desktop: via `@nuxt/ui/vite` plugin
- Landing: via `@nuxt/ui` Nuxt module

Primary components: `UButton`, `UCard`, `UInput`, `UBadge`, `UCheckbox`, `UProgress`.

### Button Variants

| Context        | Color     | Variant   | Size |
|----------------|-----------|-----------|------|
| Primary CTA    | `primary` | `solid`   | `xl` |
| Secondary CTA  | `primary` | `solid`   | `lg` |
| Tertiary       | `neutral` | `outline` | `md` |
| Ghost action   | `neutral` | `ghost`   | `sm` |

## Animations

- **Duration:** 150ms (fast), 300ms (normal), 500ms (slow)
- **Easing:** `cubic-bezier(.16, 1, .3, 1)` (expo out)
- Subtle slide-up on page transitions
- Stagger-in for list items (50-80ms delay per item)
- No gratuitous motion — animate to inform, not to decorate

## Layout Principles

1. **Center-aligned onboarding** — content vertically and horizontally centered, max-width constrained (360-440px)
2. **Sidebar + content for library** — 240px fixed sidebar, fluid main area
3. **No shadows** — use borders (`1px solid border`) for separation
4. **Flat surfaces** — no gradients, no elevation, no blur. Color alone defines hierarchy
5. **Generous content padding** — minimum 48px horizontal on desktop, 64px for hero sections

## Do / Don't

**Do:**
- Use `text-muted` for secondary info instead of smaller size
- Keep interactive elements large (min 44px touch target)
- Use monospace for numeric data
- Use uppercase + letter-spacing for labels/categories
- Leave empty space — it's a feature, not a bug

**Don't:**
- Add box shadows or drop shadows
- Use gradient text effects
- Add decorative icons without purpose
- Stack too many elements vertically without spacing
- Use borders AND background color on the same element
