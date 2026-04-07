<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { useSettingsStore } from '@/stores/settings';
import { useAutostart } from '@/composables/useAutostart';
import { useBackgroundScan } from '@/composables/useBackgroundScan';
import { resetOnboarding } from '@/composables/useOnboarding';
import { useTauri } from '@/composables/useTauri';
import { useNotify } from '@/composables/useNotify';
import { usePosthog } from '@/composables/usePosthog';
import { useScanStore } from '@/stores/scan';
import { useLibraryStore } from '@/stores/library';
import type { SupportedLocale } from '@/plugins/i18n';

const { t } = useI18n();
const router = useRouter();
const settingsStore = useSettingsStore();
const tauri = useTauri();
const notify = useNotify();
const ph = usePosthog();
const scanStore = useScanStore();

const currentTrackingId = ref('');

onMounted(async () => {
  currentTrackingId.value = await ph.loadTrackingId();
});
const libraryStore = useLibraryStore();
const { autostartEnabled, loading: autostartLoading, toggle: toggleAutostart } = useAutostart();
const { enabled: bgEnabled, intervalMinutes, isScanning, toggleEnabled, updateInterval } = useBackgroundScan();

const isDev = import.meta.env.DEV;
const isResetting = ref(false);
const isClearingCache = ref(false);

const languageOptions = [
  { label: 'English', value: 'en' },
  { label: 'Français', value: 'fr' },
];

const intervalOptions = computed(() => {
  const options = [
    { label: t('settings.interval5'), value: 5 },
    { label: t('settings.interval15'), value: 15 },
    { label: t('settings.interval30'), value: 30 },
    { label: t('settings.interval60'), value: 60 },
    { label: t('settings.interval120'), value: 120 },
    { label: t('settings.interval240'), value: 240 },
    { label: t('settings.interval480'), value: 480 },
  ];
  if (isDev) {
    options.unshift({ label: '10s (dev)', value: 0 });
  }
  return options;
});

function onLocaleChange(value: string) {
  settingsStore.setLocale(value as SupportedLocale);
  notify.success('notify.settingSaved');
}

async function onToggleAutostart() {
  await toggleAutostart();
  notify.success('notify.settingSaved');
}

function onToggleBgScan(value: boolean) {
  toggleEnabled(value);
  notify.success('notify.settingSaved');
}

function onUpdateInterval(value: number) {
  updateInterval(value);
  notify.success('notify.settingSaved');
}

async function restartOnboarding() {
  isResetting.value = true;
  try {
    await tauri.resetApp();
  } catch (error) {
    console.error('Failed to reset app:', error);
    notify.error('notify.resetFailed');
  }
  scanStore.setDetectedSources([], false);
  libraryStore.setSamples([]);
  libraryStore.setCategories([]);
  resetOnboarding();
  isResetting.value = false;
  router.push('/');
}

async function clearCache() {
  isClearingCache.value = true;
  try {
    await tauri.resetApp();
  } catch (error) {
    console.error('Failed to clear cache:', error);
    notify.error('notify.clearCacheFailed');
  }
  scanStore.setDetectedSources([], false);
  libraryStore.setSamples([]);
  libraryStore.setCategories([]);
  isClearingCache.value = false;
}
</script>

<template>
  <section class="h-full flex flex-col gap-8 rounded-2xl p-9 overflow-y-auto relative z-1">
    <div class="shrink-0">
      <h2 class="text-[22px] font-bold tracking-tight">
        {{ t('settings.title') }}
      </h2>
      <p class="text-sm text-muted mt-1.5">
        {{ t('settings.description') }}
      </p>
    </div>

    <div class="flex flex-col gap-4">
      <div class="flex items-center justify-between gap-6 py-4 px-6 bg-zinc-950 rounded-[10px]">
        <div class="flex flex-col gap-0.5">
          <span class="text-[13px] font-semibold">
            {{ t('settings.language') }}
          </span>
          <span class="text-xs text-muted flex items-center gap-1">
            {{ t('settings.languageDescription') }}
          </span>
        </div>
        <USelect
          :model-value="settingsStore.locale"
          :items="languageOptions"
          value-key="value"
          label-key="label"
          color="neutral"
          variant="outline"
          size="sm"
          class="w-40"
          @update:model-value="onLocaleChange"
        />
      </div>

      <div class="flex items-center justify-between gap-6 py-4 px-6 bg-zinc-950 rounded-[10px]">
        <div class="flex flex-col gap-0.5">
          <span class="text-[13px] font-semibold">
            {{ t('settings.autostart') }}
          </span>
          <span class="text-xs text-muted flex items-center gap-1">
            {{ t('settings.autostartDescription') }}
          </span>
        </div>
        <USwitch
          :model-value="autostartEnabled"
          :loading="autostartLoading"
          color="primary"
          @update:model-value="onToggleAutostart"
        />
      </div>

      <div class="flex items-center justify-between gap-6 py-4 px-6 bg-zinc-950 rounded-[10px]">
        <div class="flex flex-col gap-0.5">
          <span class="text-[13px] font-semibold">
            {{ t('settings.backgroundScan') }}
          </span>
          <span class="text-xs text-muted flex items-center gap-1">
            {{ t('settings.backgroundScanDescription') }}
            <span v-if="isScanning" class="text-[11px] text-orange-500 font-medium">
              {{ t('settings.scanning') }}
            </span>
          </span>
        </div>
        <USwitch :model-value="bgEnabled" color="primary" @update:model-value="onToggleBgScan" />
      </div>

      <div class="flex items-center justify-between gap-6 py-4 px-6 bg-zinc-950 rounded-[10px]">
        <div class="flex flex-col gap-0.5">
          <span class="text-[13px] font-semibold">
            {{ t('settings.scanInterval') }}
          </span>
          <span class="text-xs text-muted flex items-center gap-1">
            {{ t('settings.scanIntervalDescription') }}
          </span>
        </div>
        <USelect
          :model-value="intervalMinutes"
          :items="intervalOptions"
          value-key="value"
          label-key="label"
          color="neutral"
          variant="outline"
          size="sm"
          :disabled="!bgEnabled"
          class="w-40"
          @update:model-value="onUpdateInterval"
        />
      </div>
    </div>

    <!-- Dev tools -->
    <template v-if="isDev">
      <div class="flex flex-col gap-1 mt-4">
        <span class="text-[11px] font-semibold uppercase tracking-[0.08em] text-orange-500/70 px-1">
          {{ t('settings.devSection') }}
        </span>
      </div>

      <div class="flex flex-col gap-4">
        <div
          class="flex items-center justify-between gap-6 py-4 px-6 bg-zinc-950 rounded-[10px] border border-orange-500/20"
        >
          <div class="flex flex-col gap-0.5">
            <span class="text-[13px] font-semibold">
              {{ t('settings.restartOnboarding') }}
            </span>
            <span class="text-xs text-muted">
              {{ t('settings.restartOnboardingDescription') }}
            </span>
          </div>
          <UButton
            icon="i-lucide-rotate-ccw"
            color="neutral"
            variant="outline"
            size="sm"
            :loading="isResetting"
            @click="restartOnboarding"
          >
            {{ t('settings.restart') }}
          </UButton>
        </div>

        <div
          class="flex items-center justify-between gap-6 py-4 px-6 bg-zinc-950 rounded-[10px] border border-orange-500/20"
        >
          <div class="flex flex-col gap-0.5">
            <span class="text-[13px] font-semibold">
              {{ t('settings.clearCache') }}
            </span>
            <span class="text-xs text-muted">
              {{ t('settings.clearCacheDescription') }}
            </span>
          </div>
          <UButton
            icon="i-lucide-trash-2"
            color="error"
            variant="outline"
            size="sm"
            :loading="isClearingCache"
            @click="clearCache"
          >
            {{ t('settings.clear') }}
          </UButton>
        </div>
        <div
          v-if="currentTrackingId"
          class="flex items-center justify-between gap-6 py-4 px-6 bg-zinc-950 rounded-[10px] border border-orange-500/20"
        >
          <div class="flex flex-col gap-0.5">
            <span class="text-[13px] font-semibold">Tracking ID</span>
            <span class="text-xs text-muted font-mono">{{ currentTrackingId }}</span>
          </div>
          <UIcon name="i-lucide-link" class="text-emerald-500" />
        </div>
      </div>
    </template>

    <p class="text-xs text-muted mt-auto">
      {{ t('settings.footer') }}
    </p>
  </section>
</template>
