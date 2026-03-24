<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  color: string;
}

const props = defineProps<Props>();

const gradientStyle = computed(() => ({
  background: `radial-gradient(circle at 50% 50%, ${props.color}33, ${props.color}11, transparent 70%)`,
}));
</script>

<template>
  <div class="ambient-orb" :style="gradientStyle" />
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.ambient-orb {
  @apply fixed top-1/2 left-1/2 size-150 rounded-full
    pointer-events-none z-0;
  transform: translate(-50%, -50%);
  animation: pulse-glow 4s ease-in-out infinite;
  transition: background 1s cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes pulse-glow {
  0%,
  100% {
    opacity: 0.6;
    transform: translate(-50%, -50%) scale(1);
  }
  50% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1.05);
  }
}
</style>
