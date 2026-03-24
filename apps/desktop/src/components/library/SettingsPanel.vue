<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useSettingsStore } from '@/stores/settings';
import { useAutostart } from '@/composables/useAutostart';
import { useBackgroundScan } from '@/composables/useBackgroundScan';
import type { SupportedLocale } from '@/plugins/i18n';

const { t } = useI18n();
const settingsStore = useSettingsStore();
const { autostartEnabled, loading: autostartLoading, toggle: toggleAutostart } = useAutostart();
const { enabled: bgEnabled, intervalMinutes, isScanning, toggleEnabled, updateInterval } = useBackgroundScan();

const languageOptions = [
  { label: 'English', value: 'en' },
  { label: 'Français', value: 'fr' },
];

const intervalOptions = computed(() => [
  { label: t('settings.interval5'), value: 5 },
  { label: t('settings.interval15'), value: 15 },
  { label: t('settings.interval30'), value: 30 },
  { label: t('settings.interval60'), value: 60 },
  { label: t('settings.interval120'), value: 120 },
  { label: t('settings.interval240'), value: 240 },
  { label: t('settings.interval480'), value: 480 },
]);

function onLocaleChange(value: string) {
  settingsStore.setLocale(value as SupportedLocale);
}
</script>

<template>
  <section class="h-full flex flex-col gap-6 bg-surface rounded-2xl p-8 overflow-y-auto">
    <div class="shrink-0">
      <h2 class="text-xl font-bold tracking-tight">
        {{ t('settings.title') }}
      </h2>
      <p class="text-[13px] text-muted mt-1">
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
          @update:model-value="toggleAutostart"
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
        <USwitch :model-value="bgEnabled" color="primary" @update:model-value="toggleEnabled" />
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
          @update:model-value="updateInterval"
        />
      </div>
    </div>

    <p class="text-xs text-muted mt-auto">
      {{ t('settings.footer') }}
    </p>
  </section>
</template>
