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
    <button
      class="play-btn"
      :class="{ playing: isPlaying }"
      @click="togglePlay"
    >
      {{ isPlaying ? '&#9646;&#9646;' : '&#9654;' }}
    </button>
    <span class="sample-name">{{ sampleName }}</span>
  </div>
</template>

<style scoped>
.audio-player {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

.play-btn {
  width: 32px;
  height: 32px;
  border-radius: var(--radius-full);
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  color: var(--color-text);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  transition: border-color var(--duration-fast);
}

.play-btn:hover {
  border-color: var(--color-accent-orange);
}

.play-btn.playing {
  border-color: var(--color-accent-orange);
  color: var(--color-accent-orange);
}

.sample-name {
  font-size: 13px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
