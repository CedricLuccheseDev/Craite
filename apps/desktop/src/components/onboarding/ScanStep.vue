<script setup lang="ts">
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Category } from '@/types/sample';
import ScanProgress from '@/components/common/ScanProgress.vue';

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

const scanDone = ref(false);
watch(
  () => props.isScanning,
  (scanning, wasScanning) => {
    if (wasScanning && !scanning && props.scanStarted) {
      scanDone.value = true;
      // Auto-continue to next step if samples were found
      if (props.totalSamples > 0) {
        setTimeout(() => emit('continue'), 300);
      }
    }
  }
);
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- Content -->
    <div class="flex-1 min-h-0 overflow-y-auto scrollbar-thin">
      <div class="flex flex-col items-center justify-center gap-6 min-h-full py-4">
        <!-- Welcome -->
        <template v-if="!scanStarted">
          <div class="flex flex-col items-center gap-6">
            <!-- Logo glow -->
            <div class="relative">
              <div class="absolute inset-0 bg-orange-500/20 rounded-full blur-[40px]" />
              <div
                class="relative flex items-center justify-center size-20 rounded-3xl bg-linear-to-b from-orange-500 to-orange-600 shadow-lg shadow-orange-500/20"
              >
                <UIcon name="i-lucide-audio-waveform" class="text-[36px] text-white" />
              </div>
            </div>

            <div class="flex flex-col items-center gap-2">
              <h1
                class="font-display text-[48px] font-bold tracking-[-3px] bg-linear-to-b from-white to-white/40 bg-clip-text text-transparent"
              >
                CrAIte
              </h1>
              <p class="text-base text-muted text-center">
                {{ t('onboarding.welcome.subtitle') }}
              </p>
            </div>

            <!-- Feature pills -->
            <div class="flex flex-wrap justify-center gap-2">
              <span class="pill">
                <UIcon name="i-lucide-scan" class="text-orange-500 text-[12px]" />
                {{ t('onboarding.welcome.featureScan') }}
              </span>
              <span class="pill">
                <UIcon name="i-lucide-tags" class="text-orange-500 text-[12px]" />
                {{ t('onboarding.welcome.featureClassify') }}
              </span>
              <span class="pill">
                <UIcon name="i-lucide-link" class="text-orange-500 text-[12px]" />
                {{ t('onboarding.welcome.featureExport') }}
              </span>
            </div>

            <p class="text-xs text-muted/40 text-center max-w-xs leading-relaxed">
              {{ t('onboarding.welcome.description') }}
            </p>
          </div>
        </template>

        <!-- Scanning -->
        <template v-else-if="isScanning">
          <ScanProgress />
        </template>

        <!-- No samples -->
        <template v-else-if="scanDone && totalSamples === 0">
          <div class="flex flex-col items-center gap-4 text-center">
            <div class="flex items-center justify-center size-14 rounded-2xl bg-zinc-800/80">
              <UIcon name="i-lucide-music-2" class="text-[24px] text-muted" />
            </div>
            <h2 class="font-display text-[28px] font-bold tracking-tight">
              {{ t('onboarding.result.noSamples') }}
            </h2>
            <p class="text-sm text-muted max-w-xs">
              {{ t('onboarding.result.noSamplesHint') }}
            </p>
          </div>
        </template>
      </div>
    </div>

    <!-- Actions -->
    <div class="shrink-0 flex flex-col items-center gap-3 py-5">
      <template v-if="!scanStarted">
        <UButton color="primary" variant="solid" size="lg" class="px-10" @click="emit('start')">
          {{ t('onboarding.welcome.next') }}
        </UButton>
      </template>

      <template v-else-if="isScanning">
        <UButton color="neutral" variant="ghost" size="sm" @click="emit('skip')">
          {{ t('onboarding.scan.skip') }}
        </UButton>
      </template>

      <template v-else-if="scanDone && totalSamples === 0">
        <UButton
          color="primary"
          variant="solid"
          size="lg"
          icon="i-lucide-folder-plus"
          class="px-8"
          @click="emit('addFolder')"
        >
          {{ t('onboarding.result.addFolder') }}
        </UButton>
        <UButton color="neutral" variant="ghost" size="sm" @click="emit('skip')">
          {{ t('onboarding.result.skipForNow') }}
        </UButton>
      </template>
    </div>
  </div>
</template>

<style scoped>
.pill {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 4px 10px;
  border-radius: 999px;
  background: rgba(255, 107, 53, 0.06);
  border: 1px solid rgba(255, 107, 53, 0.12);
  font-size: 11px;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.7);
  white-space: nowrap;
}

.cat-card {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 5px 10px;
  border-radius: 8px;
  background: rgba(24, 24, 27, 0.5);
  border: 1px solid rgba(39, 39, 42, 0.4);
  transition: all 0.15s;
}
</style>
