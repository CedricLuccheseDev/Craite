<script setup lang="ts">
import type { Category } from '@/types/sample';

interface Props {
  categories: Category[];
  selected: string | null;
}

defineProps<Props>();

const emit = defineEmits<{ select: [name: string | null] }>();

function handleSelect(name: string) {
  emit('select', name);
}

function clearSelection() {
  emit('select', null);
}
</script>

<template>
  <div class="category-tree">
    <UButton
      :color="selected === null ? 'primary' : 'neutral'"
      :variant="selected === null ? 'soft' : 'ghost'"
      class="category-item"
      @click="clearSelection"
    >
      <span class="name">All</span>
      <span class="count">{{ categories.reduce((sum, c) => sum + c.count, 0) }}</span>
    </UButton>

    <UButton
      v-for="cat in categories"
      :key="cat.name"
      :color="selected === cat.name ? 'primary' : 'neutral'"
      :variant="selected === cat.name ? 'soft' : 'ghost'"
      class="category-item"
      @click="handleSelect(cat.name)"
    >
      <span
        class="dot"
        :style="{ background: cat.color }"
      />
      <span class="name">{{ cat.name }}</span>
      <span class="count">{{ cat.count }}</span>
    </UButton>
  </div>
</template>

<style scoped>
.category-tree {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.category-item {
  width: 100%;
  justify-content: flex-start;
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: var(--radius-full);
  flex-shrink: 0;
}

.name {
  flex: 1;
  text-transform: capitalize;
  text-align: left;
}

.count {
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--color-text-muted);
  margin-left: auto;
}
</style>
