import type { ReactNode } from 'react'

// Specimen-sheet rule: an oversized folio number in the margin, the title set
// against it, a hairline closing the block.
export function SectionHeading({
  index,
  label,
  title,
  children,
}: {
  index: string
  label: string
  title: string
  children?: ReactNode
}) {
  return (
    <div className="grid grid-cols-[auto_1fr] items-baseline gap-x-4 border-b border-hairline pb-5 sm:gap-x-8">
      <span
        aria-hidden
        data-folio={index}
        className="folio font-display text-4xl leading-none font-bold tabular-nums sm:text-6xl"
      />
      <div>
        <p className="type-label mb-2">{label}</p>
        <h2 className="font-display text-2xl font-bold tracking-tight text-balance sm:text-3xl">
          {title}
        </h2>
        {children ? (
          <p className="mt-2 max-w-xl text-sm text-pretty text-dim">
            {children}
          </p>
        ) : null}
      </div>
    </div>
  )
}
