<script setup lang="ts">
defineProps<{
  delay?: number;
}>();

const el = ref<HTMLElement>();
const isVisible = ref(false);

onMounted(() => {
  const observer = new IntersectionObserver(
    ([entry]) => {
      if (entry.isIntersecting) {
        isVisible.value = true;
        observer.disconnect();
      }
    },
    { threshold: 0.08 },
  );
  if (el.value) observer.observe(el.value);
});
</script>

<template>
  <div
    ref="el"
    :style="delay ? `transition-delay: ${delay}ms` : ''"
    :class="[
      'transition-all duration-700 ease-out',
      isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8',
    ]"
  >
    <slot />
  </div>
</template>
