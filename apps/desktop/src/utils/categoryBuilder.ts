import type { Sample, Category } from '@/types/sample';
import { getCategoryColorHex } from '@/utils/categoryColors';

export function buildCategoriesFromSamples(samples: Sample[]): Category[] {
  const map = new Map<string, { subcategories: string[]; count: number }>();

  for (const sample of samples) {
    if (sample.category === 'unknown') continue;

    const entry = map.get(sample.category) ?? { subcategories: [], count: 0 };
    entry.count++;
    if (sample.subcategory && !entry.subcategories.includes(sample.subcategory)) {
      entry.subcategories.push(sample.subcategory);
    }
    map.set(sample.category, entry);
  }

  const categories: Category[] = [...map.entries()].map(([name, { subcategories, count }]) => ({
    name,
    color: getCategoryColorHex(name),
    count,
    subcategories,
  }));

  categories.sort((a, b) => b.count - a.count);
  return categories;
}
