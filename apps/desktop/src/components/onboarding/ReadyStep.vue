<script setup lang="ts">
interface Props {
  totalSamples: number;
  categoryCount: number;
}

defineProps<Props>();
const emit = defineEmits<{ finish: [] }>();
</script>

<template>
  <div class="ready slide-up">
    <div class="header">
      <h2>You're all set</h2>
      <p class="subtitle">
        {{ totalSamples.toLocaleString() }} samples organized across
        {{ categoryCount }} categories
      </p>
    </div>

    <div class="info">
      <div class="info-item">
        <span class="info-icon">
          <UIcon name="i-lucide-search" />
        </span>
        <div class="info-text">
          <p class="info-title">Search & Browse</p>
          <p class="info-desc">Find any sample instantly by name or category</p>
        </div>
      </div>
      <div class="info-item">
        <span class="info-icon">
          <UIcon name="i-lucide-play" />
        </span>
        <div class="info-text">
          <p class="info-title">Preview</p>
          <p class="info-desc">Listen to samples before using them</p>
        </div>
      </div>
      <div class="info-item">
        <span class="info-icon">
          <UIcon name="i-lucide-folder-plus" />
        </span>
        <div class="info-text">
          <p class="info-title">Add More Sources</p>
          <p class="info-desc">You can always add new folders from the library</p>
        </div>
      </div>
    </div>

    <UButton
      color="primary"
      variant="solid"
      size="xl"
      @click="emit('finish')"
    >
      Open Library
    </UButton>
  </div>
</template>

<style scoped>
.ready {
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

.info {
  display: flex;
  flex-direction: column;
  gap: 20px;
  width: 100%;
}

.info-item {
  display: flex;
  align-items: flex-start;
  gap: 16px;
}

.info-icon {
  flex-shrink: 0;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-sm);
  background: var(--color-surface);
  color: var(--color-accent-orange);
  font-size: 18px;
}

.info-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.info-title {
  font-size: 15px;
  font-weight: 600;
}

.info-desc {
  font-size: 13px;
  color: var(--color-text-muted);
}
</style>
