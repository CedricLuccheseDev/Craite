<script setup lang="ts">
defineProps<{
  delay?: number;
}>();

const emit = defineEmits<{ visible: [] }>();

const el = ref<HTMLElement>();
const isVisible = ref(false);

onMounted(() => {
  const observer = new IntersectionObserver(
    ([entry]) => {
      if (entry.isIntersecting) {
        isVisible.value = true;
        emit('visible');
        observer.disconnect();
      }
    },
    {
      threshold: 0.15,
      rootMargin: '-50px',
    }
  );
  if (el.value) observer.observe(el.value);
});
</script>

<template>
  <div
    ref="el"
    :style="{
      transitionDelay: delay ? `${delay}ms` : '0ms',
      transitionTimingFunction: 'cubic-bezier(0.16, 1, 0.3, 1)',
    }"
    :class="['transition-all duration-1000', isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-10']"
  >
    <slot />
  </div>
</template>
