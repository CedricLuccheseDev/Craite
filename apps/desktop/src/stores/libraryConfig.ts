import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { useTauri } from '@/composables/useTauri';
import type { LinkStructure } from '@/types/sample';

export const useLibraryConfigStore = defineStore('libraryConfig', () => {
  const { saveSetting } = useTauri();
  const outputDir = ref<string | null>(null);
  const excludedCategories = ref<Set<string>>(new Set());
  const linkStructure = ref<LinkStructure>('nested');
  const isGenerating = ref(false);
  const linkedCount = ref(0);
  const lastGeneratedAt = ref<string | null>(null);

  const hasOutputDir = computed(() => outputDir.value !== null);

  function setOutputDir(dir: string, persist = true) {
    outputDir.value = dir;
    if (persist) persistSetting('output_dir', dir);
  }

  function addExcludedCategory(name: string) {
    const next = new Set(excludedCategories.value);
    next.add(name);
    excludedCategories.value = next;
  }

  function toggleCategory(name: string) {
    const next = new Set(excludedCategories.value);
    if (next.has(name)) {
      next.delete(name);
    }
    else {
      next.add(name);
    }
    excludedCategories.value = next;
    persistSetting('excluded_categories', JSON.stringify([...next]));
  }

  function setLinkStructure(structure: LinkStructure, persist = true) {
    linkStructure.value = structure;
    if (persist) persistSetting('link_structure', structure);
  }

  function setGenerationResult(count: number) {
    linkedCount.value = count;
    lastGeneratedAt.value = new Date().toLocaleString();
    isGenerating.value = false;
    persistSetting('linked_count', String(count));
    persistSetting('last_generated_at', lastGeneratedAt.value);
  }

  function startGenerating() {
    isGenerating.value = true;
  }

  function persistSetting(key: string, value: string) {
    saveSetting(key, value).catch((err: unknown) => {
      console.error(`Failed to persist setting ${key}:`, err);
    });
  }

  return {
    outputDir,
    excludedCategories,
    linkStructure,
    isGenerating,
    linkedCount,
    lastGeneratedAt,
    hasOutputDir,
    setOutputDir,
    addExcludedCategory,
    toggleCategory,
    setLinkStructure,
    setGenerationResult,
    startGenerating,
  };
});
