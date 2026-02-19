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
  <div
    class="ambient-orb"
    :style="gradientStyle"
  />
</template>

<style scoped>
.ambient-orb {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 600px;
  height: 600px;
  border-radius: 50%;
  pointer-events: none;
  animation: pulse-glow 4s ease-in-out infinite;
  transition: background 1s var(--ease-out-expo);
  z-index: 0;
}

@keyframes pulse-glow {
  0%, 100% {
    opacity: 0.6;
    transform: translate(-50%, -50%) scale(1);
  }
  50% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1.05);
  }
}
</style>
