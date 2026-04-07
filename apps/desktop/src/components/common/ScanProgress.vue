<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';

const { t } = useI18n();

const currentFile = ref('');
const fileCount = ref(0);
const classifying = ref(false);
let unlisten: UnlistenFn | null = null;
let classifyTimer: ReturnType<typeof setTimeout> | null = null;

function resetClassifyTimer() {
  classifying.value = false;
  if (classifyTimer) clearTimeout(classifyTimer);
  classifyTimer = setTimeout(() => {
    if (fileCount.value > 0) {
      classifying.value = true;
    }
  }, 1500);
}

onMounted(async () => {
  unlisten = await listen<[string, number]>('scan-file', event => {
    currentFile.value = event.payload[0];
    fileCount.value = event.payload[1];
    resetClassifyTimer();
  });
});

onUnmounted(() => {
  unlisten?.();
  if (classifyTimer) clearTimeout(classifyTimer);
});
</script>

<template>
  <div class="flex flex-col items-center gap-5">
    <div class="scan-ring">
      <div class="scan-ring-inner" />
    </div>
    <p class="text-sm text-muted">
      {{ classifying ? t('onboarding.scan.classifying') : t('onboarding.scan.scanning') }}
    </p>
    <p v-if="fileCount > 0" class="text-lg font-bold font-mono tabular-nums text-white/80">
      {{ fileCount.toLocaleString() }}
      <span class="text-xs font-normal text-muted ml-1">{{ t('onboarding.scan.filesFound') }}</span>
    </p>
    <p v-if="currentFile && !classifying" class="text-[11px] text-muted/30 font-mono truncate max-w-xs text-center">
      {{ currentFile }}
    </p>
  </div>
</template>

<style>
.scan-ring {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  border: 2px solid rgba(255, 255, 255, 0.06);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}

.scan-ring::before {
  content: '';
  position: absolute;
  inset: -2px;
  border-radius: 50%;
  border: 2px solid transparent;
  border-top-color: #ff6b35;
  animation: spin 1s cubic-bezier(0.4, 0, 0.2, 1) infinite;
}

.scan-ring-inner {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #ff6b35;
  animation: scan-pulse 1.5s ease-in-out infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes scan-pulse {
  0%,
  100% {
    opacity: 0.4;
    transform: scale(0.8);
  }
  50% {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
