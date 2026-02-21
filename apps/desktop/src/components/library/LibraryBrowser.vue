<script setup lang="ts">
import { useLibraryStore } from '@/stores/library';
import CategoryFilter from '@/components/library/CategoryFilter.vue';
import SampleList from '@/components/library/SampleList.vue';

const libraryStore = useLibraryStore();
</script>

<template>
  <section class="browser-section">
    <div class="section-header">
      <h2 class="section-title">Browse Samples</h2>
      <span class="result-count">
        {{ libraryStore.filteredSamples.length }} results
      </span>
    </div>

    <div class="browser-toolbar">
      <CategoryFilter
        :categories="libraryStore.categories"
        :selected="libraryStore.selectedCategory"
        @select="libraryStore.selectCategory"
      />
      <UInput
        :model-value="libraryStore.searchQuery"
        placeholder="Search samples..."
        color="neutral"
        variant="outline"
        icon="i-lucide-search"
        class="search-input"
        @update:model-value="libraryStore.setSearchQuery"
      />
    </div>

    <div class="sample-container">
      <SampleList :samples="libraryStore.filteredSamples" />
    </div>
  </section>
</template>

<style scoped>
.browser-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.section-title {
  font-size: 20px;
  font-weight: 700;
  letter-spacing: -0.3px;
}

.result-count {
  font-size: 13px;
  color: var(--color-text-muted);
  font-variant-numeric: tabular-nums;
}

.browser-toolbar {
  display: flex;
  align-items: center;
  gap: var(--space-md);
}

.search-input {
  width: 260px;
  flex-shrink: 0;
}

.sample-container {
  max-height: 480px;
  overflow-y: auto;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
}
</style>
