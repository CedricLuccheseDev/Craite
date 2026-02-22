import { ref, computed } from 'vue';
import type { OnboardingStep } from '@/types/onboarding';

const STEPS: OnboardingStep[] = ['scan', 'daw', 'ready'];
const STORAGE_KEY = 'craite_onboarding_completed';

const STEP_COLORS: Record<OnboardingStep, string> = {
  scan: '#ff6b35',
  daw: '#a78bfa',
  ready: '#22c55e',
};

export function useOnboarding() {
  const currentStep = ref<OnboardingStep>('scan');
  const isScanning = ref(false);
  const scanProgress = ref(0);
  const scanTotal = ref(0);

  const stepIndex = computed(() => STEPS.indexOf(currentStep.value));
  const stepColor = computed(() => STEP_COLORS[currentStep.value]);

  function goToStep(step: OnboardingStep) {
    currentStep.value = step;
  }

  return {
    currentStep,
    isScanning,
    scanProgress,
    scanTotal,
    stepIndex,
    stepColor,
    goToStep,
  };
}

export function isOnboardingCompleted(): boolean {
  return localStorage.getItem(STORAGE_KEY) === 'true';
}

export function completeOnboarding(): void {
  localStorage.setItem(STORAGE_KEY, 'true');
}

export function resetOnboarding(): void {
  localStorage.removeItem(STORAGE_KEY);
}
