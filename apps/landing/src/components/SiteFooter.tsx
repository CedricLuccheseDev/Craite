import { Wordmark } from './Wordmark'
import { LocaleSwitch } from './LocaleSwitch'
import { REPO } from '~/content/landing'
import { useMessages } from '~/i18n'

export function SiteFooter() {
  const m = useMessages()
  const year = new Date().getFullYear()

  const links = [
    { href: '#how', label: m.nav.how },
    { href: '#faq', label: m.nav.faq },
    { href: '#download', label: m.nav.download },
  ]
  const resources = [
    { href: REPO, label: m.footer.github },
    { href: `${REPO}/issues`, label: m.footer.issues },
    { href: `${REPO}/releases`, label: m.footer.releases },
  ]

  return (
    <footer className="border-t border-hairline">
      <div className="mx-auto grid max-w-5xl gap-10 px-5 py-12 sm:grid-cols-3 sm:px-8">
        <div>
          <Wordmark className="text-lg" />
          <p className="mt-3 max-w-xs text-sm text-pretty text-dim">
            {m.footer.tagline}
          </p>
        </div>

        <FooterColumn title={m.footer.product} items={links} />
        <FooterColumn title={m.footer.resources} items={resources} external />
      </div>

      <div className="mx-auto flex max-w-5xl flex-wrap items-center justify-between gap-2 border-t border-hairline px-5 py-6 sm:px-8">
        <p className="font-mono text-xs text-dim">
          © {year} craite. {m.footer.rights}
        </p>
        <LocaleSwitch className="-mx-2" />
      </div>
    </footer>
  )
}

function FooterColumn({
  title,
  items,
  external = false,
}: {
  title: string
  items: Array<{ href: string; label: string }>
  external?: boolean
}) {
  return (
    <div>
      <p className="type-label">{title}</p>
      <ul className="mt-2">
        {items.map((item) => (
          <li key={item.href}>
            <a
              href={item.href}
              target={external ? '_blank' : undefined}
              rel={external ? 'noreferrer' : undefined}
              className="inline-flex min-h-11 items-center text-sm text-dim transition-colors duration-150 hover:text-ink"
            >
              {item.label}
            </a>
          </li>
        ))}
      </ul>
    </div>
  )
}
