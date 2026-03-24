import { ref, computed } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

export type UpdatePhase = 'idle' | 'available' | 'downloading' | 'ready' | 'error';

export interface UpdateInfo {
  version: string;
  body: string;
}

export function useUpdater() {
  const phase = ref<UpdatePhase>('idle');
  const updateInfo = ref<UpdateInfo | null>(null);
  const downloadedBytes = ref(0);
  const totalBytes = ref<number | null>(null);
  const errorMessage = ref<string | null>(null);

  const downloadPercent = computed(() => {
    if (!totalBytes.value) return 0;
    return Math.min((downloadedBytes.value / totalBytes.value) * 100, 100);
  });

  async function setupListeners(): Promise<void> {
    await listen<UpdateInfo>('update-available', event => {
      updateInfo.value = event.payload;
      phase.value = 'available';
    });

    await listen<{ downloaded: number; total: number | null }>('update-progress', event => {
      phase.value = 'downloading';
      downloadedBytes.value = event.payload.downloaded;
      totalBytes.value = event.payload.total;
    });

    await listen('update-ready', () => {
      phase.value = 'ready';
    });

    await listen<{ message: string }>('update-error', event => {
      errorMessage.value = event.payload.message;
      phase.value = 'error';
    });
  }

  async function checkForUpdate(): Promise<void> {
    try {
      await invoke('check_for_update');
    } catch (err) {
      console.error('Update check failed:', err);
    }
  }

  async function installUpdate(): Promise<void> {
    phase.value = 'downloading';
    try {
      await invoke('download_and_install_update');
    } catch (err) {
      errorMessage.value = String(err);
      phase.value = 'error';
    }
  }

  function dismiss(): void {
    phase.value = 'idle';
    updateInfo.value = null;
    errorMessage.value = null;
  }

  return {
    phase,
    updateInfo,
    downloadedBytes,
    totalBytes,
    downloadPercent,
    errorMessage,
    setupListeners,
    checkForUpdate,
    installUpdate,
    dismiss,
  };
}
