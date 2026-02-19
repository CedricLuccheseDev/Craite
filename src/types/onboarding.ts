export type OnboardingStep = 'welcome' | 'sources' | 'scan' | 'result';

export interface SourceFolder {
  path: string;
  label: string;
  enabled: boolean;
  type: 'splice' | 'custom' | 'detected';
  sampleCount: number;
}

export interface OnboardingState {
  currentStep: OnboardingStep;
  sources: SourceFolder[];
  scanProgress: number;
  scanTotal: number;
  isScanning: boolean;
}
