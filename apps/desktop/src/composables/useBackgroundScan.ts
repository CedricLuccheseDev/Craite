import { ref, onMounted } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useLibraryStore } from '@/stores/library';
import { useLibraryConfigStore } from '@/stores/libraryConfig';
import { useTauri } from '@/composables/useTauri';
import { buildCategoriesFromSamples } from '@/utils/categoryBuilder';

interface BackgroundScanResult {
  totalFiles: number;
  sourceCount: number;
  linkedCount: number;
}

export function useBackgroundScan() {
  const enabled = ref(true);
  const intervalMinutes = ref(30);
  const isScanning = ref(false);

  const tauri = useTauri();
  const libraryStore = useLibraryStore();
  const configStore = useLibraryConfigStore();

  async function loadStatus() {
    try {
      const [isEnabled, interval, scanning] = await invoke<[boolean, number, boolean]>(
        'get_background_scan_status',
      );
      enabled.value = isEnabled;
      intervalMinutes.value = interval;
      isScanning.value = scanning;
    } catch (error) {
      console.error('Failed to load background scan status:', error);
    }
  }

  async function toggleEnabled() {
    const newValue = !enabled.value;
    try {
      await invoke('set_background_scan_enabled', { enabled: newValue });
      enabled.value = newValue;
    } catch (error) {
      console.error('Failed to toggle background scan:', error);
    }
  }

  async function updateInterval(minutes: number) {
    try {
      await invoke('set_scan_interval', { minutes });
      intervalMinutes.value = minutes;
    } catch (error) {
      console.error('Failed to update scan interval:', error);
    }
  }

  async function setupListeners(): Promise<UnlistenFn[]> {
    const unlisteners: UnlistenFn[] = [];

    unlisteners.push(
      await listen('background-scan-started', () => {
        isScanning.value = true;
      }),
    );

    unlisteners.push(
      await listen<BackgroundScanResult>('background-scan-completed', async (event) => {
        isScanning.value = false;
        try {
          const samples = await tauri.loadSamples();
          libraryStore.setSamples(samples);
          libraryStore.setCategories(buildCategoriesFromSamples(samples));

          if (event.payload.linkedCount > 0) {
            configStore.setGenerationResult(event.payload.linkedCount);
          }
        } catch (error) {
          console.error('Failed to reload after background scan:', error);
        }
      }),
    );

    unlisteners.push(
      await listen('background-scan-error', () => {
        isScanning.value = false;
      }),
    );

    return unlisteners;
  }

  onMounted(loadStatus);

  return { enabled, intervalMinutes, isScanning, toggleEnabled, updateInterval, setupListeners };
}
