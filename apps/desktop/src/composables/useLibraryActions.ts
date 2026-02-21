import { open } from '@tauri-apps/plugin-dialog';
import { useTauri } from '@/composables/useTauri';
import { useScanStore } from '@/stores/scan';
import { useLibraryStore } from '@/stores/library';
import { useLibraryConfigStore } from '@/stores/libraryConfig';

export function useLibraryActions() {
  const tauri = useTauri();
  const scanStore = useScanStore();
  const libraryStore = useLibraryStore();
  const configStore = useLibraryConfigStore();

  async function rescan() {
    if (scanStore.isScanning) return;

    scanStore.startScan();
    try {
      const result = await tauri.scanDirectories(scanStore.sources);
      scanStore.setScanResult(result);
      libraryStore.setSamples(result.samples);
      libraryStore.setCategories(result.categories);
    }
    catch (error) {
      scanStore.setScanError(String(error));
    }
  }

  async function generateLibrary() {
    if (!configStore.outputDir || configStore.isGenerating) return;

    configStore.startGenerating();
    try {
      const count = await tauri.createLinks(configStore.outputDir);
      configStore.setGenerationResult(count);
    }
    catch (error) {
      configStore.isGenerating = false;
      console.error('Failed to generate library:', error);
    }
  }

  async function pickOutputDir() {
    const selected = await open({ directory: true, multiple: false });
    if (selected) {
      configStore.setOutputDir(selected as string);
    }
  }

  async function addSourceFolder() {
    const selected = await open({ directory: true, multiple: false });
    if (selected) {
      scanStore.addSource({
        path: selected as string,
        label: (selected as string).split(/[/\\]/).pop() ?? 'Custom',
        enabled: true,
        type: 'custom',
        sampleCount: 0,
      });
    }
  }

  return { rescan, generateLibrary, pickOutputDir, addSourceFolder };
}
