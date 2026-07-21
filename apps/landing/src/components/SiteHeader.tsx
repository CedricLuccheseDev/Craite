import { Wordmark } from './Wordmark'
import { LocaleSwitch } from './LocaleSwitch'
import { useMessages } from '~/i18n'

const sections = [
  { id: 'how', key: 'how' },
  { id: 'faq', key: 'faq' },
] as const

export function SiteHeader() {
  const m = useMessages()

  return (
    <header className="sticky top-0 z-50 border-b border-hairline bg-paper">
      <div className="mx-auto flex h-14 max-w-5xl items-center justify-between px-5 sm:px-8">
        <a
          href="#top"
          className="-mx-2 inline-flex min-h-11 items-center px-2 text-lg"
        >
          <Wordmark />
        </a>

        <nav className="flex items-center gap-1">
          {sections.map((section) => (
            <a
              key={section.id}
              href={`#${section.id}`}
              className="type-label hidden min-h-11 items-center px-3 transition-colors duration-150 hover:text-ink sm:inline-flex"
            >
              {m.nav[section.key]}
            </a>
          ))}
          <LocaleSwitch className="hidden sm:inline-flex" />
          <a
            href="#download"
            className="ml-1 inline-flex min-h-11 items-center border border-ink bg-ink px-4 text-sm font-medium text-paper transition-colors duration-150 hover:border-accent-hover hover:bg-accent-hover active:scale-[0.98]"
          >
            {m.nav.download}
          </a>
        </nav>
      </div>
    </header>
  )
}
