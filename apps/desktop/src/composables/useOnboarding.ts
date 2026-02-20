import { ref, computed } from 'vue';
import type { OnboardingStep, SourceFolder } from '@/types/onboarding';

const STEPS: OnboardingStep[] = ['welcome', 'sources', 'scan', 'result'];

const ORB_COLORS: Record<OnboardingStep, string> = {
  welcome: '#ff6b35',
  sources: '#4a9eff',
  scan: '#4ade80',
  result: '#ff6b35',
};

export function useOnboarding() {
  const currentStep = ref<OnboardingStep>('welcome');
  const sources = ref<SourceFolder[]>([]);
  const scanProgress = ref(0);
  const scanTotal = ref(0);
  const isScanning = ref(false);

  const stepIndex = computed(() => STEPS.indexOf(currentStep.value));
  const isFirstStep = computed(() => stepIndex.value === 0);
  const isLastStep = computed(() => stepIndex.value === STEPS.length - 1);
  const orbColor = computed(() => ORB_COLORS[currentStep.value]);

  const canAdvance = computed(() => {
    if (currentStep.value === 'sources') {
      return sources.value.some(s => s.enabled);
    }
    if (currentStep.value === 'scan') {
      return !isScanning.value;
    }
    return true;
  });

  function nextStep() {
    if (!canAdvance.value || isLastStep.value) return;
    currentStep.value = STEPS[stepIndex.value + 1];
  }

  function previousStep() {
    if (isFirstStep.value) return;
    currentStep.value = STEPS[stepIndex.value - 1];
  }

  function goToStep(step: OnboardingStep) {
    currentStep.value = step;
  }

  return {
    currentStep,
    sources,
    scanProgress,
    scanTotal,
    isScanning,
    stepIndex,
    isFirstStep,
    isLastStep,
    orbColor,
    canAdvance,
    nextStep,
    previousStep,
    goToStep,
  };
}
