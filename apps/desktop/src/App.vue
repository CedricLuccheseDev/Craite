<script setup lang="ts">
import { onMounted } from 'vue';
import { RouterView } from 'vue-router';
import { relaunch } from '@tauri-apps/plugin-process';
import { useUpdater } from '@/composables/useUpdater';
import { useAppInit } from '@/composables/useAppInit';
import { useBackgroundScan } from '@/composables/useBackgroundScan';
import { usePosthog } from '@/composables/usePosthog';
import UpdateNotification from '@/components/UpdateNotification.vue';

const { phase, updateInfo, downloadPercent, errorMessage, setupListeners, checkForUpdate, installUpdate, dismiss } =
  useUpdater();
const { initialize } = useAppInit();
const { loadStatus: loadBgStatus, setupListeners: setupBackgroundListeners } = useBackgroundScan();
const ph = usePosthog();

async function tryClipboardTracking(): Promise<string> {
  try {
    const text = await navigator.clipboard.readText();
    const trimmed = text.trim();
    if (/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(trimmed)) {
      await ph.identify(trimmed);
      await navigator.clipboard.writeText('');
      return trimmed;
    }
  } catch {
    // Clipboard not available or permission denied
  }
  return '';
}

onMounted(async () => {
  await initialize();
  ph.init();
  let trackingId = await ph.loadTrackingId();
  if (!trackingId) {
    trackingId = await tryClipboardTracking();
  }
  if (trackingId) ph.identify(trackingId);
  ph.track('app_opened');
  await setupListeners();
  try {
    await setupBackgroundListeners();
  } catch (e) {
    console.error('bg listeners failed:', e);
  }
  try {
    await loadBgStatus();
  } catch (e) {
    console.error('bg status failed:', e);
  }
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
    <UToaster position="bottom-right" />
  </UApp>
</template>

<style>
@layer base {
  html,
  body {
    @apply w-full h-full overflow-hidden antialiased;
    font-family:
      'Inter',
      -apple-system,
      BlinkMacSystemFont,
      sans-serif;
    background: #0a0a0a;
    color: #ffffff;
  }

  h1,
  h2,
  h3,
  h4 {
    font-family: 'Space Grotesk', sans-serif;
  }

  #app {
    @apply w-full h-full;
  }
}
</style>
