<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  value: number;
  max: number;
  indeterminate?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  indeterminate: false,
});

const percentage = computed(() => {
  if (props.indeterminate || props.max === 0) return 0;
  return Math.min((props.value / props.max) * 100, 100);
});
</script>

<template>
  <div class="progress-bar">
    <div
      class="progress-fill"
      :class="{ indeterminate }"
      :style="{ width: indeterminate ? '100%' : `${percentage}%` }"
    />
  </div>
</template>

<style scoped>
.progress-bar {
  width: 100%;
  max-width: 400px;
  height: 6px;
  background: var(--color-border);
  border-radius: var(--radius-full);
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--color-accent-green);
  border-radius: var(--radius-full);
  transition: width var(--duration-normal) var(--ease-out-expo);
}

.progress-fill.indeterminate {
  animation: indeterminate 1.5s ease-in-out infinite;
  width: 30% !important;
}

@keyframes indeterminate {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(400%);
  }
}
</style>
