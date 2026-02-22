<script setup lang="ts">
import { useI18n } from 'vue-i18n';

interface Props {
  totalSamples: number;
  categoryCount: number;
}

defineProps<Props>();
const { t } = useI18n();
const emit = defineEmits<{ finish: [] }>();
</script>

<template>
  <div class="flex flex-col h-full w-full slide-up">
    <!-- Content: centered -->
    <div class="flex-1 min-h-0 flex flex-col items-center justify-center gap-12 px-16">
      <div class="flex flex-col items-center gap-2.5 text-center">
        <h2 class="text-[40px] font-extrabold tracking-[-1.5px]">
          {{ t('onboarding.ready.title') }}
        </h2>
        <p class="text-base text-muted">
          {{ t('onboarding.ready.subtitle', { samples: totalSamples.toLocaleString(), categories: categoryCount }) }}
        </p>
      </div>

      <div class="flex flex-col gap-5 w-full max-w-105">
        <div class="flex items-start gap-4">
          <span
            class="shrink-0 size-9 flex items-center justify-center
              rounded-md bg-surface text-orange-500 text-lg"
          >
            <UIcon name="i-lucide-search" />
          </span>
          <div class="flex flex-col gap-0.5">
            <p class="text-[15px] font-semibold">
              {{ t('onboarding.ready.searchTitle') }}
            </p>
            <p class="text-[13px] text-muted">
              {{ t('onboarding.ready.searchDesc') }}
            </p>
          </div>
        </div>
        <div class="flex items-start gap-4">
          <span
            class="shrink-0 size-9 flex items-center justify-center
              rounded-md bg-surface text-orange-500 text-lg"
          >
            <UIcon name="i-lucide-play" />
          </span>
          <div class="flex flex-col gap-0.5">
            <p class="text-[15px] font-semibold">
              {{ t('onboarding.ready.previewTitle') }}
            </p>
            <p class="text-[13px] text-muted">
              {{ t('onboarding.ready.previewDesc') }}
            </p>
          </div>
        </div>
        <div class="flex items-start gap-4">
          <span
            class="shrink-0 size-9 flex items-center justify-center
              rounded-md bg-surface text-orange-500 text-lg"
          >
            <UIcon name="i-lucide-folder-plus" />
          </span>
          <div class="flex flex-col gap-0.5">
            <p class="text-[15px] font-semibold">
              {{ t('onboarding.ready.addSourcesTitle') }}
            </p>
            <p class="text-[13px] text-muted">
              {{ t('onboarding.ready.addSourcesDesc') }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Actions: pinned to bottom -->
    <div class="shrink-0 flex flex-col items-center gap-2 pb-10 pt-4">
      <UButton
        color="primary"
        variant="solid"
        size="xl"
        @click="emit('finish')"
      >
        {{ t('onboarding.ready.openLibrary') }}
      </UButton>
    </div>
  </div>
</template>
