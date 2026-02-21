<script setup lang="ts">
import { computed } from 'vue';
import type { UpdatePhase, UpdateInfo } from '@/composables/useUpdater';

interface Props {
  phase: UpdatePhase;
  updateInfo: UpdateInfo | null;
  downloadPercent: number;
  errorMessage: string | null;
}

const props = defineProps<Props>();
const emit = defineEmits<{ install: []; dismiss: []; restart: [] }>();

const isVisible = computed(() => props.phase !== 'idle');

const title = computed(() => {
  switch (props.phase) {
    case 'available': return `Update available — v${props.updateInfo?.version ?? ''}`;
    case 'downloading': return 'Downloading update...';
    case 'ready': return 'Update ready to install';
    case 'error': return 'Update failed';
    default: return '';
  }
});
</script>

<template>
  <Transition name="slide-up">
    <UCard
      v-if="isVisible"
      class="update-card"
    >
      <div class="flex items-start justify-between gap-3 mb-3">
        <span class="text-sm font-semibold text-primary">{{ title }}</span>
        <UButton
          v-if="phase !== 'downloading'"
          icon="i-lucide-x"
          color="neutral"
          variant="ghost"
          size="xs"
          @click="emit('dismiss')"
        />
      </div>

      <p
        v-if="phase === 'available' && updateInfo?.body"
        class="text-xs text-muted mb-3 leading-relaxed max-h-18 overflow-y-auto"
      >
        {{ updateInfo.body }}
      </p>

      <p
        v-if="phase === 'error'"
        class="text-xs text-red-400 mb-3"
      >
        {{ errorMessage }}
      </p>

      <UProgress
        v-if="phase === 'downloading'"
        :model-value="downloadPercent > 0 ? downloadPercent : null"
        color="primary"
        class="mb-3"
      />

      <div class="flex justify-end">
        <UButton
          v-if="phase === 'available'"
          color="primary"
          variant="solid"
          size="sm"
          @click="emit('install')"
        >
          Install
        </UButton>
        <UButton
          v-if="phase === 'ready'"
          color="primary"
          variant="solid"
          size="sm"
          @click="emit('restart')"
        >
          Restart now
        </UButton>
      </div>
    </UCard>
  </Transition>
</template>

<style scoped>
.update-card {
  position: fixed;
  bottom: 24px;
  right: 24px;
  width: 320px;
  z-index: 9999;
}

.slide-up-enter-active,
.slide-up-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.slide-up-enter-from,
.slide-up-leave-to {
  opacity: 0;
  transform: translateY(8px);
}
</style>
