<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { Category } from '@/types/sample';
import { getCategoryColor } from '@/utils/categoryColors';

const { t } = useI18n();

interface Props {
  categories: Category[];
  totalSamples: number;
}

defineProps<Props>();
const emit = defineEmits<{ addFolder: []; continue: [] }>();

</script>

<template>
  <div class="flex flex-col items-center gap-12 z-1 w-full max-w-105 slide-up">
    <!-- Empty state -->
    <template v-if="totalSamples === 0">
      <div class="flex flex-col items-center gap-2.5 text-center">
        <h2 class="text-[40px] font-extrabold tracking-[-1.5px]">
          {{ t('onboarding.result.noSamples') }}
        </h2>
        <p class="text-base text-muted">
          {{ t('onboarding.result.noSamplesHint') }}
        </p>
      </div>

      <UButton
        color="primary"
        variant="solid"
        size="xl"
        icon="i-lucide-folder-plus"
        @click="emit('addFolder')"
      >
        {{ t('onboarding.result.addFolder') }}
      </UButton>

      <UButton
        color="neutral"
        variant="ghost"
        size="sm"
        @click="emit('continue')"
      >
        {{ t('onboarding.result.skipForNow') }}
      </UButton>
    </template>

    <!-- Results state -->
    <template v-else>
      <div class="flex flex-col items-center gap-2.5 text-center">
        <h2 class="text-[40px] font-extrabold tracking-[-1.5px]">
          {{ t('onboarding.result.libraryReady') }}
        </h2>
        <p class="text-base text-muted">
          {{ t('onboarding.result.samplesOrganized', { count: totalSamples.toLocaleString() }) }}
        </p>
      </div>

      <ul class="list-none w-full">
        <li
          v-for="(cat, index) in categories"
          :key="cat.name"
          class="flex items-center gap-4 py-3.5 border-b border-zinc-800
            stagger-item first:border-t"
          :style="{ animationDelay: `${index * 50}ms` }"
        >
          <span
            class="size-2 rounded-full shrink-0"
            :style="{ background: getCategoryColor(cat.name) }"
          />
          <span class="flex-1 text-[15px] font-medium capitalize">
            {{ cat.name }}
          </span>
          <span class="font-mono text-sm text-muted">
            {{ cat.count.toLocaleString() }}
          </span>
        </li>
      </ul>

      <div class="flex flex-col items-center gap-2">
        <UButton
          color="primary"
          variant="solid"
          size="lg"
          @click="emit('continue')"
        >
          {{ t('onboarding.result.continue') }}
        </UButton>
        <UButton
          color="neutral"
          variant="ghost"
          size="sm"
          icon="i-lucide-folder-plus"
          @click="emit('addFolder')"
        >
          {{ t('onboarding.result.addMoreFolders') }}
        </UButton>
      </div>
    </template>
  </div>
</template>
