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
    <button
      class="category-item"
      :class="{ active: selected === null }"
      @click="clearSelection"
    >
      <span class="name">All</span>
      <span class="count">{{ categories.reduce((sum, c) => sum + c.count, 0) }}</span>
    </button>

    <button
      v-for="cat in categories"
      :key="cat.name"
      class="category-item"
      :class="{ active: selected === cat.name }"
      @click="handleSelect(cat.name)"
    >
      <span
        class="dot"
        :style="{ background: cat.color }"
      />
      <span class="name">{{ cat.name }}</span>
      <span class="count">{{ cat.count }}</span>
    </button>
  </div>
</template>

<style scoped>
.category-tree {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.category-item {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-md);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  color: var(--color-text);
  cursor: pointer;
  font-size: 14px;
  text-align: left;
  transition: background var(--duration-fast);
}

.category-item:hover {
  background: var(--color-surface-hover);
}

.category-item.active {
  background: var(--color-surface);
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
}

.count {
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--color-text-muted);
}
</style>
