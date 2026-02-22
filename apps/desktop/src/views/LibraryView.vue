<script setup lang="ts">
import { ref } from 'vue';
import LibrarySidebar from '@/components/library/LibrarySidebar.vue';
import LibraryBrowser from '@/components/library/LibraryBrowser.vue';
import SourceManager from '@/components/library/SourceManager.vue';
import LibraryConfig from '@/components/library/LibraryConfig.vue';
import SettingsPanel from '@/components/library/SettingsPanel.vue';
import { useWatcher } from '@/composables/useWatcher';

type Section = 'browse' | 'sources' | 'export' | 'settings';

const activeSection = ref<Section>('browse');
useWatcher();
</script>

<template>
  <div class="flex w-full h-full bg-zinc-950 overflow-hidden">
    <LibrarySidebar v-model:active-section="activeSection" />

    <main class="flex-1 min-w-0 pt-8 pr-8 pb-8 overflow-hidden">
      <LibraryBrowser v-if="activeSection === 'browse'" />
      <SourceManager v-if="activeSection === 'sources'" />
      <LibraryConfig v-if="activeSection === 'export'" />
      <SettingsPanel v-if="activeSection === 'settings'" />
    </main>
  </div>
</template>
