<script setup lang="ts">
import { ref, watch, toRef } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Category } from '@/types/sample';
import { getCategoryColor } from '@/utils/categoryColors';
import { useScanTimer } from '@/composables/useScanTimer';

const { t } = useI18n();

interface Props {
  isScanning: boolean;
  categories: Category[];
  totalSamples: number;
  scanStarted: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  start: [];
  skip: [];
  continue: [];
  addFolder: [];
}>();

const { elapsedSeconds } = useScanTimer(toRef(props, 'isScanning'));

const scanDone = ref(false);
watch(
  () => props.isScanning,
  (scanning, wasScanning) => {
    if (wasScanning && !scanning && props.scanStarted) {
      scanDone.value = true;
    }
  }
);
</script>

<template>
  <div class="flex flex-col h-full w-full slide-up">
    <!-- Content: centered -->
    <div class="flex-1 min-h-0 flex flex-col items-center justify-center gap-8 px-16">
      <h1 class="text-[64px] font-black tracking-[-4px] text-orange-500 leading-none">CrAIte</h1>

      <!-- Welcome -->
      <template v-if="!scanStarted">
        <p class="text-lg text-muted text-center">
          {{ t('onboarding.welcome.subtitle') }}
        </p>
        <p class="text-sm text-muted text-center max-w-80 leading-relaxed">
          {{ t('onboarding.welcome.description') }}
        </p>
      </template>

      <!-- Scanning -->
      <template v-else-if="isScanning">
        <div class="flex flex-col items-center gap-5">
          <div class="spinner" />
          <p class="text-sm text-muted">
            {{ t('onboarding.scan.scanning') }}
          </p>
          <p v-if="elapsedSeconds > 5" class="text-xs text-muted opacity-70">
            {{ t('onboarding.scan.slowHint') }}
          </p>
        </div>
      </template>

      <!-- No samples -->
      <template v-else-if="scanDone && totalSamples === 0">
        <div class="flex flex-col items-center gap-2.5 text-center">
          <h2 class="text-[40px] font-extrabold tracking-[-1.5px]">
            {{ t('onboarding.result.noSamples') }}
          </h2>
          <p class="text-base text-muted">
            {{ t('onboarding.result.noSamplesHint') }}
          </p>
        </div>
      </template>

      <!-- Results found -->
      <template v-else-if="scanDone">
        <div class="flex flex-col items-center gap-2.5 text-center">
          <h2 class="text-[40px] font-extrabold tracking-[-1.5px]">
            {{ t('onboarding.result.libraryReady') }}
          </h2>
          <p class="text-base text-muted">
            {{ t('onboarding.result.samplesOrganized', { count: totalSamples.toLocaleString() }) }}
          </p>
        </div>

        <ul class="list-none w-full max-w-105 max-h-48 overflow-y-auto">
          <li
            v-for="(cat, index) in categories"
            :key="cat.name"
            class="flex items-center gap-4 py-3.5 border-b border-zinc-800 stagger-item first:border-t"
            :style="{ animationDelay: `${index * 50}ms` }"
          >
            <span class="size-2 rounded-full shrink-0" :style="{ background: getCategoryColor(cat.name) }" />
            <span class="flex-1 text-[15px] font-medium capitalize">
              {{ cat.name }}
            </span>
            <span class="font-mono text-sm text-muted">
              {{ cat.count.toLocaleString() }}
            </span>
          </li>
        </ul>
      </template>
    </div>

    <!-- Actions: pinned to bottom -->
    <div class="shrink-0 flex flex-col items-center gap-2 pb-10 pt-4">
      <!-- Welcome -->
      <template v-if="!scanStarted">
        <UButton color="primary" variant="solid" size="xl" class="px-8 py-3.5" @click="emit('start')">
          {{ t('onboarding.welcome.next') }}
        </UButton>
      </template>

      <!-- Scanning -->
      <template v-else-if="isScanning">
        <UButton color="neutral" variant="ghost" size="sm" @click="emit('skip')">
          {{ t('onboarding.scan.skip') }}
        </UButton>
      </template>

      <!-- No samples -->
      <template v-else-if="scanDone && totalSamples === 0">
        <UButton
          color="primary"
          variant="solid"
          size="xl"
          icon="i-lucide-folder-plus"
          class="px-8 py-3.5"
          @click="emit('addFolder')"
        >
          {{ t('onboarding.result.addFolder') }}
        </UButton>
        <UButton color="neutral" variant="ghost" size="sm" @click="emit('continue')">
          {{ t('onboarding.result.skipForNow') }}
        </UButton>
      </template>

      <!-- Results found -->
      <template v-else-if="scanDone">
        <UButton color="primary" variant="solid" size="lg" class="px-8 py-3.5" @click="emit('continue')">
          {{ t('onboarding.result.continue') }}
        </UButton>
        <UButton color="neutral" variant="ghost" size="sm" icon="i-lucide-folder-plus" @click="emit('addFolder')">
          {{ t('onboarding.result.addMoreFolders') }}
        </UButton>
      </template>
    </div>
  </div>
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.spinner {
  @apply size-12 rounded-full border-3 border-zinc-700;
  border-top-color: var(--color-orange-500);
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
