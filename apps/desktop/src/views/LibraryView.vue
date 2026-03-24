<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import LibrarySidebar from '@/components/library/LibrarySidebar.vue';
import LibraryBrowser from '@/components/library/LibraryBrowser.vue';
import SourceManager from '@/components/library/SourceManager.vue';
import LibraryConfig from '@/components/library/LibraryConfig.vue';
import SettingsPanel from '@/components/library/SettingsPanel.vue';
import { useWatcher } from '@/composables/useWatcher';
import { useBackgroundScan } from '@/composables/useBackgroundScan';
import type { UnlistenFn } from '@tauri-apps/api/event';

type Section = 'browse' | 'sources' | 'export' | 'settings';

const activeSection = ref<Section>('browse');
useWatcher();

const { setupListeners } = useBackgroundScan();
let unlisteners: UnlistenFn[] = [];

onMounted(async () => {
  unlisteners = await setupListeners();
});

onUnmounted(() => {
  unlisteners.forEach(fn => fn());
});
</script>

<template>
  <div class="flex w-full h-full overflow-hidden">
    <LibrarySidebar v-model:active-section="activeSection" />

    <main class="flex-1 min-w-0 overflow-hidden">
      <LibraryBrowser v-if="activeSection === 'browse'" />
      <SourceManager v-if="activeSection === 'sources'" />
      <LibraryConfig v-if="activeSection === 'export'" />
      <SettingsPanel v-if="activeSection === 'settings'" />
    </main>
  </div>
</template>
