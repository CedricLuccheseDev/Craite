import { SectionHeading } from '../SectionHeading'
import { trustPoints } from '~/content/landing'
import { useMessages } from '~/i18n'

export function Trust() {
  const m = useMessages()

  return (
    <section
      id="trust"
      className="mx-auto max-w-5xl px-5 py-12 sm:px-8 sm:py-16"
    >
      <SectionHeading index="02" label={m.trust.label} title={m.trust.title}>
        {m.trust.intro}
      </SectionHeading>

      <ul>
        {trustPoints.map((id) => {
          const copy = m.trustPoints[id]
          return (
            <li
              key={id}
              className="grid gap-x-8 gap-y-2 border-b border-hairline py-6 sm:grid-cols-[16rem_1fr]"
            >
              <h3 className="font-display font-semibold tracking-tight text-balance">
                {copy.title}
              </h3>
              <p className="max-w-2xl text-sm leading-relaxed text-pretty text-dim">
                {copy.description}
              </p>
            </li>
          )
        })}
      </ul>
    </section>
  )
}
