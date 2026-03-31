<script setup lang="ts">
import { ref } from 'vue';
import LibrarySidebar from '@/components/library/LibrarySidebar.vue';
import LibraryBrowser from '@/components/library/LibraryBrowser.vue';
import SourceManager from '@/components/library/SourceManager.vue';
import LibraryConfig from '@/components/library/LibraryConfig.vue';
import SettingsPanel from '@/components/library/SettingsPanel.vue';
import ScanProgress from '@/components/common/ScanProgress.vue';
import { useWatcher } from '@/composables/useWatcher';
import { useScanStore } from '@/stores/scan';
import { provideNavigation, type Section } from '@/composables/useNavigation';
import AmbientBackground from '@/components/common/AmbientBackground.vue';

const scanStore = useScanStore();

const activeSection = ref<Section>('sources');
provideNavigation(activeSection);
useWatcher();
</script>

<template>
  <div class="flex w-full h-full overflow-hidden relative">
    <LibrarySidebar v-model:active-section="activeSection" />

    <main class="flex-1 min-w-0 h-full overflow-hidden relative">
      <AmbientBackground />
      <Transition name="page" mode="out-in">
        <LibraryBrowser v-if="activeSection === 'browse'" key="browse" />
        <SourceManager v-else-if="activeSection === 'sources'" key="sources" />
        <LibraryConfig v-else-if="activeSection === 'export'" key="export" />
        <SettingsPanel v-else-if="activeSection === 'settings'" key="settings" />
      </Transition>
    </main>

    <!-- Scan overlay -->
    <Transition name="overlay">
      <div v-if="scanStore.isScanning" class="scan-overlay">
        <ScanProgress />
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.page-enter-active {
  transition:
    opacity 0.15s ease,
    transform 0.15s ease;
}

.page-leave-active {
  transition: opacity 0.1s ease;
}

.page-enter-from {
  opacity: 0;
  transform: translateY(6px);
}

.page-leave-to {
  opacity: 0;
}

.scan-overlay {
  position: absolute;
  inset: 0;
  z-index: 50;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(10, 10, 10, 0.8);
  backdrop-filter: blur(12px);
}

.overlay-enter-active {
  transition: opacity 0.2s ease;
}

.overlay-leave-active {
  transition: opacity 0.3s ease;
}

.overlay-enter-from,
.overlay-leave-to {
  opacity: 0;
}
</style>
