import { useEffect, useState } from 'react'
import type { PlatformDownload, ReleaseData } from '~/routes/api.release'
import type { PlatformKey } from '~/content/landing'

export function detectPlatform(): PlatformKey {
  if (typeof navigator === 'undefined') return 'windows'
  const ua = navigator.userAgent.toLowerCase()
  if (ua.includes('mac')) return 'mac_arm'
  if (ua.includes('linux') && !ua.includes('android')) return 'linux'
  return 'windows'
}

export function useRelease() {
  const [release, setRelease] = useState<ReleaseData>({
    version: null,
    platforms: {},
  })

  useEffect(() => {
    let cancelled = false
    fetch('/api/release')
      .then((res) => res.json() as Promise<ReleaseData>)
      .then((data) => {
        if (!cancelled && data.version) setRelease(data)
      })
      .catch(() => {})
    return () => {
      cancelled = true
    }
  }, [])

  // macOS ships as two builds; the page offers one button for both.
  function downloadFor(key: PlatformKey): PlatformDownload | undefined {
    if (key === 'mac_arm') {
      return release.platforms.mac_arm ?? release.platforms.mac_intel
    }
    return release.platforms[key]
  }

  return { version: release.version, downloadFor }
}

export function formatSize(bytes: number): string {
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`
}
