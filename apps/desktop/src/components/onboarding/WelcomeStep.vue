<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const emit = defineEmits<{ next: [] }>();
</script>

<template>
  <div class="flex flex-col items-center gap-12 z-1 w-full max-w-110 slide-up">
    <div class="flex flex-col items-center gap-4 text-center">
      <h1 class="text-[64px] font-black tracking-[-4px] text-orange-500 leading-none">
        CrAIte
      </h1>
      <p class="text-lg text-muted">
        {{ t('onboarding.welcome.subtitle') }}
      </p>
    </div>

    <p class="text-sm text-muted text-center max-w-80 leading-relaxed">
      {{ t('onboarding.welcome.description') }}
    </p>

    <UButton
      color="primary"
      variant="solid"
      size="xl"
      @click="emit('next')"
    >
      {{ t('onboarding.welcome.next') }}
    </UButton>
  </div>
</template>
