import { createFileRoute } from '@tanstack/react-router'

// Proxies the release manifest published to S3 by the desktop build. Keeping
// it server-side means the bucket URL stays configurable through RELEASE_URL
// and the browser never needs to know where releases actually live.
const RELEASE_URL = process.env.RELEASE_URL ?? 'https://dl.craite.clhub.fr'

export type PlatformDownload = {
  url: string
  filename: string
  size: number
}

export type ReleaseData = {
  version: string | null
  // Which platforms a release ships is decided by the desktop build, so no key
  // is guaranteed to be there.
  platforms: Record<string, PlatformDownload | undefined>
}

export const Route = createFileRoute('/api/release')({
  server: {
    handlers: {
      GET: async () => {
        try {
          const res = await fetch(`${RELEASE_URL}/latest.json`)
          if (!res.ok) throw new Error(`upstream ${res.status}`)
          // Upstream JSON is not validated by anyone, so treat it as partial.
          const data = (await res.json()) as Partial<ReleaseData>
          if (!data.version) throw new Error('no version in manifest')
          return Response.json(
            { version: data.version, platforms: data.platforms ?? {} },
            { headers: { 'cache-control': 'public, max-age=300' } },
          )
        } catch (error) {
          console.error('Failed to fetch release info:', error)
          // The page must still render with the download buttons disabled.
          return Response.json({ version: null, platforms: {} })
        }
      },
    },
  },
})
