<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useScanStore } from '@/stores/scan';
import { useLibraryActions } from '@/composables/useLibraryActions';

const { t } = useI18n();
const scanStore = useScanStore();
const { rescan, addSourceFolder } = useLibraryActions();
</script>

<template>
  <section class="h-full flex flex-col gap-6 bg-surface rounded-2xl p-8 overflow-y-auto">
    <div class="flex items-start justify-between shrink-0">
      <div>
        <h2 class="text-xl font-bold tracking-tight">
          {{ t('sources.title') }}
        </h2>
        <p class="text-[13px] text-muted mt-1">
          {{ t('sources.description') }}
        </p>
      </div>
      <div class="flex gap-2">
        <UButton
          icon="i-lucide-folder-plus"
          color="neutral"
          variant="outline"
          size="sm"
          :disabled="scanStore.isScanning"
          @click="addSourceFolder"
        >
          {{ t('sources.addFolder') }}
        </UButton>
        <UButton
          icon="i-lucide-refresh-cw"
          color="primary"
          variant="solid"
          size="sm"
          :loading="scanStore.isScanning"
          @click="rescan"
        >
          {{ t('sources.rescan') }}
        </UButton>
      </div>
    </div>

    <div class="relative flex flex-col gap-px bg-zinc-800 rounded-[10px] overflow-hidden">
      <div
        v-if="scanStore.isScanning"
        class="absolute inset-0 z-10 flex items-center justify-center bg-zinc-900/70 backdrop-blur-[2px]"
      >
        <div class="flex items-center gap-3 text-sm text-muted">
          <UIcon name="i-lucide-loader-2" class="animate-spin size-5" />
          <span>{{ t('sources.scanning') }}</span>
        </div>
      </div>

      <div
        v-for="source in scanStore.sources"
        :key="source.path"
        class="flex items-center gap-4 py-2 px-4 bg-surface transition-colors duration-150 hover:bg-surface-hover"
      >
        <UCheckbox
          :model-value="source.enabled"
          color="primary"
          @update:model-value="scanStore.toggleSource(source.path)"
        />
        <div class="flex flex-col flex-1 min-w-0">
          <span class="text-[13px] font-semibold">{{ source.label }}</span>
          <span class="text-[11px] text-muted truncate">{{ source.path }}</span>
        </div>
        <UBadge :label="source.type" color="neutral" variant="subtle" size="xs" />
        <span class="text-xs tabular-nums text-muted">
          {{ source.sampleCount.toLocaleString() }}
        </span>
      </div>

      <p v-if="scanStore.sources.length === 0" class="p-8 text-center text-muted bg-surface">
        {{ t('sources.noSources') }}
      </p>
    </div>
  </section>
</template>
