import { useTauri } from '@/composables/useTauri';
import { useLibraryStore } from '@/stores/library';
import { useScanStore } from '@/stores/scan';
import { useLibraryConfigStore } from '@/stores/libraryConfig';
import { useSettingsStore } from '@/stores/settings';
import { buildCategoriesFromSamples } from '@/utils/categoryBuilder';
import type { LinkStructure } from '@/types/sample';
import type { SupportedLocale } from '@/plugins/i18n';

export function useAppInit() {
  const tauri = useTauri();
  const libraryStore = useLibraryStore();
  const scanStore = useScanStore();
  const configStore = useLibraryConfigStore();
  const settingsStore = useSettingsStore();

  async function initialize(): Promise<void> {
    await Promise.all([
      loadSources(),
      loadSamples(),
      loadSettings(),
    ]);
  }

  async function loadSources(): Promise<void> {
    try {
      const sources = await tauri.loadSources();
      if (sources.length > 0) {
        scanStore.setDetectedSources(sources, false);
      }
    } catch (error) {
      console.error('Failed to load sources:', error);
    }
  }

  async function loadSamples(): Promise<void> {
    try {
      const samples = await tauri.loadSamples();
      if (samples.length > 0) {
        libraryStore.setSamples(samples);
        libraryStore.setCategories(buildCategoriesFromSamples(samples));
        scanStore.updateSourceCounts(samples);
      }
    } catch (error) {
      console.error('Failed to load samples:', error);
    }
  }

  async function loadSettings(): Promise<void> {
    try {
      const settings = await tauri.loadAllSettings();
      const map = new Map(settings);

      const outputDir = map.get('output_dir');
      if (outputDir) configStore.setOutputDir(outputDir, false);

      const excluded = map.get('excluded_categories');
      if (excluded) {
        const parsed: string[] = JSON.parse(excluded);
        parsed.forEach(cat => configStore.addExcludedCategory(cat));
      }

      const linkStructure = map.get('link_structure') as LinkStructure | undefined;
      if (linkStructure === 'flat' || linkStructure === 'nested') {
        configStore.setLinkStructure(linkStructure, false);
      }

      const linkedCount = map.get('linked_count');
      if (linkedCount) configStore.linkedCount = parseInt(linkedCount, 10);

      const lastGenerated = map.get('last_generated_at');
      if (lastGenerated) configStore.lastGeneratedAt = lastGenerated;

      const locale = map.get('locale') as SupportedLocale | undefined;
      if (locale === 'en' || locale === 'fr') {
        settingsStore.applyLocale(locale);
      }
    } catch (error) {
      console.error('Failed to load settings:', error);
    }
  }

  return { initialize };
}
