<script setup lang="ts">
interface Props {
  total: number;
  current: number;
}

defineProps<Props>();
</script>

<template>
  <div class="step-dots">
    <span
      v-for="i in total"
      :key="i"
      class="dot"
      :class="{ active: i - 1 === current, completed: i - 1 < current }"
    />
  </div>
</template>

<style scoped>
.step-dots {
  display: flex;
  gap: 12px;
  justify-content: center;
}

.dot {
  width: 12px;
  height: 12px;
  border-radius: var(--radius-full);
  background: var(--color-border);
  transition: all var(--duration-normal) var(--ease-out-expo);
}

.dot.active {
  width: 28px;
  background: var(--color-accent-orange);
}

.dot.completed {
  background: var(--color-accent-orange);
  opacity: 0.5;
}
</style>
