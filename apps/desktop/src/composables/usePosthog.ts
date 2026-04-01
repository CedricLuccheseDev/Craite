import posthog from 'posthog-js';
import { useTauri } from '@/composables/useTauri';

const POSTHOG_KEY = import.meta.env.VITE_POSTHOG_KEY || '';
const POSTHOG_HOST = import.meta.env.VITE_POSTHOG_HOST || 'https://us.i.posthog.com';
const SETTING_KEY = 'tracking_id';
const ENV = import.meta.env.DEV ? 'dev' : 'production';

let initialized = false;

export function usePosthog() {
  const tauri = useTauri();

  function init() {
    if (initialized || !POSTHOG_KEY) return;
    posthog.init(POSTHOG_KEY, {
      api_host: POSTHOG_HOST,
      capture_pageview: false,
      persistence: 'memory',
      autocapture: false,
    });
    posthog.register({ env: ENV, app: 'desktop' });
    initialized = true;
  }

  async function identify(trackingId: string) {
    init();
    if (!trackingId) return;
    posthog.identify(trackingId);
    await tauri.saveSetting(SETTING_KEY, trackingId);
  }

  async function loadTrackingId(): Promise<string> {
    const settings = await tauri.loadAllSettings();
    const map = new Map(settings);
    return map.get(SETTING_KEY) ?? '';
  }

  function track(event: string, properties?: Record<string, unknown>) {
    init();
    if (!POSTHOG_KEY) return;
    posthog.capture(event, properties);
  }

  return { init, identify, loadTrackingId, track };
}
