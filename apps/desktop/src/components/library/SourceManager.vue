<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { listen } from '@tauri-apps/api/event';
import { useTauri } from '@/composables/useTauri';
import { useScanStore } from '@/stores/scan';
import { useLibraryStore } from '@/stores/library';
import { useLibraryActions } from '@/composables/useLibraryActions';
import { useNotify } from '@/composables/useNotify';
import { useBackgroundScan } from '@/composables/useBackgroundScan';

const { t } = useI18n();
const tauri = useTauri();
const notify = useNotify();
const scanStore = useScanStore();
const libraryStore = useLibraryStore();
const { reloadLibrary, rescan, generateLibrary, addSourceFolder } = useLibraryActions();
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

function formatCountdown(seconds: number): string {
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return m > 0 ? `${m}m ${s.toString().padStart(2, '0')}s` : `${s}s`;
}

function openInExplorer(path: string) {
  tauri.openFolder(path);
}

function removeSource(path: string) {
  scanStore.removeSource(path);
  syncSamples();
}

const isSyncing = ref(false);

const enabledCount = () => scanStore.sources.filter(s => s.enabled).length;
const totalSamples = () => scanStore.sources.reduce((sum, s) => sum + s.sampleCount, 0);

let syncTimer: ReturnType<typeof setTimeout> | null = null;

function toggleWithSync(path: string) {
  scanStore.toggleSource(path);
  scheduleSyncSamples();
}

function scheduleSyncSamples() {
  if (syncTimer) clearTimeout(syncTimer);
  isSyncing.value = true;
  syncTimer = setTimeout(() => syncSamples(), 600);
}

async function syncSamples() {
  try {
    const result = await tauri.scanDirectories(scanStore.sources);
    scanStore.setScanResult(result);
    scanStore.updateSourceCounts(result.samples);
    libraryStore.setSamples(result.samples);
    libraryStore.setCategories(result.categories);
    await generateLibrary();
  } catch (error) {
    console.error('Failed to sync samples:', error);
    notify.error('notify.scanFailed');
  } finally {
    isSyncing.value = false;
  }
}

onUnmounted(() => {
  if (syncTimer) clearTimeout(syncTimer);
  if (countdownTimer) clearInterval(countdownTimer);
  unlistenSkip?.();
  unlistenStarted?.();
  unlistenCompleted?.();
});
</script>

<template>
  <section class="h-full flex flex-col gap-8 rounded-2xl p-9 overflow-hidden relative z-1">
    <!-- Header — fixed -->
    <div class="flex items-start justify-between shrink-0">
      <div>
        <h2 class="text-[22px] font-bold tracking-tight">
          {{ t('sources.title') }}
        </h2>
        <p class="text-sm text-muted mt-1.5">
          {{ t('sources.description') }}
        </p>
      </div>
      <UButton
        :icon="scanStore.sources.length === 0 ? 'i-lucide-scan' : 'i-lucide-refresh-cw'"
        color="primary"
        variant="solid"
        size="sm"
        :loading="scanStore.isScanning"
        @click="rescan"
      >
        {{ scanStore.sources.length === 0 ? t('sources.autoDetect') : t('sources.rescan') }}
      </UButton>
    </div>

    <!-- Summary bar — fixed -->
    <div v-if="scanStore.sources.length > 0" class="flex items-center gap-6 text-xs text-muted shrink-0">
      <span class="flex items-center gap-1.5">
        <UIcon name="i-lucide-hard-drive" class="text-[14px]" />
        {{ t('sources.sourceCount', { active: enabledCount(), total: scanStore.sources.length }) }}
      </span>
      <span v-if="isSyncing" class="flex items-center gap-1.5 text-orange-500">
        <UIcon name="i-lucide-loader-2" class="animate-spin text-[14px]" />
        {{ t('export.syncing') }}
      </span>
      <span v-else class="flex items-center gap-1.5">
        <UIcon name="i-lucide-music-2" class="text-[14px]" />
        {{ totalSamples().toLocaleString() }} {{ t('stats.samples').toLowerCase() }}
      </span>
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
    </div>

    <!-- Source list — scrollable -->
    <div class="flex flex-col gap-3 flex-1 min-h-0 overflow-y-auto scrollbar-thin">
      <div
        v-for="source in scanStore.sources"
        :key="source.path"
        class="flex items-center justify-between gap-4 py-4 px-5 rounded-xl border border-zinc-800 bg-zinc-950 transition-all duration-150 hover:border-zinc-700 cursor-pointer"
        :class="{ 'opacity-40': !source.enabled }"
        @click="openInExplorer(source.path)"
      >
        <div class="flex items-center gap-4 flex-1 min-w-0">
          <USwitch
            :model-value="source.enabled"
            color="primary"
            size="sm"
            @click.stop
            @update:model-value="toggleWithSync(source.path)"
          />
          <div class="flex items-center gap-3 flex-1 min-w-0">
            <UIcon name="i-lucide-folder" class="text-[18px] text-muted shrink-0" />
            <div class="flex flex-col min-w-0">
              <span class="text-[13px] font-semibold">{{ source.label }}</span>
              <span class="text-[11px] text-muted truncate">{{ source.path }}</span>
            </div>
          </div>
        </div>
        <div class="flex items-center gap-3 shrink-0">
          <span class="text-xs tabular-nums text-muted font-mono">
            {{ source.sampleCount.toLocaleString() }}
          </span>
          <UButton
            icon="i-lucide-trash-2"
            color="neutral"
            variant="ghost"
            size="2xs"
            class="text-muted hover:text-red-400"
            @click.stop="removeSource(source.path)"
          />
        </div>
      </div>

      <!-- Add source dropzone -->
      <button
        v-if="!scanStore.isScanning && scanStore.sources.length > 0"
        class="flex items-center justify-center gap-2 py-4 px-5 rounded-xl border-2 border-dashed border-zinc-800 transition-all duration-150 hover:border-zinc-600 hover:bg-white/2"
        @click="addSourceFolder"
      >
        <UIcon name="i-lucide-plus" class="text-[16px] text-muted" />
        <span class="text-[12px] text-muted">{{ t('sources.addFolder') }}</span>
      </button>

      <!-- Empty state -->
      <div
        v-if="!scanStore.isScanning && scanStore.sources.length === 0"
        class="flex flex-col items-center gap-2.5 py-12 px-6 rounded-xl border-2 border-dashed border-zinc-700 text-center"
      >
        <div class="flex items-center justify-center size-12 rounded-2xl bg-zinc-800">
          <UIcon name="i-lucide-folder-plus" class="text-[22px] text-muted" />
        </div>
        <p class="text-sm font-medium text-white/80">{{ t('sources.noSources') }}</p>
        <p class="text-[12px] text-muted">{{ t('sources.noSourcesHint') }}</p>
        <div class="flex gap-2 mt-2">
          <UButton icon="i-lucide-scan" color="primary" variant="solid" size="sm" @click="rescan">
            {{ t('sources.autoDetect') }}
          </UButton>
          <UButton icon="i-lucide-folder-plus" color="neutral" variant="outline" size="sm" @click="addSourceFolder">
            {{ t('sources.addFolder') }}
          </UButton>
        </div>
      </div>
    </div>
  </section>
</template>
