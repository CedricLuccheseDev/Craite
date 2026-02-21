<script setup lang="ts">
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { open } from '@tauri-apps/plugin-dialog';
import { useOnboarding, completeOnboarding } from '@/composables/useOnboarding';
import { useTauri } from '@/composables/useTauri';
import { useScanStore } from '@/stores/scan';
import AmbientOrb from '@/components/onboarding/AmbientOrb.vue';
import StepDots from '@/components/onboarding/StepDots.vue';
import ScanStep from '@/components/onboarding/ScanStep.vue';
import ResultStep from '@/components/onboarding/ResultStep.vue';
import ReadyStep from '@/components/onboarding/ReadyStep.vue';

const router = useRouter();
const scanStore = useScanStore();
const tauri = useTauri();

const {
  currentStep,
  scanProgress,
  scanTotal,
  isScanning,
  stepIndex,
  orbColor,
  goToStep,
} = useOnboarding();

onMounted(() => {
  runScan();
});

async function runScan() {
  isScanning.value = true;
  scanStore.startScan();

  try {
    if (scanStore.sources.length === 0) {
      const detected = await tauri.detectSources();
      scanStore.setDetectedSources(detected);
    }

    const result = await tauri.scanDirectories(scanStore.enabledSources);

    scanTotal.value = result.totalFiles;
    scanProgress.value = result.totalFiles;
    scanStore.setScanResult(result);
  } catch (error) {
    console.error('Scan failed:', error);
    scanStore.setScanError(String(error));
  } finally {
    isScanning.value = false;
  }

  goToStep('result');
}

function onSkip() {
  completeOnboarding();
  router.push('/library');
}

async function onAddFolder() {
  const selected = await open({ directory: true, multiple: false });
  if (!selected) return;

  scanStore.addCustomSource(selected as string);
  goToStep('scan');
  await runScan();
}

function onContinue() {
  goToStep('ready');
}

function onFinish() {
  completeOnboarding();
  router.push('/library');
}
</script>

<template>
  <div class="onboarding">
    <AmbientOrb :color="orbColor" />

    <div class="top-bar">
      <StepDots :total="3" :current="stepIndex" />
    </div>

    <div class="step-container">
      <ScanStep
        v-if="currentStep === 'scan'"
        :progress="scanProgress"
        :total="scanTotal"
        :is-scanning="isScanning"
        @skip="onSkip"
      />

      <ResultStep
        v-else-if="currentStep === 'result'"
        :categories="scanStore.categories"
        :total-samples="scanStore.totalSamples"
        @add-folder="onAddFolder"
        @continue="onContinue"
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

<style scoped>
.onboarding {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  background: var(--color-bg);
  position: relative;
  overflow: hidden;
}

.top-bar {
  padding: var(--space-2xl) 0 0;
  z-index: 1;
}

.step-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  padding: 0 var(--space-3xl);
}
</style>
