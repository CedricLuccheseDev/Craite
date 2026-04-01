<script lang="ts">
import { ref } from 'vue';

// Shared across all AudioPlayer instances: only one sample plays at a time
const activeAudio = ref<HTMLAudioElement | null>(null);
const activePath = ref<string | null>(null);

// Global blob URL cache — persists across component mounts
const blobUrlCache = new Map<string, string>();
</script>

<script setup lang="ts">
import { computed, onUnmounted } from 'vue';
import { useTauri } from '@/composables/useTauri';
import { useNotify } from '@/composables/useNotify';

interface Props {
  samplePath: string;
  sampleName: string;
}

const props = defineProps<Props>();
const { readAudioFile } = useTauri();
const notify = useNotify();

const hasError = ref(false);
const isLoading = ref(false);
const isPlaying = computed(() => activePath.value === props.samplePath);

function stopCurrent() {
  if (activeAudio.value) {
    activeAudio.value.pause();
    activeAudio.value = null;
  }
  activePath.value = null;
}

function getMimeType(path: string): string {
  const ext = path.split('.').pop()?.toLowerCase();
  const mimes: Record<string, string> = {
    wav: 'audio/wav',
    mp3: 'audio/mpeg',
    flac: 'audio/flac',
    ogg: 'audio/ogg',
    aiff: 'audio/aiff',
    aif: 'audio/aiff',
  };
  return mimes[ext ?? ''] ?? 'audio/wav';
}

async function togglePlay() {
  hasError.value = false;

  if (isPlaying.value) {
    stopCurrent();
    return;
  }

  // Stop any other playing instance
  stopCurrent();

  try {
    // Get or create blob URL
    let blobUrl = blobUrlCache.get(props.samplePath);
    if (!blobUrl) {
      isLoading.value = true;
      const bytes = await readAudioFile(props.samplePath);
      const blob = new Blob([new Uint8Array(bytes)], { type: getMimeType(props.samplePath) });
      blobUrl = URL.createObjectURL(blob);
      blobUrlCache.set(props.samplePath, blobUrl);
      isLoading.value = false;
    }

    const audio = new Audio(blobUrl);
    audio.volume = 0.75;

    audio.onended = () => {
      if (activePath.value === props.samplePath) {
        activePath.value = null;
        activeAudio.value = null;
      }
    };

    audio.onerror = () => {
      hasError.value = true;
      if (activePath.value === props.samplePath) {
        activePath.value = null;
        activeAudio.value = null;
      }
    };

    await audio.play();
    activeAudio.value = audio;
    activePath.value = props.samplePath;
  } catch {
    hasError.value = true;
    isLoading.value = false;
    notify.error('notify.audioPlaybackFailed');
  }
}

onUnmounted(() => {
  if (isPlaying.value) {
    stopCurrent();
  }
});

defineExpose({ togglePlay });
</script>

<template>
  <div class="flex items-center gap-2 min-w-0">
    <UButton
      :icon="
        hasError
          ? 'i-lucide-alert-circle'
          : isLoading
            ? 'i-lucide-loader'
            : isPlaying
              ? 'i-lucide-pause'
              : 'i-lucide-play'
      "
      size="sm"
      square
      :color="hasError ? 'error' : isPlaying ? 'primary' : 'neutral'"
      :variant="isPlaying ? 'solid' : 'ghost'"
      :class="{ 'animate-spin': isLoading }"
      @click="togglePlay"
    />
    <span class="text-[13px] truncate">{{ sampleName }}</span>
  </div>
</template>
