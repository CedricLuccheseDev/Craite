<script setup lang="ts">
import { ref, computed, nextTick } from 'vue';
import { useRouter } from 'vue-router';
import { open } from '@tauri-apps/plugin-dialog';
import { useOnboarding, completeOnboarding } from '@/composables/useOnboarding';
import { useTauri } from '@/composables/useTauri';
import { useNotify } from '@/composables/useNotify';
import { useLibraryActions } from '@/composables/useLibraryActions';
import { useScanStore } from '@/stores/scan';
import { useLibraryConfigStore } from '@/stores/libraryConfig';
import { useSettingsStore } from '@/stores/settings';
import type { DawInfo } from '@/types/sample';
import type { SupportedLocale } from '@/plugins/i18n';
import StepDots from '@/components/onboarding/StepDots.vue';
import ScanStep from '@/components/onboarding/ScanStep.vue';
import DawStep from '@/components/onboarding/DawStep.vue';
import ReadyStep from '@/components/onboarding/ReadyStep.vue';

const router = useRouter();
const scanStore = useScanStore();
const configStore = useLibraryConfigStore();
const tauri = useTauri();
const notify = useNotify();
const { reloadLibrary } = useLibraryActions();
const settingsStore = useSettingsStore();
const scanStarted = ref(false);

const languageOptions = [
  { label: 'EN', value: 'en' },
  { label: 'FR', value: 'fr' },
];

function onLocaleChange(value: string) {
  settingsStore.setLocale(value as SupportedLocale);
}

const { currentStep, scanProgress, scanTotal, isScanning, stepIndex, goToStep } = useOnboarding();

const visualStepIndex = computed(() => {
  if (currentStep.value === 'scan' && !scanStarted.value) return 0;
  return stepIndex.value + 1;
});

const scanRunning = ref(false);

async function runScan() {
  if (scanRunning.value) return;
  scanRunning.value = true;
  scanStarted.value = true;
  isScanning.value = true;
  scanStore.startScan();

  await nextTick();
  await new Promise(resolve => setTimeout(resolve, 300));

  try {
    if (scanStore.sources.length === 0) {
      const detected = await tauri.detectSources();
      scanStore.setDetectedSources(detected);
    }

    const result = await tauri.scanDirectories(scanStore.enabledSources);
    scanTotal.value = result.totalFiles;
    scanProgress.value = result.totalFiles;
    scanStore.setScanResult(result);
    scanStore.updateSourceCounts(result.samples);
    scanStore.removeEmptySources();
  } catch (error) {
    console.error('Scan failed:', error);
    scanStore.setScanError(String(error));
    notify.error('notify.scanFailed');
  } finally {
    isScanning.value = false;
    scanRunning.value = false;
  }
}

async function onSkip() {
  completeOnboarding();
  await loadSamplesIntoLibrary();
  router.push('/library');
}

async function onAddFolder() {
  const selected = await open({ directory: true, multiple: false });
  if (!selected) return;
  scanStore.addCustomSource(selected as string);
  await runScan();
}

function onContinue() {
  goToStep('daw');
}

async function onDawSelect(daw: DawInfo | null, path: string) {
  goToStep('ready');
  try {
    await tauri.createDawLibraryFolder(path);
    configStore.setOutputDir(path);
    if (daw) await tauri.saveSetting('daw_choice', daw.id);
    const count = await tauri.createLinks(path, []);
    configStore.setGenerationResult(count);
  } catch (error) {
    console.error('Failed to generate library:', error);
    notify.error('notify.dawSetupFailed');
  }
}

function onDawSkip() {
  goToStep('ready');
}

async function onFinish() {
  completeOnboarding();
  await loadSamplesIntoLibrary();
  router.push('/library');
}

async function loadSamplesIntoLibrary() {
  try {
    await reloadLibrary({ withSources: true });
  } catch (error) {
    console.error('Failed to load library after onboarding:', error);
  }
}
</script>

<template>
  <div class="flex flex-col w-full h-full bg-[#0a0a0a] relative overflow-hidden">
    <!-- Ambient gradients -->
    <div
      class="absolute rounded-full pointer-events-none blur-[100px] w-[500px] h-[350px] -top-30 left-1/2 -translate-x-1/2 animate-[glow-pulse_8s_ease-in-out_infinite]"
      style="background: radial-gradient(circle, rgba(255, 107, 53, 0.08) 0%, transparent 70%)"
    />
    <div
      class="absolute rounded-full pointer-events-none blur-[100px] w-[400px] h-[250px] -bottom-20 -right-12.5 animate-[glow-pulse_10s_ease-in-out_infinite_reverse]"
      style="background: radial-gradient(circle, rgba(99, 102, 241, 0.05) 0%, transparent 70%)"
    />

    <!-- Top bar: progress + language -->
    <div class="absolute top-6 inset-x-0 z-20 flex items-center justify-center px-6">
      <div class="flex-1" />
      <StepDots :total="4" :current="visualStepIndex" />
      <div class="flex-1 flex justify-end">
        <USelect
          :model-value="settingsStore.locale"
          :items="languageOptions"
          value-key="value"
          label-key="label"
          color="neutral"
          variant="ghost"
          size="xs"
          class="w-16"
          @update:model-value="onLocaleChange"
        />
      </div>
    </div>

    <!-- Step content -->
    <div class="relative z-10 flex-1 min-h-0 w-full max-w-4xl mx-auto flex flex-col px-8 pt-10">
      <Transition name="step" mode="out-in">
        <ScanStep
          v-if="currentStep === 'scan'"
          key="scan"
          :is-scanning="isScanning"
          :categories="scanStore.categories"
          :total-samples="scanStore.totalSamples"
          :scan-started="scanStarted"
          @start="runScan"
          @skip="onSkip"
          @continue="onContinue"
          @add-folder="onAddFolder"
        />

        <DawStep v-else-if="currentStep === 'daw'" key="daw" @select="onDawSelect" @skip="onDawSkip" />

        <ReadyStep
          v-else-if="currentStep === 'ready'"
          key="ready"
          :total-samples="scanStore.totalSamples"
          :category-count="scanStore.categories.length"
          :categories="scanStore.categories"
          @finish="onFinish"
        />
      </Transition>
    </div>
  </div>
</template>

<style>
@keyframes glow-pulse {
  0%,
  100% {
    opacity: 0.6;
  }
  50% {
    opacity: 1;
  }
}

.step-enter-active {
  transition:
    opacity 0.2s ease,
    transform 0.2s ease;
}

.step-leave-active {
  transition: opacity 0.12s ease;
}

.step-enter-from {
  opacity: 0;
  transform: translateY(12px);
}

.step-leave-to {
  opacity: 0;
}
</style>
