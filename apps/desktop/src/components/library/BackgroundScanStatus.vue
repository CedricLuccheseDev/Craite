<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { listen } from '@tauri-apps/api/event';
import { useLibraryStore } from '@/stores/library';
import { useLibraryActions } from '@/composables/useLibraryActions';
import { useNotify } from '@/composables/useNotify';
import { useBackgroundScan } from '@/composables/useBackgroundScan';

const { t } = useI18n();
const notify = useNotify();
const libraryStore = useLibraryStore();
const { reloadLibrary } = useLibraryActions();
const {
  enabled: bgScanEnabled,
  isScanning: bgIsScanning,
  loadStatus: loadBgStatus,
  intervalMinutes,
} = useBackgroundScan();

const countdown = ref<number | null>(null);
let countdownTimer: ReturnType<typeof setInterval> | null = null;
let unlistenSkip: (() => void) | null = null;
let unlistenStarted: (() => void) | null = null;
let unlistenCompleted: (() => void) | null = null;

function getIntervalSecs() {
  return intervalMinutes.value === 0 ? 10 : intervalMinutes.value * 60;
}

function formatCountdown(seconds: number): string {
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return m > 0 ? `${m}m ${s.toString().padStart(2, '0')}s` : `${s}s`;
}

onMounted(async () => {
  await loadBgStatus();

  if (bgScanEnabled.value) {
    countdown.value = getIntervalSecs();
    countdownTimer = setInterval(() => {
      if (countdown.value !== null && countdown.value > 0) {
        countdown.value--;
      }
    }, 1000);
  }

  unlistenSkip = await listen('background-scan-skipped', () => {
    countdown.value = getIntervalSecs();
  });

  unlistenStarted = await listen('background-scan-started', () => {
    countdown.value = null;
  });

  unlistenCompleted = await listen('background-scan-completed', async () => {
    countdown.value = getIntervalSecs();
    const previousCount = libraryStore.samples.length;
    try {
      const samples = await reloadLibrary();
      const diff = samples.length - previousCount;
      if (diff > 0) {
        notify.success('notify.backgroundScanAdded', { added: diff, total: samples.length });
      } else if (diff < 0) {
        notify.success('notify.backgroundScanRemoved', { removed: Math.abs(diff), total: samples.length });
      } else {
        notify.success('notify.backgroundScanComplete', { count: samples.length });
      }
    } catch (e) {
      console.error('Failed to reload after background scan:', e);
    }
  });
});

onUnmounted(() => {
  if (countdownTimer) clearInterval(countdownTimer);
  unlistenSkip?.();
  unlistenStarted?.();
  unlistenCompleted?.();
});
</script>

<template>
  <span v-if="bgScanEnabled" class="flex items-center gap-1.5 ml-auto text-muted/50">
    <template v-if="bgIsScanning">
      <UIcon name="i-lucide-loader-2" class="animate-spin text-[14px] text-orange-500" />
      <span class="text-orange-500">{{ t('settings.scanning') }}</span>
    </template>
    <template v-else-if="countdown != null && countdown > 0">
      <UIcon name="i-lucide-timer" class="text-[14px]" />
      {{ formatCountdown(countdown) }}
    </template>
    <template v-else>
      <UIcon name="i-lucide-loader-2" class="animate-spin text-[14px] text-orange-500" />
      <span class="text-orange-500">{{ t('settings.scanning') }}</span>
    </template>
  </span>
</template>
