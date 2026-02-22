<script setup lang="ts">
import { onMounted } from 'vue';
import { RouterView } from 'vue-router';
import { relaunch } from '@tauri-apps/plugin-process';
import { useUpdater } from '@/composables/useUpdater';
import { useAppInit } from '@/composables/useAppInit';
import { useBackgroundScan } from '@/composables/useBackgroundScan';
import UpdateNotification from '@/components/UpdateNotification.vue';

const { phase, updateInfo, downloadPercent, errorMessage, setupListeners, checkForUpdate, installUpdate, dismiss } = useUpdater();
const { initialize } = useAppInit();
const { setupListeners: setupBackgroundListeners } = useBackgroundScan();

onMounted(async () => {
  await initialize();
  await setupListeners();
  await setupBackgroundListeners();
  if (!import.meta.env.DEV) {
    await checkForUpdate();
  }
});
</script>

<template>
  <UApp>
    <RouterView />
    <UpdateNotification
      :phase="phase"
      :update-info="updateInfo"
      :download-percent="downloadPercent"
      :error-message="errorMessage"
      @install="installUpdate"
      @dismiss="dismiss"
      @restart="relaunch"
    />
  </UApp>
</template>

<style>
@layer base {
  html, body {
    @apply w-full h-full overflow-hidden antialiased;
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
    background: #0a0a0a;
    color: #ffffff;
  }

  h1, h2, h3, h4 {
    font-family: 'Space Grotesk', sans-serif;
  }

  #app {
    @apply w-full h-full;
  }
}
</style>
