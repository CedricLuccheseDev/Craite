<script setup lang="ts">
import { onMounted } from 'vue';
import { RouterView } from 'vue-router';
import { relaunch } from '@tauri-apps/plugin-process';
import { useUpdater } from '@/composables/useUpdater';
import UpdateNotification from '@/components/UpdateNotification.vue';

const { phase, updateInfo, downloadPercent, errorMessage, setupListeners, checkForUpdate, installUpdate, dismiss } = useUpdater();

onMounted(async () => {
  await setupListeners();
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
    width: 100%;
    height: 100%;
    overflow: hidden;
    font-family: var(--font-sans);
    background: var(--color-bg);
    color: var(--color-text);
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  h1, h2, h3, h4 {
    font-family: var(--font-display);
  }

  #app {
    width: 100%;
    height: 100%;
  }
}
</style>
