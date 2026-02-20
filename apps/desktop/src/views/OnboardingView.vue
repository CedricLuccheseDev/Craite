<script setup lang="ts">
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useOnboarding } from '@/composables/useOnboarding';
import { useTauri } from '@/composables/useTauri';
import { useScanStore } from '@/stores/scan';
import type { SourceFolder } from '@/types/onboarding';
import AmbientOrb from '@/components/onboarding/AmbientOrb.vue';
import StepDots from '@/components/onboarding/StepDots.vue';
import WelcomeStep from '@/components/onboarding/WelcomeStep.vue';
import SourcesStep from '@/components/onboarding/SourcesStep.vue';
import ScanStep from '@/components/onboarding/ScanStep.vue';
import ResultStep from '@/components/onboarding/ResultStep.vue';

const router = useRouter();
const scanStore = useScanStore();
const tauri = useTauri();

const {
  currentStep,
  sources,
  scanProgress,
  scanTotal,
  isScanning,
  stepIndex,
  orbColor,
  canAdvance,
  nextStep,
} = useOnboarding();

onMounted(async () => {
  const detected = await tauri.detectSources();
  sources.value = detected.map(s => ({
    path: s.path,
    label: s.label,
    enabled: s.enabled,
    type: s.type as SourceFolder['type'],
    sampleCount: s.sampleCount,
  }));
});

function toggleSource(path: string) {
  const source = sources.value.find(s => s.path === path);
  if (source) source.enabled = !source.enabled;
}

function addSource(source: SourceFolder) {
  sources.value.push(source);
}

async function startScan() {
  nextStep();
  isScanning.value = true;
  scanStore.startScan();

  const result = await tauri.scanDirectories(
    sources.value.filter(s => s.enabled),
  );

  scanTotal.value = result.totalFiles;
  scanProgress.value = result.totalFiles;
  isScanning.value = false;
  scanStore.setScanResult(result);
}

function finishOnboarding() {
  router.push('/library');
}
</script>

<template>
  <div class="onboarding">
    <AmbientOrb :color="orbColor" />

    <StepDots
      :total="4"
      :current="stepIndex"
    />

    <div class="step-container">
      <WelcomeStep
        v-if="currentStep === 'welcome'"
        @next="nextStep"
      />

      <SourcesStep
        v-else-if="currentStep === 'sources'"
        :sources="sources"
        @toggle="toggleSource"
        @add-source="addSource"
      />

      <ScanStep
        v-else-if="currentStep === 'scan'"
        :progress="scanProgress"
        :total="scanTotal"
        :is-scanning="isScanning"
      />

      <ResultStep
        v-else-if="currentStep === 'result'"
        :categories="scanStore.categories"
        :total-samples="scanStore.totalSamples"
        @finish="finishOnboarding"
      />
    </div>

    <div class="bottom-actions">
      <UButton
        v-if="currentStep === 'sources'"
        color="primary"
        variant="solid"
        size="lg"
        :disabled="!canAdvance"
        @click="startScan"
      >
        Start Scan
      </UButton>
      <UButton
        v-if="currentStep === 'scan' && !isScanning"
        color="primary"
        variant="solid"
        size="lg"
        @click="nextStep"
      >
        See Results
      </UButton>
      <UButton
        v-if="currentStep !== 'welcome' && currentStep !== 'result'"
        color="neutral"
        variant="ghost"
        size="sm"
        @click="finishOnboarding"
      >
        Skip
      </UButton>
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

.step-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  padding: var(--space-xl);
}

.bottom-actions {
  padding: var(--space-xl);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-md);
  z-index: 1;
}

</style>
