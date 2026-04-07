import { useTauri } from '@/composables/useTauri';
import { useLibraryConfigStore } from '@/stores/libraryConfig';
import { useSettingsStore } from '@/stores/settings';
import { useLibraryActions } from '@/composables/useLibraryActions';
import type { LinkStructure } from '@/types/sample';
import type { SupportedLocale } from '@/plugins/i18n';

export function useAppInit() {
  const tauri = useTauri();
  const configStore = useLibraryConfigStore();
  const settingsStore = useSettingsStore();
  const { reloadLibrary } = useLibraryActions();

  async function initialize(): Promise<void> {
    await loadSettings();
    try {
      await reloadLibrary({ withSources: true });
    } catch (error) {
      console.error('Failed to load library:', error);
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
      const lastGenerated = map.get('last_generated_at');
      if (linkedCount && lastGenerated) {
        configStore.loadGenerationResult(parseInt(linkedCount, 10), lastGenerated);
      }

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
