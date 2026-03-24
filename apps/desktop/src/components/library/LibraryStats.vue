<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useLibraryStore } from '@/stores/library';
import { useScanStore } from '@/stores/scan';
import { useLibraryConfigStore } from '@/stores/libraryConfig';

const { t } = useI18n();
const libraryStore = useLibraryStore();
const scanStore = useScanStore();
const configStore = useLibraryConfigStore();

const stats = computed(() => [
  { label: t('stats.samples'), value: libraryStore.sampleCount.toLocaleString(), icon: 'i-lucide-music' },
  { label: t('stats.categories'), value: libraryStore.categories.length.toString(), icon: 'i-lucide-folder-tree' },
  { label: t('stats.sources'), value: scanStore.sources.length.toString(), icon: 'i-lucide-hard-drive' },
  { label: t('stats.linked'), value: configStore.linkedCount.toLocaleString(), icon: 'i-lucide-link' },
]);
</script>

<template>
  <div class="flex gap-6">
    <div v-for="stat in stats" :key="stat.label" class="flex items-center gap-1">
      <UIcon :name="stat.icon" class="text-sm text-muted" />
      <span class="text-[15px] font-bold tabular-nums">{{ stat.value }}</span>
      <span class="text-xs text-muted">{{ stat.label }}</span>
    </div>
  </div>
</template>
