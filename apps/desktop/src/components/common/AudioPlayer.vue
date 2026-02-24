<script setup lang="ts">
import { ref, onUnmounted, watch } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';

interface Props {
  samplePath: string;
  sampleName: string;
}

const props = defineProps<Props>();

const isPlaying = ref(false);
const hasError = ref(false);

let audio: HTMLAudioElement | null = null;
let currentPath: string | null = null;

function cleanup() {
  if (audio) {
    audio.pause();
    audio.removeAttribute('src');
    audio.load();
    audio = null;
  }
  currentPath = null;
  isPlaying.value = false;
}

function togglePlay() {
  hasError.value = false;

  // If playing, just pause the existing audio element
  if (isPlaying.value && audio) {
    audio.pause();
    isPlaying.value = false;
    return;
  }

  // Check if we need to create a new audio element (different sample or no audio element)
  const assetUrl = convertFileSrc(props.samplePath);
  if (!audio || currentPath !== assetUrl) {
    // Clean up old audio if exists
    if (audio) {
      cleanup();
    }

    try {
      audio = new Audio(assetUrl);
      currentPath = assetUrl;

      audio.onended = () => {
        isPlaying.value = false;
      };

      audio.onerror = () => {
        hasError.value = true;
        isPlaying.value = false;
      };
    } catch {
      hasError.value = true;
      return;
    }
  }

  // Play the audio (reuse existing or newly created)
  audio.play()
    .then(() => {
      isPlaying.value = true;
    })
    .catch(() => {
      hasError.value = true;
      isPlaying.value = false;
    });
}

// Watch for sample path changes and cleanup if needed
watch(() => props.samplePath, () => {
  if (audio && isPlaying.value) {
    cleanup();
  }
});

onUnmounted(cleanup);
</script>

<template>
  <div class="flex items-center gap-2">
    <UButton
      :icon="hasError ? 'i-lucide-alert-circle' : isPlaying ? 'i-lucide-pause' : 'i-lucide-play'"
      size="sm"
      square
      :color="hasError ? 'error' : isPlaying ? 'primary' : 'neutral'"
      :variant="isPlaying ? 'solid' : 'ghost'"
      @click="togglePlay"
    />
    <span class="text-[13px] truncate">{{ sampleName }}</span>
  </div>
</template>
