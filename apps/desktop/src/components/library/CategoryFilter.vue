<script setup lang="ts">
import type { Category } from '@/types/sample';

interface Props {
  categories: Category[];
  selected: string | null;
}

defineProps<Props>();

const emit = defineEmits<{
  select: [name: string | null];
}>();
</script>

<template>
  <div class="category-filter">
    <UButton
      :color="selected === null ? 'primary' : 'neutral'"
      :variant="selected === null ? 'soft' : 'ghost'"
      size="xs"
      class="shrink-0 rounded-full capitalize"
      @click="emit('select', null)"
    >
      All
    </UButton>

    <UButton
      v-for="cat in categories"
      :key="cat.name"
      :color="selected === cat.name ? 'primary' : 'neutral'"
      :variant="selected === cat.name ? 'soft' : 'ghost'"
      size="xs"
      class="shrink-0 rounded-full capitalize"
      @click="emit('select', cat.name)"
    >
      <span
        class="inline-block size-1.5 rounded-full"
        :style="{ background: cat.color }"
      />
      {{ cat.name }}
      <span class="text-[11px] tabular-nums text-muted">{{ cat.count }}</span>
    </UButton>
  </div>
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.category-filter {
  @apply flex gap-1 overflow-x-auto;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.category-filter::-webkit-scrollbar {
  display: none;
}
</style>
