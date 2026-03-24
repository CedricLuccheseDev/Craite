<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { resetOnboarding } from '@/composables/useOnboarding';
import { useTauri } from '@/composables/useTauri';
import { useScanStore } from '@/stores/scan';
import { useLibraryStore } from '@/stores/library';

type Section = 'browse' | 'sources' | 'export' | 'settings';

interface Props {
  activeSection: Section;
}

defineProps<Props>();
const emit = defineEmits<{ 'update:activeSection': [value: Section] }>();

const { t } = useI18n();
const router = useRouter();
const tauri = useTauri();
const scanStore = useScanStore();
const libraryStore = useLibraryStore();
const isDev = import.meta.env.DEV;
const isResetting = ref(false);

const navItems = computed(() => [
  { id: 'browse' as Section, label: t('nav.browse'), icon: 'i-lucide-music-2' },
  { id: 'sources' as Section, label: t('nav.sources'), icon: 'i-lucide-hard-drive' },
  { id: 'export' as Section, label: t('nav.export'), icon: 'i-lucide-link' },
  { id: 'settings' as Section, label: t('nav.settings'), icon: 'i-lucide-settings' },
]);

async function restartOnboarding() {
  isResetting.value = true;
  resetOnboarding();
  router.push('/');

  // Heavy IPC in background after navigation
  try {
    await tauri.resetApp();
  } catch (error) {
    console.error('Failed to reset app:', error);
  }
  scanStore.setDetectedSources([], false);
  libraryStore.setSamples([]);
  libraryStore.setCategories([]);
  isResetting.value = false;
}
</script>

<template>
  <aside class="flex flex-col gap-8 py-9 px-3 w-[220px] shrink-0 bg-black/10">
    <div class="px-4">
      <span class="font-display text-[22px] font-bold tracking-tight">CrAIte</span>
    </div>

    <nav class="flex flex-col gap-1">
      <button
        v-for="item in navItems"
        :key="item.id"
        class="nav-item"
        :class="{ 'bg-surface text-white': activeSection === item.id }"
        @click="emit('update:activeSection', item.id)"
      >
        <UIcon :name="item.icon" class="text-[16px] shrink-0" />
        <span>{{ item.label }}</span>
      </button>
    </nav>

    <div class="mt-auto">
      <UButton
        v-if="isDev"
        icon="i-lucide-rotate-ccw"
        color="neutral"
        variant="ghost"
        size="xs"
        class="opacity-30 transition-opacity duration-150 hover:opacity-100"
        title="Restart onboarding (dev only)"
        :loading="isResetting"
        @click="restartOnboarding"
      />
    </div>
  </aside>
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.nav-item {
  @apply flex items-center gap-3 w-full py-3 px-4 rounded-full
    text-[14px] font-medium text-muted bg-transparent border-none
    cursor-pointer transition-colors duration-150
    hover:bg-surface-hover hover:text-white;
}
</style>
