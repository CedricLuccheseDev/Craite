<script setup lang="ts">
import { ref, onUnmounted } from 'vue';
import { convertFileSrc } from '@tauri-apps/api/core';

interface Props {
  samplePath: string;
  sampleName: string;
}

const props = defineProps<Props>();

const isPlaying = ref(false);
const hasError = ref(false);

let audio: HTMLAudioElement | null = null;

function cleanup() {
  if (audio) {
    audio.pause();
    audio.removeAttribute('src');
    audio.load();
    audio = null;
  }
  isPlaying.value = false;
}

function togglePlay() {
  hasError.value = false;

  if (isPlaying.value) {
    cleanup();
    return;
  }

  cleanup();

  try {
    const assetUrl = convertFileSrc(props.samplePath);
    audio = new Audio(assetUrl);

    audio.onended = () => {
      isPlaying.value = false;
    };

    audio.onerror = () => {
      hasError.value = true;
      isPlaying.value = false;
    };

    audio.play()
      .then(() => {
        isPlaying.value = true;
      })
      .catch(() => {
        hasError.value = true;
        isPlaying.value = false;
      });
  } catch {
    hasError.value = true;
  }
}

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
