import { ref, onMounted, onUnmounted } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useScanStore } from '@/stores/scan';
import { useLibraryActions } from '@/composables/useLibraryActions';

interface SamplesChangedPayload {
  paths: string[];
}

export function useWatcher() {
  const isWatching = ref(false);
  let unlisten: UnlistenFn | null = null;

  async function startWatching() {
    const scanStore = useScanStore();
    const paths = scanStore.sources.filter(s => s.enabled).map(s => s.path);

    if (paths.length === 0) return;

    try {
      await invoke('start_watching', { paths });
      isWatching.value = true;
    } catch (error) {
      console.error('Failed to start watcher:', error);
    }
  }

  async function stopWatching() {
    await invoke('stop_watching');
    isWatching.value = false;
  }

  async function setupListener() {
    // Clean up existing listener before creating a new one
    unlisten?.();

    const { rescan } = useLibraryActions();

    unlisten = await listen<SamplesChangedPayload>('samples-changed', () => {
      rescan();
    });
  }

  onMounted(async () => {
    await setupListener();
    await startWatching();
  });

  onUnmounted(async () => {
    unlisten?.();
    await stopWatching();
  });

  return { isWatching, startWatching, stopWatching };
}
