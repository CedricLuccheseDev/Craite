import { ArrowDownToLine } from 'lucide-react'
import { platforms } from '~/content/landing'
import { useMessages } from '~/i18n'
import { formatSize, useRelease } from '~/lib/useRelease'

export function Download() {
  const m = useMessages()
  const { version, downloadFor } = useRelease()

  return (
    <section
      id="download"
      className="border-t border-hairline bg-card"
      aria-labelledby="download-title"
    >
      <div className="mx-auto max-w-5xl px-5 py-16 sm:px-8 sm:py-20">
        <p className="type-label">{m.download.label}</p>
        <h2
          id="download-title"
          className="mt-3 max-w-2xl font-display text-3xl font-bold tracking-tight text-balance sm:text-4xl"
        >
          {m.download.title}
        </h2>
        <p className="mt-5 max-w-xl text-base leading-relaxed text-pretty text-dim">
          {m.download.lead}
        </p>
        <p className="mt-2 max-w-xl text-base leading-relaxed font-medium text-pretty">
          {m.download.pitch}
        </p>

        <ul className="mt-10 grid gap-px border border-hairline bg-hairline sm:grid-cols-3">
          {platforms.map((platform) => {
            const file = downloadFor(platform.key)
            return (
              <li key={platform.key} className="bg-paper">
                {file ? (
                  <a
                    href={file.url}
                    download={file.filename}
                    className="flex min-h-11 flex-col gap-1 px-5 py-6 transition-colors duration-150 hover:bg-ink hover:text-paper"
                  >
                    <span className="flex items-center gap-2 font-medium">
                      <ArrowDownToLine aria-hidden className="size-4" />
                      {platform.label}
                    </span>
                    <span className="font-mono text-xs text-dim tabular-nums">
                      {formatSize(file.size)}
                    </span>
                  </a>
                ) : (
                  <div className="flex min-h-11 flex-col gap-1 px-5 py-6 opacity-50">
                    <span className="font-medium">{platform.label}</span>
                    <span className="font-mono text-xs text-dim">
                      {m.download.soon}
                    </span>
                  </div>
                )}
              </li>
            )
          })}
        </ul>

        <div className="mt-5 flex flex-wrap items-center gap-x-5 gap-y-2">
          {version ? (
            <p className="font-mono text-xs text-dim tabular-nums">
              {m.download.version} {version}
            </p>
          ) : null}
          <p className="font-mono text-xs text-dim">
            {m.download.badges.join(' · ')}
          </p>
        </div>
      </div>
    </section>
  )
}
