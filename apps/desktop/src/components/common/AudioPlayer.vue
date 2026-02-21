<script setup lang="ts">
import { ref } from 'vue';
import { useTauri } from '@/composables/useTauri';

interface Props {
  samplePath: string;
  sampleName: string;
}

const props = defineProps<Props>();
const { previewSample, stopPreview } = useTauri();

const isPlaying = ref(false);

async function togglePlay() {
  if (isPlaying.value) {
    await stopPreview();
    isPlaying.value = false;
  } else {
    await previewSample(props.samplePath);
    isPlaying.value = true;
  }
}
</script>

<template>
  <div class="audio-player">
    <UButton
      :icon="isPlaying ? 'i-lucide-pause' : 'i-lucide-play'"
      size="sm"
      square
      :color="isPlaying ? 'primary' : 'neutral'"
      :variant="isPlaying ? 'solid' : 'ghost'"
      @click="togglePlay"
    />
    <span class="sample-name">{{ sampleName }}</span>
  </div>
</template>

<style scoped>
.audio-player {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

.sample-name {
  font-size: 13px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
