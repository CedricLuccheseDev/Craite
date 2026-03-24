<script setup lang="ts">
interface Props {
  total: number;
  current: number;
}

defineProps<Props>();
</script>

<template>
  <div class="flex gap-3 justify-center">
    <span v-for="i in total" :key="i" class="dot" :class="{ active: i - 1 === current, completed: i - 1 < current }" />
  </div>
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.dot {
  @apply size-3 rounded-full bg-zinc-800 transition-all duration-300;
  transition-timing-function: cubic-bezier(0.16, 1, 0.3, 1);
}

.dot.active {
  @apply w-7 bg-orange-500;
}

.dot.completed {
  @apply bg-orange-500 opacity-50;
}
</style>
