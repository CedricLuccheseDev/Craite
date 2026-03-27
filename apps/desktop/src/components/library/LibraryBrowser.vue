<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useLibraryStore } from '@/stores/library';
import { useNavigation } from '@/composables/useNavigation';
import LibraryStats from '@/components/library/LibraryStats.vue';
import CategoryTree from '@/components/library/CategoryTree.vue';
import SampleList from '@/components/library/SampleList.vue';

const { t } = useI18n();
const libraryStore = useLibraryStore();
const { navigateTo } = useNavigation();
</script>

<template>
  <section class="h-full flex flex-col gap-6 bg-surface rounded-2xl p-9">
    <div class="flex flex-col gap-4 shrink-0">
      <div class="flex items-start justify-between">
        <div>
          <h2 class="text-[22px] font-bold tracking-tight">
            {{ t('browse.title') }}
          </h2>
          <p class="text-sm text-muted mt-1.5">
            {{ t('browse.description') }}
          </p>
        </div>
        <LibraryStats />
      </div>
      <div class="flex items-center justify-between">
        <UInput
          :model-value="libraryStore.searchQuery"
          :placeholder="t('browse.searchPlaceholder')"
          color="neutral"
          variant="outline"
          icon="i-lucide-search"
          class="w-72"
          @update:model-value="libraryStore.setSearchQuery"
        />
        <span class="text-sm text-muted tabular-nums">
          {{ t('browse.results', { count: libraryStore.filteredSamples.length }) }}
        </span>
      </div>
    </div>

    <!-- Empty state: no samples at all -->
    <div v-if="libraryStore.samples.length === 0" class="flex-1 flex flex-col items-center justify-center gap-4 text-center">
      <div class="flex items-center justify-center size-14 rounded-2xl bg-zinc-800">
        <UIcon name="i-lucide-music-2" class="text-[26px] text-muted" />
      </div>
      <div>
        <p class="text-sm font-medium text-white/80">{{ t('browse.emptyTitle') }}</p>
        <p class="text-[12px] text-muted mt-1">{{ t('browse.emptyHint') }}</p>
      </div>
      <UButton
        icon="i-lucide-hard-drive"
        color="primary"
        variant="solid"
        size="sm"
        class="mt-2"
        @click="navigateTo('sources')"
      >
        {{ t('browse.goToSources') }}
      </UButton>
    </div>

    <!-- Main content -->
    <template v-else>
      <div class="flex-1 min-h-0 flex gap-7 overflow-hidden">
        <div class="w-52 shrink-0 overflow-y-auto scrollbar-thin pr-2">
          <CategoryTree
            :categories="libraryStore.categories"
            :selected="libraryStore.selectedCategory"
            @select="libraryStore.selectCategory"
          />
        </div>
        <div class="flex-1 min-w-0 min-h-0">
          <SampleList :samples="libraryStore.filteredSamples" />
        </div>
      </div>

    </template>
  </section>
</template>
