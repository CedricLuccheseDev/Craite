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
*,
*::before,
*::after {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  width: 100%;
  height: 100%;
  overflow: hidden;
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  background: #0a0a0a;
  color: #e0e0e0;
}

#app {
  width: 100%;
  height: 100%;
}
</style>
