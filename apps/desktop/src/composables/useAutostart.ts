import { ref, onMounted } from 'vue';
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';

export function useAutostart() {
  const autostartEnabled = ref(false);
  const loading = ref(false);

  async function loadState() {
    try {
      autostartEnabled.value = await isEnabled();
    } catch (error) {
      console.error('Failed to check autostart state:', error);
    }
  }

  async function toggle() {
    loading.value = true;
    try {
      if (autostartEnabled.value) {
        await disable();
      } else {
        await enable();
      }
      autostartEnabled.value = !autostartEnabled.value;
    } catch (error) {
      console.error('Failed to toggle autostart:', error);
    } finally {
      loading.value = false;
    }
  }

  onMounted(loadState);

  return { autostartEnabled, loading, toggle };
}
