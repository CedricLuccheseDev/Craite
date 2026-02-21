<script setup lang="ts">
import type { Category } from '@/types/sample';

interface Props {
  categories: Category[];
  totalSamples: number;
}

defineProps<Props>();
const emit = defineEmits<{ addFolder: []; continue: [] }>();

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
  <div class="result slide-up">
    <!-- Empty state -->
    <template v-if="totalSamples === 0">
      <div class="header">
        <h2>No samples found</h2>
        <p class="subtitle">
          Add a folder containing your audio samples to get started
        </p>
      </div>

      <UButton
        color="primary"
        variant="solid"
        size="xl"
        icon="i-lucide-folder-plus"
        @click="emit('addFolder')"
      >
        Add a folder
      </UButton>

      <UButton
        color="neutral"
        variant="ghost"
        size="sm"
        @click="emit('continue')"
      >
        Skip for now
      </UButton>
    </template>

    <!-- Results state -->
    <template v-else>
      <div class="header">
        <h2>Library ready</h2>
        <p class="subtitle">
          {{ totalSamples.toLocaleString() }} samples organized
        </p>
      </div>

      <ul class="category-list">
        <li
          v-for="(cat, index) in categories"
          :key="cat.name"
          class="category-row stagger-item"
          :style="{ animationDelay: `${index * 50}ms` }"
        >
          <span
            class="dot"
            :style="{ background: getCategoryColor(cat.name) }"
          />
          <span class="cat-name">{{ cat.name }}</span>
          <span class="cat-count">{{ cat.count.toLocaleString() }}</span>
        </li>
      </ul>

      <div class="actions">
        <UButton
          color="primary"
          variant="solid"
          size="lg"
          @click="emit('continue')"
        >
          Continue
        </UButton>
        <UButton
          color="neutral"
          variant="ghost"
          size="sm"
          icon="i-lucide-folder-plus"
          @click="emit('addFolder')"
        >
          Add more folders
        </UButton>
      </div>
    </template>
  </div>
</template>

<style scoped>
.result {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-2xl);
  z-index: 1;
  width: 100%;
  max-width: 420px;
}

.header {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  text-align: center;
}

h2 {
  font-size: 40px;
  font-weight: 800;
  letter-spacing: -1.5px;
}

.subtitle {
  font-size: 16px;
  color: var(--color-text-muted);
}

.category-list {
  list-style: none;
  width: 100%;
}

.category-row {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 14px 0;
  border-bottom: 1px solid var(--color-border);
}

.category-row:first-child {
  border-top: 1px solid var(--color-border);
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.cat-name {
  flex: 1;
  font-size: 15px;
  font-weight: 500;
  text-transform: capitalize;
}

.cat-count {
  font-family: var(--font-mono);
  font-size: 14px;
  color: var(--color-text-muted);
}

.actions {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-sm);
}
</style>
