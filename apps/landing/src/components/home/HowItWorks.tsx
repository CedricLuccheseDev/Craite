import { SectionHeading } from '../SectionHeading'
import { steps } from '~/content/landing'
import { useMessages } from '~/i18n'

export function HowItWorks() {
  const m = useMessages()

  return (
    <section id="how" className="mx-auto max-w-5xl px-5 py-12 sm:px-8 sm:py-16">
      <SectionHeading index="01" label={m.how.label} title={m.how.title}>
        {m.how.intro}
      </SectionHeading>

      <ol className="grid sm:grid-cols-3">
        {steps.map((id, index) => {
          const copy = m.steps[id]
          return (
            <li
              key={id}
              className="border-b border-hairline py-6 sm:border-b-0 sm:pr-8"
            >
              <span className="font-mono text-xs text-accent tabular-nums">
                {String(index + 1).padStart(2, '0')}
              </span>
              <h3 className="mt-3 font-display font-semibold tracking-tight">
                {copy.title}
              </h3>
              <p className="mt-2 text-sm leading-relaxed text-pretty text-dim">
                {copy.description}
              </p>
            </li>
          )
        })}
      </ol>
    </section>
  )
}
