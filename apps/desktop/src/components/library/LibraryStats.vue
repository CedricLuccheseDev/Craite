<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useLibraryStore } from '@/stores/library';
import { useScanStore } from '@/stores/scan';

const { t } = useI18n();
const libraryStore = useLibraryStore();
const scanStore = useScanStore();

const stats = computed(() => [
  { label: t('stats.samples'), value: libraryStore.sampleCount.toLocaleString(), icon: 'i-lucide-music' },
  { label: t('stats.categories'), value: libraryStore.categories.length.toString(), icon: 'i-lucide-folder-tree' },
  { label: t('stats.sources'), value: scanStore.sources.length.toString(), icon: 'i-lucide-hard-drive' },
]);
</script>

<template>
  <div class="flex gap-7">
    <div v-for="stat in stats" :key="stat.label" class="flex items-center gap-1.5">
      <UIcon :name="stat.icon" class="text-[14px] text-muted" />
      <span class="text-[16px] font-bold tabular-nums">{{ stat.value }}</span>
      <span class="text-[13px] text-muted">{{ stat.label }}</span>
    </div>
  </div>
</template>
