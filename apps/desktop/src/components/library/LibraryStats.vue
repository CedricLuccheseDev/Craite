<script setup lang="ts">
import { computed } from 'vue';
import { useLibraryStore } from '@/stores/library';
import { useScanStore } from '@/stores/scan';
import { useLibraryConfigStore } from '@/stores/libraryConfig';

const libraryStore = useLibraryStore();
const scanStore = useScanStore();
const configStore = useLibraryConfigStore();

const stats = computed(() => [
  { label: 'Total Samples', value: libraryStore.sampleCount.toLocaleString(), icon: 'i-lucide-music' },
  { label: 'Categories', value: libraryStore.categories.length.toString(), icon: 'i-lucide-folder-tree' },
  { label: 'Sources', value: scanStore.sources.length.toString(), icon: 'i-lucide-hard-drive' },
  { label: 'Linked Files', value: configStore.linkedCount.toLocaleString(), icon: 'i-lucide-link' },
]);
</script>

<template>
  <section class="stats-section">
    <div
      v-for="stat in stats"
      :key="stat.label"
      class="stat-card"
    >
      <div class="stat-icon">
        <UIcon :name="stat.icon" />
      </div>
      <span class="stat-value">{{ stat.value }}</span>
      <span class="stat-label">{{ stat.label }}</span>
    </div>
  </section>
</template>

<style scoped>
.stats-section {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: var(--space-md);
}

.stat-card {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--space-xs);
  padding: var(--space-lg);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
}

.stat-icon {
  font-size: 18px;
  color: var(--color-text-muted);
  margin-bottom: var(--space-xs);
}

.stat-value {
  font-size: 32px;
  font-weight: 700;
  font-variant-numeric: tabular-nums;
  line-height: 1;
}

.stat-label {
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--color-text-muted);
}
</style>
