<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  value: number;
  max: number;
  indeterminate?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  indeterminate: false,
});

const modelValue = computed(() => {
  if (props.indeterminate || props.max === 0) return null;
  return Math.min((props.value / props.max) * 100, 100);
});
</script>

<template>
  <UProgress :model-value="modelValue" :max="100" color="success" animation="carousel" class="w-full max-w-100" />
</template>
