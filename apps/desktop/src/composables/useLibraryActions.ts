import { nextTick } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { useTauri } from '@/composables/useTauri';
import { useNotify } from '@/composables/useNotify';
import { usePosthog } from '@/composables/usePosthog';
import { useScanStore } from '@/stores/scan';
import { useLibraryStore } from '@/stores/library';
import { useLibraryConfigStore } from '@/stores/libraryConfig';
import type { Source } from '@/types/sample';

export function useLibraryActions() {
  const tauri = useTauri();
  const notify = useNotify();
  const scanStore = useScanStore();
  const libraryStore = useLibraryStore();
  const configStore = useLibraryConfigStore();
  const ph = usePosthog();

  async function rescan() {
    if (scanStore.isScanning) return;

    scanStore.startScan();

    // Yield to let Vue render the loading UI before IPC blocks
    await nextTick();
    await new Promise(resolve => setTimeout(resolve, 50));

    try {
      const detected = await tauri.detectSources();
      const customSources = scanStore.sources.filter(s => s.type === 'custom');
      const merged = mergeSourceLists(detected, customSources);
      scanStore.setDetectedSources(merged);

      const result = await tauri.scanDirectories(scanStore.sources);
      scanStore.setScanResult(result);
      scanStore.updateSourceCounts(result.samples);
      scanStore.removeEmptySources();
      libraryStore.setSamples(result.samples);
      libraryStore.setCategories(result.categories);

      notify.success('notify.scanComplete', { count: result.totalFiles, categories: result.categories.length });
      ph.track('scan_completed', { samples: result.totalFiles, categories: result.categories.length });
      await generateLibrary();
    } catch (error) {
      scanStore.setScanError(String(error));
      console.error('Scan failed:', error);
      notify.error('notify.scanFailed');
    }
  }

  function mergeSourceLists(detected: Source[], custom: Source[]): Source[] {
    const merged = [...detected];
    for (const src of custom) {
      if (!merged.some(s => s.path === src.path)) {
        merged.push(src);
      }
    }
    return merged;
  }

  async function generateLibrary() {
    if (!configStore.outputDir || configStore.isGenerating) return;

    configStore.startGenerating();
    try {
      const count = await tauri.createLinks(configStore.outputDir, [...configStore.excludedCategories]);
      configStore.setGenerationResult(count);
      ph.track('library_generated', { linked: count });
    } catch (error) {
      configStore.isGenerating = false;
      console.error('Failed to generate library:', error);
      notify.error('notify.generateFailed');
    }
  }

  async function pickOutputDir() {
    const selected = await open({ directory: true, multiple: false });
    if (selected) {
      configStore.setOutputDir(selected as string);
      await generateLibrary();
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
      notify.info('notify.sourceAdded');
    }
  }

  return { rescan, generateLibrary, pickOutputDir, addSourceFolder };
}
