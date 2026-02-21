<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useLibraryStore } from '@/stores/library';
import { resetOnboarding } from '@/composables/useOnboarding';
import LibraryStats from '@/components/library/LibraryStats.vue';
import SourceManager from '@/components/library/SourceManager.vue';
import LibraryBrowser from '@/components/library/LibraryBrowser.vue';
import LibraryConfig from '@/components/library/LibraryConfig.vue';

const router = useRouter();
const libraryStore = useLibraryStore();

const isDev = import.meta.env.DEV;

function restartOnboarding() {
  resetOnboarding();
  router.push('/');
}
</script>

<template>
  <div class="library-page">
    <div class="library-container">
      <header class="page-header">
        <h1 class="page-title">Library</h1>
        <p class="page-subtitle">
          {{ libraryStore.sampleCount.toLocaleString() }} samples organized by CrAIte
        </p>
      </header>

      <LibraryStats />
      <SourceManager />
      <LibraryBrowser />
      <LibraryConfig />
    </div>

    <UButton
      v-if="isDev"
      icon="i-lucide-rotate-ccw"
      color="neutral"
      variant="ghost"
      size="xs"
      class="dev-reset"
      title="Restart onboarding (dev only)"
      @click="restartOnboarding"
    />
  </div>
</template>

<style scoped>
.library-page {
  width: 100%;
  height: 100%;
  overflow-y: auto;
  background: var(--color-bg);
}

.library-container {
  max-width: 1280px;
  margin: 0 auto;
  padding: var(--space-2xl) var(--space-2xl) var(--space-4xl);
  display: flex;
  flex-direction: column;
  gap: var(--space-2xl);
}

.page-header {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
}

.page-title {
  font-size: 28px;
  font-weight: 700;
  letter-spacing: -0.5px;
}

.page-subtitle {
  font-size: 14px;
  color: var(--color-text-muted);
}

.dev-reset {
  position: fixed;
  bottom: 12px;
  right: 12px;
  opacity: 0.3;
  transition: opacity 0.15s;
}

.dev-reset:hover {
  opacity: 1;
}
</style>
