import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Source, ScanResult, Category } from '@/types/sample';

export const useScanStore = defineStore('scan', () => {
  const sources = ref<Source[]>([]);
  const scanResult = ref<ScanResult | null>(null);
  const isScanning = ref(false);
  const scanProgress = ref(0);
  const error = ref<string | null>(null);

  const enabledSources = computed(() => sources.value.filter(s => s.enabled));
  const totalSamples = computed(() => scanResult.value?.totalFiles ?? 0);
  const categories = computed<Category[]>(() => scanResult.value?.categories ?? []);

  function addSource(source: Source) {
    sources.value.push(source);
  }

  function toggleSource(path: string) {
    const source = sources.value.find(s => s.path === path);
    if (source) {
      source.enabled = !source.enabled;
    }
  }

  function setDetectedSources(detected: Source[]) {
    sources.value = detected;
  }

  function addCustomSource(path: string) {
    if (sources.value.some(s => s.path === path)) return;
    const label = path.split(/[/\\]/).pop() ?? 'Custom';
    sources.value.push({
      path,
      label,
      enabled: true,
      type: 'custom',
      sampleCount: 0,
    });
  }

  function setScanResult(result: ScanResult) {
    scanResult.value = result;
    isScanning.value = false;
    scanProgress.value = result.totalFiles;
  }

  function startScan() {
    isScanning.value = true;
    scanProgress.value = 0;
    error.value = null;
  }

  function setScanError(message: string) {
    error.value = message;
    isScanning.value = false;
  }

  return {
    sources,
    scanResult,
    isScanning,
    scanProgress,
    error,
    enabledSources,
    totalSamples,
    categories,
    addSource,
    toggleSource,
    setDetectedSources,
    addCustomSource,
    setScanResult,
    startScan,
    setScanError,
  };
});
