<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const emit = defineEmits<{ next: [] }>();
</script>

<template>
  <div class="flex flex-col items-center gap-14 z-1 w-full max-w-110 slide-up">
    <div class="flex flex-col items-center gap-5 text-center">
      <h1 class="text-[72px] font-black tracking-[-4px] text-orange-500 leading-none">CrAIte</h1>
      <p class="text-xl text-muted">
        {{ t('onboarding.welcome.subtitle') }}
      </p>
    </div>

    <p class="text-base text-muted text-center max-w-80 leading-relaxed">
      {{ t('onboarding.welcome.description') }}
    </p>

    <UButton color="primary" variant="solid" size="xl" class="px-10 py-4" @click="emit('next')">
      {{ t('onboarding.welcome.next') }}
    </UButton>
  </div>
</template>
