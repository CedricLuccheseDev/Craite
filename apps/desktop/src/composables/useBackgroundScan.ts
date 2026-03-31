import { ref } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useLibraryStore } from '@/stores/library';
import { useLibraryConfigStore } from '@/stores/libraryConfig';
import { useTauri } from '@/composables/useTauri';
import { useNotify } from '@/composables/useNotify';
import { buildCategoriesFromSamples } from '@/utils/categoryBuilder';

interface BackgroundScanResult {
  totalFiles: number;
  sourceCount: number;
  linkedCount: number;
  skipped: boolean;
}

// Global singleton state
const enabled = ref(false);
const intervalMinutes = ref(30);
const isScanning = ref(false);
const secondsUntilNextScan = ref<number | null>(null);
let countdownHandle: ReturnType<typeof setInterval> | null = null;
let statusLoaded = false;

function getIntervalSecs() {
  return intervalMinutes.value === 0 ? 10 : intervalMinutes.value * 60;
}

function resetCountdown() {
  secondsUntilNextScan.value = getIntervalSecs();
}

function startCountdownLoop() {
  stopCountdownLoop();
  resetCountdown();
  countdownHandle = setInterval(() => {
    if (!enabled.value) {
      secondsUntilNextScan.value = null;
      return;
    }
    const cur = secondsUntilNextScan.value ?? 0;
    if (cur > 0) {
      secondsUntilNextScan.value = cur - 1;
    }
    // When it hits 0, just stay at 0 — the Rust scan event will reset it
  }, 1000);
}

function stopCountdownLoop() {
  if (countdownHandle) {
    clearInterval(countdownHandle);
    countdownHandle = null;
  }
  secondsUntilNextScan.value = null;
}

export function useBackgroundScan() {
  async function loadStatus() {
    if (statusLoaded) return;
    try {
      const [isEnabled, interval] = await invoke<[boolean, number, boolean]>('get_background_scan_status');
      statusLoaded = true;
      enabled.value = isEnabled;
      intervalMinutes.value = interval;
      if (enabled.value && !countdownHandle) {
        startCountdownLoop();
      }
    } catch (error) {
      console.error('Failed to load background scan status:', error);
    }
  }

  async function toggleEnabled(value?: boolean) {
    const newValue = value !== undefined ? value : !enabled.value;
    try {
      await invoke('set_background_scan_enabled', { enabled: newValue });
      enabled.value = newValue;
      if (newValue) {
        startCountdownLoop();
      } else {
        stopCountdownLoop();
      }
    } catch (error) {
      console.error('Failed to toggle background scan:', error);
    }
  }

  async function updateInterval(minutes: number) {
    try {
      await invoke('set_scan_interval', { minutes });
      intervalMinutes.value = minutes;
      if (enabled.value) resetCountdown();
    } catch (error) {
      console.error('Failed to update scan interval:', error);
    }
  }

  async function setupListeners(): Promise<UnlistenFn[]> {
    const tauri = useTauri();
    const notify = useNotify();
    const libraryStore = useLibraryStore();
    const configStore = useLibraryConfigStore();

    const unlisteners: UnlistenFn[] = [];

    unlisteners.push(
      await listen('background-scan-started', () => {
        isScanning.value = true;
      })
    );

    unlisteners.push(
      await listen<BackgroundScanResult>('background-scan-completed', async event => {
        isScanning.value = false;
        resetCountdown();

        if (event.payload.skipped) return;

        try {
          const samples = await tauri.loadSamples();
          libraryStore.setSamples(samples);
          libraryStore.setCategories(buildCategoriesFromSamples(samples));

          notify.success('notify.backgroundScanComplete', { count: event.payload.totalFiles });
          if (event.payload.linkedCount > 0) {
            configStore.setGenerationResult(event.payload.linkedCount);
          }
        } catch (error) {
          console.error('Failed to reload after background scan:', error);
        }
      })
    );

    unlisteners.push(
      await listen('background-scan-skipped', () => {
        resetCountdown();
      })
    );

    unlisteners.push(
      await listen('background-scan-error', () => {
        isScanning.value = false;
        resetCountdown();
      })
    );

    return unlisteners;
  }

  return {
    enabled,
    intervalMinutes,
    isScanning,
    secondsUntilNextScan,
    loadStatus,
    toggleEnabled,
    updateInterval,
    setupListeners,
  };
}
