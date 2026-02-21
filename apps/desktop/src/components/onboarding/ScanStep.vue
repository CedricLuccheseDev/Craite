<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';

interface Props {
  progress: number;
  total: number;
  isScanning: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{ skip: [] }>();
const displayCount = ref(0);
const elapsedSeconds = ref(0);

let timerInterval: ReturnType<typeof setInterval> | null = null;

function animateCount(target: number) {
  const duration = 2200;
  const start = displayCount.value;
  const startTime = performance.now();

  function step(now: number) {
    const elapsed = now - startTime;
    const t = Math.min(elapsed / duration, 1);
    const eased = t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t;
    displayCount.value = Math.round(start + (target - start) * eased);
    if (t < 1) requestAnimationFrame(step);
  }

  requestAnimationFrame(step);
}

function startTimer() {
  elapsedSeconds.value = 0;
  timerInterval = setInterval(() => {
    elapsedSeconds.value++;
  }, 1000);
}

function stopTimer() {
  if (timerInterval) {
    clearInterval(timerInterval);
    timerInterval = null;
  }
}

watch(() => props.progress, animateCount);
watch(() => props.isScanning, (scanning) => {
  if (scanning) {
    displayCount.value = 0;
    startTimer();
  } else {
    stopTimer();
  }
});

onMounted(() => {
  if (props.progress > 0) animateCount(props.progress);
  if (props.isScanning) startTimer();
});

onUnmounted(stopTimer);
</script>

<template>
  <div class="scan slide-up">
    <h1 class="logo">CrAIte</h1>

    <div class="scan-status">
      <UProgress
        :model-value="(isScanning && total === 0) ? null : Math.min((progress / total) * 100, 100)"
        color="primary"
        class="bar"
      />
      <p class="status-text">
        {{ isScanning ? 'Scanning in progress...' : 'Scan complete' }}
      </p>
      <p
        v-if="!isScanning && displayCount > 0"
        class="count-text"
      >
        {{ displayCount.toLocaleString() }} samples found
      </p>
      <p
        v-if="isScanning && elapsedSeconds > 5"
        class="slow-hint"
      >
        Large libraries may take a moment...
      </p>
    </div>

    <UButton
      color="neutral"
      variant="ghost"
      size="sm"
      @click="emit('skip')"
    >
      Skip
    </UButton>
  </div>
</template>

<style scoped>
.scan {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-2xl);
  z-index: 1;
  width: 100%;
  max-width: 440px;
}

.logo {
  font-size: 64px;
  font-weight: 900;
  letter-spacing: -4px;
  color: var(--color-accent-orange);
  line-height: 0.9;
}

.scan-status {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 14px;
  width: 100%;
}

.bar {
  width: 100%;
}

.status-text {
  font-size: 14px;
  color: var(--color-text-muted);
}

.count-text {
  font-size: 13px;
  color: var(--color-text-muted);
  opacity: 0.8;
}

.slow-hint {
  font-size: 12px;
  color: var(--color-text-muted);
  opacity: 0.7;
}
</style>
