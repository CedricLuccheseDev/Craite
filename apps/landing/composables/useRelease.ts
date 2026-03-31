interface PlatformDownload {
  url: string;
  filename: string;
  size: number;
}

interface ReleaseData {
  version: string | null;
  platforms: Record<string, PlatformDownload>;
}

export function useRelease() {
  const version = useState<string>('release-version', () => '');
  const platforms = useState<Record<string, PlatformDownload>>('release-platforms', () => ({}));
  const hasRelease = useState<boolean>('release-has', () => false);

  if (import.meta.client && !hasRelease.value && !version.value) {
    $fetch<ReleaseData>('/api/release')
      .then(data => {
        if (!data?.version) return;
        version.value = data.version;
        platforms.value = data.platforms ?? {};
        hasRelease.value = Object.keys(data.platforms ?? {}).length > 0;
      })
      .catch(() => {});
  }

  function detectPlatform(): string {
    if (import.meta.server) return 'windows';
    const ua = navigator.userAgent.toLowerCase();
    if (ua.includes('mac')) return 'mac_arm';
    if (ua.includes('linux')) return 'linux';
    return 'windows';
  }

  function triggerDownload(platform?: string) {
    const key = platform ?? detectPlatform();
    const dl = platforms.value[key];

    if (dl) {
      const link = document.createElement('a');
      link.href = dl.url;
      link.download = dl.filename;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
    }
  }

  return { version, platforms, hasRelease, detectPlatform, triggerDownload };
}
