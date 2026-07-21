import { ArrowDown, FolderTree, ListTree } from 'lucide-react'
import type { CSSProperties } from 'react'
import { useMessages } from '~/i18n'

export function Hero() {
  const m = useMessages()

  return (
    <section
      id="top"
      className="mx-auto max-w-5xl px-5 pt-12 pb-12 sm:px-8 sm:pt-16 sm:pb-16"
    >
      <p className="reveal type-label" style={{ '--step': 0 } as CSSProperties}>
        {m.hero.badge}
      </p>

      <h1
        className="reveal mt-6 max-w-3xl font-display text-[clamp(2.25rem,7vw,4.25rem)] leading-[0.95] font-bold tracking-tight text-balance"
        style={{ '--step': 1 } as CSSProperties}
      >
        {m.hero.title}
      </h1>

      <p className="mt-6 max-w-xl border-t border-hairline pt-6 text-base leading-relaxed text-pretty text-dim">
        {m.hero.intro}
      </p>

      <div
        className="reveal mt-7 flex flex-wrap gap-3"
        style={{ '--step': 2 } as CSSProperties}
      >
        <a
          href="#download"
          className="inline-flex min-h-11 items-center border border-ink bg-ink px-5 py-3 text-sm font-medium text-paper transition-colors duration-150 hover:border-accent-hover hover:bg-accent-hover active:scale-[0.98]"
        >
          {m.hero.download}
        </a>
        <a
          href="#how"
          className="inline-flex min-h-11 items-center gap-2 border border-hairline px-5 py-3 text-sm font-medium transition-colors duration-150 hover:border-ink active:scale-[0.98]"
        >
          {m.hero.seeHow}
          <ArrowDown aria-hidden className="size-4" />
        </a>
      </div>

      <BeforeAfter />
    </section>
  )
}

// Two facing plates, the way a printed comparison would set them: same frame,
// same weight, only the content differs.
function BeforeAfter() {
  const m = useMessages()

  return (
    <div className="mt-14">
      <p className="type-label">{m.hero.beforeAfter}</p>
      <div className="mt-4 grid gap-px border border-hairline bg-hairline sm:grid-cols-2">
        <figure className="bg-card p-6">
          <figcaption className="flex items-baseline justify-between">
            <span className="font-mono text-xs text-dim">{m.hero.before}</span>
          </figcaption>
          <FolderTree
            aria-hidden
            className="mt-6 size-8 text-hairline"
            strokeWidth={1.25}
          />
          <p className="mt-4 text-sm text-pretty text-dim">
            {m.hero.beforeCaption}
          </p>
        </figure>

        <figure className="bg-card p-6">
          <figcaption className="flex items-baseline justify-between">
            <span className="font-mono text-xs text-accent">
              {m.hero.after}
            </span>
          </figcaption>
          <ListTree
            aria-hidden
            className="mt-6 size-8 text-accent"
            strokeWidth={1.25}
          />
          <p className="mt-4 text-sm text-pretty text-dim">
            {m.hero.afterCaption}
          </p>
        </figure>
      </div>
    </div>
  )
}
