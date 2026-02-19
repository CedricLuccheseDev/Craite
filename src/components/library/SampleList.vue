<script setup lang="ts">
import type { Sample } from '@/types/sample';
import AudioPlayer from '@/components/common/AudioPlayer.vue';

interface Props {
  samples: Sample[];
}

defineProps<Props>();
</script>

<template>
  <div class="sample-list">
    <div
      v-for="sample in samples"
      :key="sample.id"
      class="sample-row"
    >
      <AudioPlayer
        :sample-path="sample.path"
        :sample-name="sample.name"
      />
      <span class="sample-category">{{ sample.category }}</span>
    </div>

    <p
      v-if="samples.length === 0"
      class="empty"
    >
      No samples found
    </p>
  </div>
</template>

<style scoped>
.sample-list {
  display: flex;
  flex-direction: column;
  gap: 1px;
  overflow-y: auto;
}

.sample-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-sm) var(--space-md);
  background: var(--color-surface);
  transition: background var(--duration-fast);
}

.sample-row:hover {
  background: var(--color-surface-hover);
}

.sample-category {
  font-size: 12px;
  color: var(--color-text-muted);
  text-transform: capitalize;
}

.empty {
  padding: var(--space-2xl);
  text-align: center;
  color: var(--color-text-muted);
}
</style>
