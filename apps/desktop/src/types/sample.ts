export interface Sample {
  id: number;
  name: string;
  path: string;
  category: string;
  subcategory: string;
  confidence: number;
  source: string;
  duration: number;
  sampleRate: number;
  linkedPath: string | null;
  hidden: boolean;
}

export interface Category {
  name: string;
  color: string;
  count: number;
  subcategories: string[];
}

export interface ScanResult {
  totalFiles: number;
  classifiedFiles: number;
  categories: Category[];
  samples: Sample[];
}

export interface Source {
  path: string;
  label: string;
  enabled: boolean;
  type: 'splice' | 'custom' | 'detected';
  sampleCount: number;
}

export type LinkStructure = 'flat' | 'nested';

export interface DawInfo {
  id: string;
  name: string;
  detected: boolean;
  libraryPath: string;
}
