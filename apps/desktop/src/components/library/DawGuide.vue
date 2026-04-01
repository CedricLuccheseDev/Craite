<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useTauri } from '@/composables/useTauri';
import type { DawInfo } from '@/types/sample';

const { t } = useI18n();
const tauri = useTauri();
const daws = ref<DawInfo[]>([]);
const dismissed = ref(false);

const STORAGE_KEY = 'craite_daw_guide_dismissed';

onMounted(async () => {
  if (localStorage.getItem(STORAGE_KEY) === 'true') {
    dismissed.value = true;
    return;
  }
  try {
    const detected = await tauri.detectInstalledDaws();
    daws.value = detected.filter(d => d.detected);
  } catch {
    // silently ignore
  }
});

function dismiss() {
  dismissed.value = true;
  localStorage.setItem(STORAGE_KEY, 'true');
}

interface DawSteps {
  [key: string]: string;
}

const dawSteps: DawSteps = {
  fl_studio: 'Options → File Settings → Browser extra search folders',
  ableton: 'Preferences → Library → Add Folder',
  logic_pro: 'Logic Pro → Settings → Assets → Add search path',
  reaper: 'Options → Preferences → Media → Add path',
  cubase: 'MediaBay → Define Locations → Add',
  bitwig: 'Settings → Library → Add Location',
  studio_one: 'Options → Locations → User Data',
  maschine: 'Preferences → Library → Add',
  pro_tools: 'Setup → Preferences → Processing → Add search path',
};

function getSteps(dawId: string): string {
  return dawSteps[dawId] ?? '';
}
</script>

<template>
  <div v-if="!dismissed && daws.length > 0" class="rounded-xl border border-orange-500/20 bg-orange-500/5 p-5">
    <div class="flex items-start justify-between gap-3 mb-3">
      <div class="flex items-center gap-2">
        <UIcon name="i-lucide-lightbulb" class="text-orange-500 text-[16px]" />
        <span class="text-[13px] font-semibold">{{ t('dawGuide.title') }}</span>
      </div>
      <UButton icon="i-lucide-x" color="neutral" variant="ghost" size="xs" @click="dismiss" />
    </div>

    <p class="text-xs text-zinc-400 mb-3">{{ t('dawGuide.description') }}</p>

    <div class="flex flex-col gap-2">
      <div v-for="daw in daws" :key="daw.id" class="flex items-start gap-2 text-xs">
        <span class="font-medium text-white shrink-0">{{ daw.name }}:</span>
        <span class="text-zinc-400">{{ getSteps(daw.id) || t('dawGuide.addPath') }}</span>
      </div>
    </div>

    <p class="text-xs text-zinc-500 mt-3">{{ t('dawGuide.removeTip') }}</p>
  </div>
</template>
