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
  [key: string]: string[];
}

const dawSteps: DawSteps = {
  fl_studio: ['Options', 'File Settings', 'Browser extra search folders'],
  ableton: ['Preferences', 'Library', 'Add Folder'],
  logic_pro: ['Logic Pro', 'Settings', 'Assets', 'Add search path'],
  reaper: ['Options', 'Preferences', 'Media', 'Add path'],
  cubase: ['MediaBay', 'Define Locations', 'Add'],
  bitwig: ['Settings', 'Library', 'Add Location'],
  studio_one: ['Options', 'Locations', 'User Data'],
  maschine: ['Preferences', 'Library', 'Add'],
  pro_tools: ['Setup', 'Preferences', 'Processing', 'Add search path'],
};

function getSteps(dawId: string): string[] {
  return dawSteps[dawId] ?? [];
}
</script>

<template>
  <div v-if="!dismissed && daws.length > 0" class="flex flex-col gap-3">
    <div class="flex items-center justify-between">
      <span class="text-[12px] font-semibold uppercase tracking-[0.07em] text-muted">{{ t('dawGuide.title') }}</span>
      <UButton icon="i-lucide-x" color="neutral" variant="ghost" size="xs" @click="dismiss" />
    </div>

    <p class="text-xs text-zinc-400 -mt-1">{{ t('dawGuide.description') }}</p>

    <div class="flex flex-wrap gap-2">
      <div
        v-for="daw in daws"
        :key="daw.id"
        class="flex flex-col gap-2 px-4 py-3 rounded-xl border border-zinc-800 bg-zinc-900/40"
      >
        <span class="text-[12px] font-semibold text-white">{{ daw.name }}</span>
        <div class="flex items-center gap-1 flex-wrap">
          <template v-for="(step, i) in getSteps(daw.id)" :key="i">
            <UIcon v-if="i > 0" name="i-lucide-chevron-right" class="text-[10px] text-zinc-600" />
            <span class="text-[11px] text-zinc-400 px-2 py-0.5 rounded-md bg-zinc-800/80">{{ step }}</span>
          </template>
        </div>
      </div>
    </div>

    <p class="text-[11px] text-zinc-600 italic">{{ t('dawGuide.removeTip') }}</p>
  </div>
</template>
