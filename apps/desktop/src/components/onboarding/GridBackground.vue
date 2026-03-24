<script setup lang="ts"></script>

<template>
  <div class="grid-bg" aria-hidden="true">
    <div class="grid-lines" />
    <div class="grid-fade" />
  </div>
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.grid-bg {
  @apply fixed inset-0 pointer-events-none z-0 overflow-hidden;
}

.grid-lines {
  @apply absolute inset-0;
  background-image:
    linear-gradient(to right, rgba(255, 107, 53, 0.06) 1px, transparent 1px),
    linear-gradient(to bottom, rgba(255, 107, 53, 0.06) 1px, transparent 1px);
  background-size: 48px 48px;
}

/* Horizontal accent lines simulating sequencer rows */
.grid-lines::before {
  content: '';
  @apply absolute inset-0;
  background-image: linear-gradient(to bottom, rgba(255, 107, 53, 0.1) 1px, transparent 1px);
  background-size: 48px 192px;
}

/* Radial fade to black edges */
.grid-fade {
  @apply absolute inset-0;
  background: radial-gradient(
    ellipse 70% 60% at 50% 50%,
    transparent 0%,
    rgba(10, 10, 10, 0.6) 50%,
    rgba(10, 10, 10, 1) 80%
  );
}
</style>
