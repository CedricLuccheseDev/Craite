export function useTrack() {
  function track(event: string, properties?: Record<string, string>) {
    const { $posthog } = useNuxtApp();
    $posthog?.capture(event, properties);
  }

  function trackSectionView(section: string) {
    const tracked = new Set<string>();
    return () => {
      if (tracked.has(section)) return;
      tracked.add(section);
      track('section_viewed', { section });
    };
  }

  return { track, trackSectionView };
}
