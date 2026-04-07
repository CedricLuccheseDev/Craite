<template>
  <div class="ambient-bg absolute inset-0 z-0 pointer-events-none" aria-hidden="true" />
</template>

<style>
.ambient-bg {
  background-image: radial-gradient(circle, rgba(40, 40, 40, 1) 1px, transparent 1px);
  background-size: 24px 24px;
  mask-image: radial-gradient(ellipse 80% 70% at 50% 50%, black 0%, transparent 100%);
}
</style>
