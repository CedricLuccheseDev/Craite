<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { open } from '@tauri-apps/plugin-dialog';
import type { DawInfo } from '@/types/sample';
import { useTauri } from '@/composables/useTauri';
import { getDawIconPath, getDawIconSvg, getCustomFolderSvg } from '@/utils/dawIcons';

const { t } = useI18n();

const emit = defineEmits<{
  select: [daw: DawInfo | null, path: string];
  skip: [];
}>();

const tauri = useTauri();
const daws = ref<DawInfo[]>([]);
const selectedId = ref<string | null>(null);
const customPath = ref('');
const isLoading = ref(true);

const selectedDaw = computed(() => daws.value.find(d => d.id === selectedId.value) ?? null);

onMounted(async () => {
  try {
    daws.value = await tauri.detectInstalledDaws();
    const firstDetected = daws.value.find(d => d.detected);
    if (firstDetected) selectedId.value = firstDetected.id;
  } catch (error) {
    console.error('Failed to detect DAWs:', error);
  } finally {
    isLoading.value = false;
  }
});

async function pickCustomFolder() {
  const selected = await open({ directory: true, multiple: false });
  if (selected) customPath.value = selected as string;
}

function onConfirm() {
  if (selectedId.value === 'custom') {
    if (customPath.value) emit('select', null, customPath.value);
    return;
  }
  if (selectedDaw.value) {
    emit('select', selectedDaw.value, selectedDaw.value.libraryPath);
  }
}
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex-1 min-h-0 overflow-y-auto scrollbar-thin">
      <div class="flex flex-col items-center justify-center gap-4 min-h-full py-4">
        <div class="flex flex-col items-center gap-2 text-center shrink-0">
          <h2 class="font-display text-[28px] font-bold tracking-tight">
            {{ t('onboarding.daw.title') }}
          </h2>
          <p class="text-sm text-muted">
            {{ t('onboarding.daw.subtitle') }}
          </p>
        </div>

        <div v-if="isLoading" class="flex items-center gap-2 text-sm text-muted">
          <UIcon name="i-lucide-loader-2" class="animate-spin" />
          <span>{{ t('onboarding.daw.detecting') }}</span>
        </div>

        <div v-else class="w-full max-w-xl grid grid-cols-4 gap-2">
          <label
            v-for="daw in daws"
            :key="daw.id"
            class="flex flex-col items-center justify-center gap-2 h-22 px-3 border border-zinc-800/60 rounded-xl cursor-pointer transition-all duration-150 bg-zinc-900/30 hover:border-zinc-600 hover:bg-zinc-800/40"
            :class="{ 'border-orange-500! bg-orange-500/8!': selectedId === daw.id }"
          >
            <input v-model="selectedId" type="radio" name="daw" :value="daw.id" class="sr-only" />
            <img v-if="getDawIconPath(daw.id)" :src="getDawIconPath(daw.id)!" :alt="daw.name" class="shrink-0 size-6" />
            <!-- eslint-disable vue/no-v-html -- SVG from internal static data, no XSS risk -->
            <span
              v-else
              class="shrink-0 size-7 overflow-hidden flex items-center justify-center"
              v-html="getDawIconSvg(daw.id)"
            />
            <!-- eslint-enable vue/no-v-html -->
            <span class="text-[12px] font-medium truncate">{{ daw.name }}</span>
          </label>

          <label
            class="flex flex-col items-center justify-center gap-2 h-22 px-3 border border-zinc-800/60 rounded-xl cursor-pointer transition-all duration-150 bg-zinc-900/30 hover:border-zinc-600 hover:bg-zinc-800/40"
            :class="{ 'border-orange-500! bg-orange-500/8!': selectedId === 'custom' }"
          >
            <input v-model="selectedId" type="radio" name="daw" value="custom" class="sr-only" />
            <!-- eslint-disable vue/no-v-html -- SVG from internal static data, no XSS risk -->
            <span
              class="shrink-0 size-7 overflow-hidden flex items-center justify-center"
              v-html="getCustomFolderSvg()"
            />
            <!-- eslint-enable vue/no-v-html -->
            <span class="text-[12px] font-medium truncate">
              {{ t('onboarding.daw.customFolder') }}
            </span>
          </label>
        </div>

        <p
          v-if="selectedId && selectedId !== 'custom' && selectedDaw"
          class="w-full max-w-xl py-2 px-4 bg-zinc-900/60 rounded-lg border border-zinc-800/40 font-mono text-[11px] text-muted truncate"
        >
          {{ selectedDaw.libraryPath }}
        </p>

        <div v-if="selectedId === 'custom'" class="w-full">
          <UButton color="neutral" variant="outline" size="sm" icon="i-lucide-folder-open" @click="pickCustomFolder">
            {{ customPath || t('onboarding.daw.chooseFolder') }}
          </UButton>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="shrink-0 flex flex-col items-center gap-3 py-5">
      <UButton
        color="primary"
        variant="solid"
        size="lg"
        class="px-10"
        :disabled="!selectedId || (selectedId === 'custom' && !customPath)"
        @click="onConfirm"
      >
        {{ t('onboarding.daw.continue') }}
      </UButton>
      <UButton color="neutral" variant="ghost" size="sm" @click="emit('skip')">
        {{ t('onboarding.daw.skipForNow') }}
      </UButton>
    </div>
  </div>
</template>
