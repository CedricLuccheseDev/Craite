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
      class="filter-pill"
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
      class="filter-pill"
      @click="emit('select', cat.name)"
    >
      <span class="dot" :style="{ background: cat.color }" />
      {{ cat.name }}
      <span class="pill-count">{{ cat.count }}</span>
    </UButton>
  </div>
</template>

<style scoped>
.category-filter {
  display: flex;
  gap: var(--space-xs);
  overflow-x: auto;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.category-filter::-webkit-scrollbar {
  display: none;
}

.filter-pill {
  flex-shrink: 0;
  border-radius: var(--radius-full);
  text-transform: capitalize;
}

.dot {
  display: inline-block;
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.pill-count {
  font-size: 11px;
  font-variant-numeric: tabular-nums;
  color: var(--color-text-muted);
}
</style>
