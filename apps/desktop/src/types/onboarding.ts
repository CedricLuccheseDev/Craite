export type OnboardingStep = 'scan' | 'daw' | 'ready';

export interface SourceFolder {
  path: string;
  label: string;
  enabled: boolean;
  type: 'splice' | 'custom' | 'detected';
  sampleCount: number;
}
