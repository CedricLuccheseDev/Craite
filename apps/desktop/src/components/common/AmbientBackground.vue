<template>
  <div class="grid-bg" aria-hidden="true" />
</template>

<style scoped>
.grid-bg {
  position: absolute;
  inset: 0;
  z-index: 0;
  pointer-events: none;
  background-image: radial-gradient(circle, rgba(0, 0, 0, 1) 1px, transparent 1px);
  background-size: 24px 24px;
  mask-image: radial-gradient(ellipse 80% 70% at 50% 50%, black 0%, transparent 100%);
}
</style>
