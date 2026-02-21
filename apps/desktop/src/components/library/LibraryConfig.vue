<script setup lang="ts">
import { useLibraryStore } from '@/stores/library';
import { useLibraryConfigStore } from '@/stores/libraryConfig';
import { useLibraryActions } from '@/composables/useLibraryActions';

const libraryStore = useLibraryStore();
const configStore = useLibraryConfigStore();
const { generateLibrary, pickOutputDir } = useLibraryActions();
</script>

<template>
  <section class="config-section">
    <div class="section-header">
      <div>
        <h2 class="section-title">Library Output</h2>
        <p class="section-description">Configure how CrAIte organizes your samples</p>
      </div>
      <UButton
        color="primary"
        variant="solid"
        size="sm"
        icon="i-lucide-link"
        :loading="configStore.isGenerating"
        :disabled="!configStore.hasOutputDir"
        @click="generateLibrary"
      >
        Generate Library
      </UButton>
    </div>

    <div class="config-rows">
      <div class="config-row">
        <span class="config-label">Output directory</span>
        <div class="config-value">
          <span v-if="configStore.outputDir" class="path-display">
            {{ configStore.outputDir }}
          </span>
          <span v-else class="placeholder">No directory selected</span>
          <UButton
            icon="i-lucide-folder"
            color="neutral"
            variant="outline"
            size="xs"
            @click="pickOutputDir"
          >
            {{ configStore.outputDir ? 'Change' : 'Choose' }}
          </UButton>
        </div>
      </div>

      <div class="config-row">
        <span class="config-label">Include categories</span>
        <div class="category-toggles">
          <label
            v-for="cat in libraryStore.categories"
            :key="cat.name"
            class="category-toggle"
          >
            <UCheckbox
              :model-value="!configStore.excludedCategories.has(cat.name)"
              color="primary"
              @update:model-value="configStore.toggleCategory(cat.name)"
            />
            <span class="dot" :style="{ background: cat.color }" />
            <span class="toggle-label">{{ cat.name }}</span>
          </label>
        </div>
      </div>
    </div>

    <p v-if="configStore.lastGeneratedAt" class="config-footer">
      Last generated: {{ configStore.lastGeneratedAt }}
      — {{ configStore.linkedCount.toLocaleString() }} files linked
    </p>
  </section>
</template>

<style scoped>
.config-section {
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

.config-rows {
  display: flex;
  flex-direction: column;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.config-row {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  padding: var(--space-md) var(--space-lg);
  background: var(--color-surface);
}

.config-row + .config-row {
  border-top: 1px solid var(--color-border);
}

.config-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-muted);
  min-width: 140px;
  flex-shrink: 0;
}

.config-value {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  flex: 1;
  min-width: 0;
}

.path-display {
  font-size: 13px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.placeholder {
  font-size: 13px;
  color: var(--color-text-muted);
  flex: 1;
}

.category-toggles {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-md);
}

.category-toggle {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
  cursor: pointer;
  text-transform: capitalize;
}

.dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.toggle-label {
  font-size: 13px;
}

.config-footer {
  font-size: 12px;
  color: var(--color-text-muted);
}
</style>
