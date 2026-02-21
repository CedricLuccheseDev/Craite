import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { LinkStructure } from '@/types/sample';

export const useLibraryConfigStore = defineStore('libraryConfig', () => {
  const outputDir = ref<string | null>(null);
  const excludedCategories = ref<Set<string>>(new Set());
  const linkStructure = ref<LinkStructure>('nested');
  const isGenerating = ref(false);
  const linkedCount = ref(0);
  const lastGeneratedAt = ref<string | null>(null);

  const hasOutputDir = computed(() => outputDir.value !== null);

  function setOutputDir(dir: string) {
    outputDir.value = dir;
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
  }

  function setLinkStructure(structure: LinkStructure) {
    linkStructure.value = structure;
  }

  function setGenerationResult(count: number) {
    linkedCount.value = count;
    lastGeneratedAt.value = new Date().toLocaleString();
    isGenerating.value = false;
  }

  function startGenerating() {
    isGenerating.value = true;
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
    toggleCategory,
    setLinkStructure,
    setGenerationResult,
    startGenerating,
  };
});
