<script setup lang="ts">
import type { Category } from '@/types/sample';

interface Props {
  categories: Category[];
  totalSamples: number;
}

defineProps<Props>();

const emit = defineEmits<{ finish: [] }>();

const CATEGORY_COLORS: Record<string, string> = {
  kick: 'var(--color-kick)',
  snare: 'var(--color-snare)',
  hihat: 'var(--color-hihat)',
  clap: 'var(--color-clap)',
  pad: 'var(--color-pad)',
  lead: 'var(--color-lead)',
  bass: 'var(--color-bass)',
  fx: 'var(--color-fx)',
  vocal: 'var(--color-vocal)',
  loop: 'var(--color-loop)',
};

function getCategoryColor(name: string): string {
  return CATEGORY_COLORS[name.toLowerCase()] ?? 'var(--color-text-muted)';
}
</script>

<template>
  <div class="result-step slide-up">
    <h2>Your library is ready</h2>
    <p class="subtitle">{{ totalSamples.toLocaleString() }} samples organized</p>

    <div class="tree">
      <div
        v-for="(cat, index) in categories"
        :key="cat.name"
        class="tree-row stagger-item"
        :style="{ animationDelay: `${index * 60}ms` }"
      >
        <span
          class="tree-color"
          :style="{ background: getCategoryColor(cat.name) }"
        />
        <span class="tree-name">{{ cat.name }}</span>
        <span class="tree-count">{{ cat.count }}</span>
      </div>
    </div>

    <button
      class="finish-button"
      @click="emit('finish')"
    >
      Open Library
    </button>
  </div>
</template>

<style scoped>
.result-step {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-lg);
  z-index: 1;
  width: 100%;
  max-width: 400px;
}

h2 {
  font-size: 28px;
  font-weight: 700;
}

.subtitle {
  color: var(--color-text-muted);
}

.tree {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.tree-row {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface);
  border-radius: var(--radius-md);
}

.tree-color {
  width: 12px;
  height: 12px;
  border-radius: var(--radius-full);
  flex-shrink: 0;
}

.tree-name {
  flex: 1;
  font-weight: 600;
  text-transform: capitalize;
}

.tree-count {
  font-family: var(--font-mono);
  font-size: 14px;
  color: var(--color-text-muted);
}

.finish-button {
  margin-top: var(--space-md);
  padding: var(--space-md) var(--space-2xl);
  background: var(--color-accent-orange);
  color: white;
  border: none;
  border-radius: var(--radius-lg);
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: transform var(--duration-fast) var(--ease-out-expo);
}

.finish-button:hover {
  transform: scale(1.05);
}
</style>
