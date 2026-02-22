<script setup lang="ts">
import { ref, computed, nextTick } from 'vue';
import { useRouter } from 'vue-router';
import { open } from '@tauri-apps/plugin-dialog';
import { useOnboarding, completeOnboarding } from '@/composables/useOnboarding';
import { useTauri } from '@/composables/useTauri';
import { useScanStore } from '@/stores/scan';
import { useLibraryConfigStore } from '@/stores/libraryConfig';
import type { DawInfo } from '@/types/sample';
import GridBackground from '@/components/onboarding/GridBackground.vue';
import StepDots from '@/components/onboarding/StepDots.vue';
import ScanStep from '@/components/onboarding/ScanStep.vue';
import DawStep from '@/components/onboarding/DawStep.vue';
import ReadyStep from '@/components/onboarding/ReadyStep.vue';

const router = useRouter();
const scanStore = useScanStore();
const configStore = useLibraryConfigStore();
const tauri = useTauri();
const scanStarted = ref(false);

const {
  currentStep,
  scanProgress,
  scanTotal,
  isScanning,
  stepIndex,
  goToStep,
} = useOnboarding();

// 4 visual dots: welcome(0) → scan/result(1) → daw(2) → ready(3)
const visualStepIndex = computed(() => {
  if (currentStep.value === 'scan' && !scanStarted.value) return 0;
  return stepIndex.value + 1;
});

async function runScan() {
  scanStarted.value = true;
  isScanning.value = true;
  scanStore.startScan();

  // Wait for Vue to render the scanning UI and the browser to paint it
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
  } catch (error) {
    console.error('Scan failed:', error);
    scanStore.setScanError(String(error));
  } finally {
    isScanning.value = false;
  }
}

function onSkip() {
  completeOnboarding();
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
  // Navigate immediately to avoid UI freeze from IPC calls
  goToStep('ready');

  try {
    await tauri.createDawLibraryFolder(path);
    configStore.setOutputDir(path);
    if (daw) {
      await tauri.saveSetting('daw_choice', daw.id);
    }
    const count = await tauri.createLinks(path);
    configStore.setGenerationResult(count);
  } catch (error) {
    console.error('Failed to generate library:', error);
  }
}

function onDawSkip() {
  goToStep('ready');
}

function onFinish() {
  completeOnboarding();
  router.push('/library');
}
</script>

<template>
  <div class="flex flex-col w-full h-full bg-zinc-950 relative overflow-hidden">
    <GridBackground />

    <div class="shrink-0 pt-8 z-1 flex justify-center">
      <StepDots :total="4" :current="visualStepIndex" />
    </div>

    <div class="flex-1 min-h-0 w-full z-1 flex flex-col">
      <ScanStep
        v-if="currentStep === 'scan'"
        :is-scanning="isScanning"
        :categories="scanStore.categories"
        :total-samples="scanStore.totalSamples"
        :scan-started="scanStarted"
        @start="runScan"
        @skip="onSkip"
        @continue="onContinue"
        @add-folder="onAddFolder"
      />

      <DawStep
        v-else-if="currentStep === 'daw'"
        @select="onDawSelect"
        @skip="onDawSkip"
      />

      <ReadyStep
        v-else-if="currentStep === 'ready'"
        :total-samples="scanStore.totalSamples"
        :category-count="scanStore.categories.length"
        @finish="onFinish"
      />
    </div>
  </div>
</template>
