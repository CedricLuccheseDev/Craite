<script setup lang="ts"></script>

<template>
  <div class="fixed inset-0 pointer-events-none z-0 overflow-hidden" aria-hidden="true">
    <div class="grid-lines absolute inset-0" />
    <div
      class="absolute inset-0"
      style="
        background: radial-gradient(
          ellipse 70% 60% at 50% 50%,
          transparent 0%,
          rgba(10, 10, 10, 0.6) 50%,
          rgba(10, 10, 10, 1) 80%
        );
      "
    />
  </div>
</template>

<style>
.grid-lines {
  background-image:
    linear-gradient(to right, rgba(255, 107, 53, 0.06) 1px, transparent 1px),
    linear-gradient(to bottom, rgba(255, 107, 53, 0.06) 1px, transparent 1px);
  background-size: 48px 48px;
}

.grid-lines::before {
  content: '';
  position: absolute;
  inset: 0;
  background-image: linear-gradient(to bottom, rgba(255, 107, 53, 0.1) 1px, transparent 1px);
  background-size: 48px 192px;
}
</style>
