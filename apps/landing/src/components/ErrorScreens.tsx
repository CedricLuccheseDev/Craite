import { Link } from '@tanstack/react-router'
import { useMessages } from '~/i18n'

// Router-level fallbacks. The error screen deliberately shows no detail:
// stack traces stay in the console, never in the page.
export function ErrorScreen() {
  const m = useMessages()

  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-3 px-5 text-center">
      <p className="text-xl font-bold">{m.errors.title}</p>
      <p className="text-dim">{m.errors.tryReload}</p>
      <button
        onClick={() => window.location.reload()}
        className="mt-4 bg-accent px-6 py-3 font-semibold text-paper transition-colors hover:bg-accent-hover"
      >
        {m.errors.reload}
      </button>
    </main>
  )
}

export function NotFoundScreen() {
  const m = useMessages()
  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-3 px-5 text-center">
      <p className="font-display text-8xl font-bold tracking-tight text-hairline">
        404
      </p>
      <p className="text-dim">{m.errors.pageNotFound}</p>
      <Link
        to="/"
        className="mt-4 text-sm font-medium text-dim underline underline-offset-4 hover:text-ink"
      >
        {m.errors.backHome}
      </Link>
    </main>
  )
}
