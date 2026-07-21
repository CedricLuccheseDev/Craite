import { Plus } from 'lucide-react'
import { SectionHeading } from '../SectionHeading'
import { faqItems } from '~/content/landing'
import { useMessages } from '~/i18n'

export function Faq() {
  const m = useMessages()

  return (
    <section id="faq" className="mx-auto max-w-5xl px-5 py-12 sm:px-8 sm:py-16">
      <SectionHeading index="03" label={m.faq.label} title={m.faq.title} />

      <div>
        {faqItems.map((id) => {
          const copy = m.faqItems[id]
          return (
            <details key={id} className="group border-b border-hairline">
              <summary className="flex min-h-14 cursor-pointer list-none items-center justify-between gap-4 py-4 font-medium transition-colors duration-150 hover:text-accent">
                {copy.question}
                <Plus
                  aria-hidden
                  className="size-4 shrink-0 text-dim transition-transform duration-150 group-open:rotate-45"
                />
              </summary>
              <p className="max-w-2xl pb-5 text-sm leading-relaxed text-pretty text-dim">
                {copy.answer}
              </p>
            </details>
          )
        })}
      </div>
    </section>
  )
}
