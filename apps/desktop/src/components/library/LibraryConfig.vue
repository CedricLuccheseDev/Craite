<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useLibraryStore } from '@/stores/library';
import { useLibraryConfigStore } from '@/stores/libraryConfig';
import { useLibraryActions } from '@/composables/useLibraryActions';

const { t } = useI18n();
const libraryStore = useLibraryStore();
const configStore = useLibraryConfigStore();
const { pickOutputDir } = useLibraryActions();
</script>

<template>
  <section class="h-full flex flex-col gap-6 bg-surface rounded-2xl p-8 overflow-y-auto">
    <div class="shrink-0">
      <h2 class="text-xl font-bold tracking-tight">
        {{ t('export.title') }}
      </h2>
      <p class="text-[13px] text-muted mt-1">
        {{ t('export.description') }}
      </p>
    </div>

    <div class="flex flex-col gap-4">
      <div class="flex items-center gap-6 py-4 px-6 bg-zinc-950 rounded-[10px]">
        <span class="text-[13px] font-semibold text-muted min-w-35 shrink-0">
          {{ t('export.outputDir') }}
        </span>
        <div class="flex items-center gap-2 flex-1 min-w-0">
          <span
            v-if="configStore.outputDir"
            class="text-[13px] truncate flex-1"
          >
            {{ configStore.outputDir }}
          </span>
          <span
            v-else
            class="text-[13px] text-muted flex-1"
          >
            {{ t('export.noDirectory') }}
          </span>
          <UButton
            icon="i-lucide-folder"
            color="neutral"
            variant="outline"
            size="xs"
            @click="pickOutputDir"
          >
            {{ configStore.outputDir ? t('export.change') : t('export.choose') }}
          </UButton>
        </div>
      </div>

      <div class="flex items-center gap-6 py-4 px-6 bg-zinc-950 rounded-[10px]">
        <span class="text-[13px] font-semibold text-muted min-w-35 shrink-0">
          {{ t('export.includeCategories') }}
        </span>
        <div class="flex flex-wrap gap-4">
          <label
            v-for="cat in libraryStore.categories"
            :key="cat.name"
            class="flex items-center gap-1 cursor-pointer capitalize"
          >
            <UCheckbox
              :model-value="!configStore.excludedCategories.has(cat.name)"
              color="primary"
              @update:model-value="configStore.toggleCategory(cat.name)"
            />
            <span
              class="size-1.5 rounded-full"
              :style="{ background: cat.color }"
            />
            <span class="text-[13px]">{{ cat.name }}</span>
          </label>
        </div>
      </div>
    </div>

    <p
      v-if="configStore.lastGeneratedAt"
      class="text-xs text-muted"
    >
      {{ t('export.lastGenerated', { date: configStore.lastGeneratedAt, count: configStore.linkedCount.toLocaleString() }) }}
    </p>
  </section>
</template>
