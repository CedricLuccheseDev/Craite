<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
interface Props {
  progress: number;
  total: number;
  isScanning: boolean;
}

const props = defineProps<Props>();

const displayCount = ref(0);

function animateCount(target: number) {
  const duration = 2200;
  const start = displayCount.value;
  const startTime = performance.now();

  function step(now: number) {
    const elapsed = now - startTime;
    const t = Math.min(elapsed / duration, 1);
    // Quadratic easing
    const eased = t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t;
    displayCount.value = Math.round(start + (target - start) * eased);

    if (t < 1) {
      requestAnimationFrame(step);
    }
  }

  requestAnimationFrame(step);
}

watch(() => props.progress, (newVal) => {
  animateCount(newVal);
});

onMounted(() => {
  if (props.progress > 0) {
    animateCount(props.progress);
  }
});
</script>

<template>
  <div class="scan-step slide-up">
    <h2>Scanning your samples</h2>

    <div class="counter">
      <span class="count">{{ displayCount.toLocaleString() }}</span>
      <span class="label">samples found</span>
    </div>

    <UProgress
      :model-value="(isScanning && total === 0) ? null : Math.min((progress / total) * 100, 100)"
      :max="100"
      color="success"
      animation="carousel"
      class="scan-progress"
    />

    <p
      v-if="isScanning"
      class="status"
    >
      Analyzing files...
    </p>
    <p
      v-else
      class="status done"
    >
      Scan complete
    </p>
  </div>
</template>

<style scoped>
.scan-step {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-lg);
  z-index: 1;
}

h2 {
  font-size: 28px;
  font-weight: 700;
}

.counter {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-xs);
}

.count {
  font-size: 72px;
  font-weight: 800;
  font-family: var(--font-mono);
  background: linear-gradient(135deg, var(--color-accent-green), #6ee7a0);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.label {
  font-size: 16px;
  color: var(--color-text-muted);
}

.status {
  color: var(--color-text-muted);
  font-size: 14px;
}

.status.done {
  color: var(--color-accent-green);
}

.scan-progress {
  width: 100%;
  max-width: 400px;
}
</style>
