export interface Sample {
  id: number;
  name: string;
  path: string;
  category: string;
  subcategory: string;
  source: string;
  duration: number;
  sampleRate: number;
  linkedPath: string | null;
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
