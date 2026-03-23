<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue';

const vantaRef = ref<HTMLElement | null>(null);
let vantaEffect: ReturnType<typeof Object> | null = null;

onMounted(async () => {
  const THREE = await import('three');
  const WAVES = await import('vanta/dist/vanta.waves.min');

  if (!vantaRef.value) return;

  vantaEffect = WAVES.default({
    el: vantaRef.value,
    THREE,
    mouseControls: false,
    touchControls: false,
    gyroControls: false,
    minHeight: 200,
    minWidth: 200,
    scale: 1.0,
    scaleMobile: 1.0,
    backgroundColor: 0x0a0a0a,
    color: 0xff6b35,
    shininess: 25,
    waveHeight: 20,
    waveSpeed: 0.5,
    zoom: 0.75,
  });
});

onBeforeUnmount(() => {
  if (vantaEffect && typeof vantaEffect === 'object' && 'destroy' in vantaEffect) {
    (vantaEffect as { destroy: () => void }).destroy();
  }
});
</script>

<template>
  <div class="absolute inset-0 z-0 vanta-fade opacity-40">
    <div ref="vantaRef" class="absolute inset-0" />
  </div>
</template>

<style scoped>
.vanta-fade {
  mask-image: linear-gradient(to bottom, black 10%, transparent 70%);
  -webkit-mask-image: linear-gradient(to bottom, black 10%, transparent 70%);
}
</style>
