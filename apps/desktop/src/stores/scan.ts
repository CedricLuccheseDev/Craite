import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { useTauri } from '@/composables/useTauri';
import type { Sample, Source, ScanResult, Category } from '@/types/sample';

export const useScanStore = defineStore('scan', () => {
  const { saveSource, deleteSource, updateSourceEnabled } = useTauri();
  const sources = ref<Source[]>([]);
  const dismissedPaths = new Set<string>();
  const scanResult = ref<ScanResult | null>(null);
  const isScanning = ref(false);
  const scanProgress = ref(0);
  const error = ref<string | null>(null);

  const enabledSources = computed(() => sources.value.filter(s => s.enabled));
  const totalSamples = computed(() => scanResult.value?.totalFiles ?? 0);
  const categories = computed<Category[]>(() => scanResult.value?.categories ?? []);

  function addSource(source: Source) {
    sources.value.push(source);
    persistSource(source);
  }

  function toggleSource(path: string) {
    const source = sources.value.find(s => s.path === path);
    if (source) {
      source.enabled = !source.enabled;
      updateSourceEnabled(path, source.enabled).catch((err: unknown) => {
        console.error('Failed to persist source toggle:', err);
      });
    }
  }

  function setDetectedSources(detected: Source[], persist = true) {
    const filtered = detected.filter(s => !dismissedPaths.has(s.path));
    sources.value = filtered;
    if (persist) {
      filtered.forEach(s => persistSource(s));
    }
  }

  function addCustomSource(path: string) {
    if (sources.value.some(s => s.path === path)) return;
    const label = path.split(/[/\\]/).pop() ?? 'Custom';
    const source: Source = {
      path,
      label,
      enabled: true,
      type: 'custom',
      sampleCount: 0,
    };
    sources.value.push(source);
    persistSource(source);
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

  function updateSourceCounts(samples: Sample[]) {
    const counts = new Map<string, number>();
    for (const sample of samples) {
      counts.set(sample.source, (counts.get(sample.source) ?? 0) + 1);
    }
    for (const source of sources.value) {
      source.sampleCount = counts.get(source.path) ?? 0;
    }
  }

  function removeSource(path: string) {
    sources.value = sources.value.filter(s => s.path !== path);
    dismissedPaths.add(path);
    deleteSource(path).catch((err: unknown) => {
      console.error('Failed to delete source:', err);
    });
  }

  function removeEmptySources() {
    const empty = sources.value.filter(s => s.sampleCount === 0);
    const nonEmpty = sources.value.filter(s => s.sampleCount > 0);
    sources.value = nonEmpty;
    empty.forEach(s => {
      dismissedPaths.add(s.path);
      deleteSource(s.path).catch((err: unknown) => {
        console.error('Failed to delete empty source:', err);
      });
    });
  }

  function setScanError(message: string) {
    error.value = message;
    isScanning.value = false;
  }

  function persistSource(source: Source) {
    saveSource(source).catch((err: unknown) => {
      console.error('Failed to persist source:', err);
    });
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
    updateSourceCounts,
    removeSource,
    removeEmptySources,
    startScan,
    setScanError,
  };
});
