import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Sample, Category } from '@/types/sample';

export const useLibraryStore = defineStore('library', () => {
  const samples = ref<Sample[]>([]);
  const categories = ref<Category[]>([]);
  const selectedCategory = ref<string | null>(null);
  const searchQuery = ref('');

  const filteredSamples = computed(() => {
    let result = samples.value;

    if (selectedCategory.value) {
      result = result.filter(s => s.category === selectedCategory.value);
    }

    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase();
      result = result.filter(s => s.name.toLowerCase().includes(query));
    }

    return result;
  });

  const sampleCount = computed(() => samples.value.length);

  function setSamples(newSamples: Sample[]) {
    samples.value = newSamples;
  }

  function setCategories(newCategories: Category[]) {
    categories.value = newCategories;
  }

  function selectCategory(name: string | null) {
    selectedCategory.value = name;
  }

  function setSearchQuery(query: string) {
    searchQuery.value = query;
  }

  return {
    samples,
    categories,
    selectedCategory,
    searchQuery,
    filteredSamples,
    sampleCount,
    setSamples,
    setCategories,
    selectCategory,
    setSearchQuery,
  };
});
