<script setup lang="ts">
import { useScanStore } from '@/stores/scan';
import { useLibraryActions } from '@/composables/useLibraryActions';

const scanStore = useScanStore();
const { rescan, addSourceFolder } = useLibraryActions();
</script>

<template>
  <section class="source-section">
    <div class="section-header">
      <div>
        <h2 class="section-title">Source Folders</h2>
        <p class="section-description">Directories CrAIte scans for audio samples</p>
      </div>
      <div class="section-actions">
        <UButton
          icon="i-lucide-folder-plus"
          color="neutral"
          variant="outline"
          size="sm"
          @click="addSourceFolder"
        >
          Add folder
        </UButton>
        <UButton
          icon="i-lucide-refresh-cw"
          color="primary"
          variant="solid"
          size="sm"
          :loading="scanStore.isScanning"
          @click="rescan"
        >
          Rescan
        </UButton>
      </div>
    </div>

    <div class="source-list">
      <div
        v-for="source in scanStore.sources"
        :key="source.path"
        class="source-row"
      >
        <UCheckbox
          :model-value="source.enabled"
          color="primary"
          @update:model-value="scanStore.toggleSource(source.path)"
        />
        <div class="source-info">
          <span class="source-label">{{ source.label }}</span>
          <span class="source-path">{{ source.path }}</span>
        </div>
        <UBadge :label="source.type" color="neutral" variant="subtle" size="xs" />
        <span class="source-count">{{ source.sampleCount.toLocaleString() }}</span>
      </div>

      <p v-if="scanStore.sources.length === 0" class="empty">
        No source folders configured
      </p>
    </div>
  </section>
</template>

<style scoped>
.source-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
}

.section-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
}

.section-title {
  font-size: 20px;
  font-weight: 700;
  letter-spacing: -0.3px;
}

.section-description {
  font-size: 13px;
  color: var(--color-text-muted);
  margin-top: var(--space-xs);
}

.section-actions {
  display: flex;
  gap: var(--space-sm);
}

.source-list {
  display: flex;
  flex-direction: column;
  gap: 1px;
  background: var(--color-border);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.source-row {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface);
  transition: background var(--duration-fast);
}

.source-row:hover {
  background: var(--color-surface-hover);
}

.source-info {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-width: 0;
}

.source-label {
  font-size: 13px;
  font-weight: 600;
}

.source-path {
  font-size: 11px;
  color: var(--color-text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.source-count {
  font-size: 12px;
  font-variant-numeric: tabular-nums;
  color: var(--color-text-muted);
}

.empty {
  padding: var(--space-xl);
  text-align: center;
  color: var(--color-text-muted);
  background: var(--color-surface);
}
</style>
