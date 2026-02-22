<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useLibraryStore } from '@/stores/library';
import LibraryStats from '@/components/library/LibraryStats.vue';
import CategoryTree from '@/components/library/CategoryTree.vue';
import SampleList from '@/components/library/SampleList.vue';

const { t } = useI18n();
const libraryStore = useLibraryStore();
</script>

<template>
  <section class="h-full flex flex-col gap-6 bg-surface rounded-2xl p-8">
    <div class="flex flex-col gap-4 shrink-0">
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-bold tracking-tight">
          {{ t('browse.title') }}
        </h2>
        <span class="text-[13px] text-muted tabular-nums">
          {{ t('browse.results', { count: libraryStore.filteredSamples.length }) }}
        </span>
      </div>
      <LibraryStats />
    </div>

    <div class="shrink-0">
      <UInput
        :model-value="libraryStore.searchQuery"
        :placeholder="t('browse.searchPlaceholder')"
        color="neutral"
        variant="outline"
        icon="i-lucide-search"
        class="w-70"
        @update:model-value="libraryStore.setSearchQuery"
      />
    </div>

    <div class="flex-1 min-h-0 flex gap-6 overflow-hidden">
      <div class="w-45 shrink-0 overflow-y-auto scrollbar-thin">
        <CategoryTree
          :categories="libraryStore.categories"
          :selected="libraryStore.selectedCategory"
          @select="libraryStore.selectCategory"
        />
      </div>
      <div class="flex-1 min-w-0 min-h-0 scrollbar-thin">
        <SampleList :samples="libraryStore.filteredSamples" />
      </div>
    </div>
  </section>
</template>
