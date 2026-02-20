<script setup lang="ts">
import { useLibraryStore } from '@/stores/library';
import CategoryTree from '@/components/library/CategoryTree.vue';
import SampleList from '@/components/library/SampleList.vue';

const libraryStore = useLibraryStore();
</script>

<template>
  <div class="library">
    <aside class="sidebar">
      <h2 class="sidebar-title">Categories</h2>
      <CategoryTree
        :categories="libraryStore.categories"
        :selected="libraryStore.selectedCategory"
        @select="libraryStore.selectCategory"
      />
    </aside>

    <main class="content">
      <header class="content-header">
        <UInput
          :model-value="libraryStore.searchQuery"
          placeholder="Search samples..."
          color="neutral"
          variant="outline"
          icon="i-lucide-search"
          class="search-input"
          @update:model-value="libraryStore.setSearchQuery"
        />
        <span class="sample-count">
          {{ libraryStore.filteredSamples.length }} samples
        </span>
      </header>

      <SampleList :samples="libraryStore.filteredSamples" />
    </main>
  </div>
</template>

<style scoped>
.library {
  width: 100%;
  height: 100%;
  display: flex;
  background: var(--color-bg);
}

.sidebar {
  width: 240px;
  border-right: 1px solid var(--color-border);
  padding: var(--space-lg);
  overflow-y: auto;
}

.sidebar-title {
  font-size: 14px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 1px;
  color: var(--color-text-muted);
  margin-bottom: var(--space-md);
}

.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-md) var(--space-lg);
  border-bottom: 1px solid var(--color-border);
}

.search-input {
  flex: 1;
}

.sample-count {
  font-size: 13px;
  color: var(--color-text-muted);
  white-space: nowrap;
}
</style>
