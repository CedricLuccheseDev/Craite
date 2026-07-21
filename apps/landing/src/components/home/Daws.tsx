import { daws } from '~/content/landing'
import { useMessages } from '~/i18n'

// The logos are the point here, so they sit in a plain grid of hairline cells
// rather than in cards.
export function Daws() {
  const m = useMessages()

  return (
    <section
      id="daws"
      className="mx-auto max-w-5xl px-5 py-12 sm:px-8 sm:py-16"
    >
      <p className="type-label">{m.daws.label}</p>
      <h2 className="mt-2 font-display text-2xl font-bold tracking-tight text-balance sm:text-3xl">
        {m.daws.title}
      </h2>

      <ul className="mt-8 grid grid-cols-2 gap-px border border-hairline bg-hairline sm:grid-cols-3 lg:grid-cols-5">
        {daws.map((daw) => (
          <li
            key={daw.slug}
            className="flex flex-col items-center justify-center gap-3 bg-card px-4 py-8"
          >
            <img
              src={`/daws/${daw.slug}.svg`}
              alt=""
              width={32}
              height={32}
              loading="lazy"
              className="size-8 opacity-70"
            />
            <span className="text-center font-mono text-xs text-dim">
              {daw.name}
            </span>
          </li>
        ))}
      </ul>
    </section>
  )
}
