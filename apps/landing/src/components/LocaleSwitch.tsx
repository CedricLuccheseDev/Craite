import { useLocale, useMessages } from '~/i18n'
import { LOCALE_COOKIE } from '~/i18n/locale'

// The locale is resolved server-side, so switching writes the cookie and
// reloads rather than re-rendering with a client-side dictionary.
export function LocaleSwitch({ className = '' }: { className?: string }) {
  const locale = useLocale()
  const m = useMessages()

  function toggle() {
    const next = locale === 'fr' ? 'en' : 'fr'
    document.cookie = `${LOCALE_COOKIE}=${next};path=/;max-age=31536000;samesite=lax`
    window.location.reload()
  }

  return (
    <button
      type="button"
      onClick={toggle}
      lang={locale === 'fr' ? 'en' : 'fr'}
      className={`type-label inline-flex min-h-11 items-center px-2 transition-colors duration-150 hover:text-ink ${className}`}
    >
      {m.footer.localeSwitch}
    </button>
  )
}
