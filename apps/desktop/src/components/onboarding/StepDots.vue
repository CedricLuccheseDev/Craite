<script setup lang="ts">
interface Props {
  total: number;
  current: number;
}

defineProps<Props>();
</script>

<template>
  <div class="flex gap-2 justify-center">
    <span
      v-for="i in total"
      :key="i"
      class="size-2 rounded-full bg-zinc-700 transition-all duration-300"
      :class="{ 'w-6! bg-orange-500!': i - 1 === current, 'bg-orange-500/40!': i - 1 < current }"
    />
  </div>
</template>
