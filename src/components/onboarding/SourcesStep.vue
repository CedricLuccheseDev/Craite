<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import type { SourceFolder } from '@/types/onboarding';

interface Props {
  sources: SourceFolder[];
}

defineProps<Props>();

const emit = defineEmits<{
  toggle: [path: string];
  addSource: [source: SourceFolder];
}>();

async function addCustomFolder() {
  const selected = await open({ directory: true, multiple: false });
  if (selected) {
    emit('addSource', {
      path: selected as string,
      label: (selected as string).split(/[/\\]/).pop() ?? 'Custom',
      enabled: true,
      type: 'custom',
      sampleCount: 0,
    });
  }
}
</script>

<template>
  <div class="sources-step slide-up">
    <h2>Sample Sources</h2>
    <p class="description">Select the folders to scan for samples</p>

    <div class="source-list">
      <div
        v-for="(source, index) in sources"
        :key="source.path"
        class="source-item stagger-item"
        :style="{ animationDelay: `${index * 80}ms` }"
      >
        <label class="source-toggle">
          <input
            type="checkbox"
            :checked="source.enabled"
            @change="emit('toggle', source.path)"
          >
          <div class="source-info">
            <span class="source-label">{{ source.label }}</span>
            <span class="source-path">{{ source.path }}</span>
          </div>
          <span class="source-badge">{{ source.type }}</span>
        </label>
      </div>
    </div>

    <button
      class="add-folder-btn"
      @click="addCustomFolder"
    >
      + Add folder
    </button>
  </div>
</template>

<style scoped>
.sources-step {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-lg);
  width: 100%;
  max-width: 500px;
  z-index: 1;
}

h2 {
  font-size: 28px;
  font-weight: 700;
}

.description {
  color: var(--color-text-muted);
}

.source-list {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.source-item {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--space-md);
  transition: border-color var(--duration-fast);
}

.source-item:hover {
  border-color: var(--color-accent-orange);
}

.source-toggle {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  cursor: pointer;
}

.source-info {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-width: 0;
}

.source-label {
  font-weight: 600;
}

.source-path {
  font-size: 12px;
  color: var(--color-text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.source-badge {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: var(--radius-full);
  background: var(--color-border);
  color: var(--color-text-muted);
  text-transform: uppercase;
}

.add-folder-btn {
  padding: var(--space-sm) var(--space-lg);
  background: transparent;
  color: var(--color-accent-orange);
  border: 1px dashed var(--color-accent-orange);
  border-radius: var(--radius-md);
  font-size: 14px;
  cursor: pointer;
  transition: background var(--duration-fast);
}

.add-folder-btn:hover {
  background: rgba(255, 107, 53, 0.1);
}
</style>
