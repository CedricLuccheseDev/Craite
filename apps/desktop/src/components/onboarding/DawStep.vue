<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { open } from '@tauri-apps/plugin-dialog';
import type { DawInfo } from '@/types/sample';
import { useTauri } from '@/composables/useTauri';
import { getDawIconSvg, getCustomFolderSvg } from '@/utils/dawIcons';

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

const selectedDaw = computed(() =>
  daws.value.find(d => d.id === selectedId.value) ?? null,
);

onMounted(async () => {
  daws.value = await tauri.detectInstalledDaws();
  const firstDetected = daws.value.find(d => d.detected);
  if (firstDetected) selectedId.value = firstDetected.id;
  isLoading.value = false;
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
  <div class="flex flex-col h-full w-full slide-up">
    <!-- Content: centered -->
    <div class="flex-1 min-h-0 flex flex-col items-center justify-center gap-8 px-16 overflow-y-auto">
      <div class="flex flex-col items-center gap-2.5 text-center">
        <h2 class="text-[40px] font-extrabold tracking-[-1.5px]">
          {{ t('onboarding.daw.title') }}
        </h2>
        <p class="text-base text-muted">
          {{ t('onboarding.daw.subtitle') }}
        </p>
      </div>

      <div v-if="isLoading" class="flex items-center gap-2 text-muted">
        <UIcon name="i-lucide-loader-2" class="animate-spin" />
        <span>{{ t('onboarding.daw.detecting') }}</span>
      </div>

      <div v-else class="w-full max-w-160 grid grid-cols-3 gap-2.5">
        <label
          v-for="daw in daws"
          :key="daw.id"
          class="daw-card"
          :class="{ selected: selectedId === daw.id }"
        >
          <input
            v-model="selectedId"
            type="radio"
            name="daw"
            :value="daw.id"
            class="sr-only"
          />
          <span class="shrink-0 size-9" v-html="getDawIconSvg(daw.id)" />
          <span class="flex flex-col gap-0.5 min-w-0">
            <span class="text-[13px] font-medium truncate">
              {{ daw.name }}
            </span>
            <span
              v-if="daw.detected"
              class="text-[11px] text-orange-500 font-medium"
            >
              {{ t('onboarding.daw.detected') }}
            </span>
          </span>
        </label>

        <label
          class="daw-card"
          :class="{ selected: selectedId === 'custom' }"
        >
          <input
            v-model="selectedId"
            type="radio"
            name="daw"
            value="custom"
            class="sr-only"
          />
          <span class="shrink-0 size-9" v-html="getCustomFolderSvg()" />
          <span class="flex flex-col gap-0.5 min-w-0">
            <span class="text-[13px] font-medium truncate">
              {{ t('onboarding.daw.customFolder') }}
            </span>
          </span>
        </label>
      </div>

      <p
        v-if="selectedId && selectedId !== 'custom' && selectedDaw"
        class="w-full max-w-160 py-3 px-4 bg-surface rounded-md font-mono
          text-xs text-muted break-all"
      >
        {{ selectedDaw.libraryPath }}
      </p>

      <div v-if="selectedId === 'custom'" class="w-full max-w-160">
        <UButton
          color="neutral"
          variant="outline"
          size="sm"
          icon="i-lucide-folder-open"
          @click="pickCustomFolder"
        >
          {{ customPath || t('onboarding.daw.chooseFolder') }}
        </UButton>
      </div>
    </div>

    <!-- Actions: pinned to bottom -->
    <div class="shrink-0 flex flex-col items-center gap-2 pb-10 pt-4">
      <UButton
        color="primary"
        variant="solid"
        size="lg"
        :disabled="!selectedId || (selectedId === 'custom' && !customPath)"
        @click="onConfirm"
      >
        {{ t('onboarding.daw.continue') }}
      </UButton>
      <UButton
        color="neutral"
        variant="ghost"
        size="sm"
        @click="emit('skip')"
      >
        {{ t('onboarding.daw.skipForNow') }}
      </UButton>
    </div>
  </div>
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.daw-card {
  @apply flex items-center gap-2.5 p-3 border border-zinc-800
    rounded-[10px] cursor-pointer transition-all duration-150;
}

.daw-card:hover {
  @apply border-zinc-400 bg-surface;
}

.daw-card.selected {
  border-color: var(--color-orange-500);
  background: color-mix(in srgb, var(--color-orange-500) 8%, transparent);
}
</style>
