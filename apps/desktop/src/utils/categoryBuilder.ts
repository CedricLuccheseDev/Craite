import type { Sample, Category } from '@/types/sample';
import { getCategoryColorHex } from '@/utils/categoryColors';

export function buildCategoriesFromSamples(samples: Sample[]): Category[] {
  const map = new Map<string, { subcategories: string[]; count: number }>();
  let unknownCount = 0;

  for (const sample of samples) {
    if (sample.category === 'unknown') {
      unknownCount++;
      continue;
    }

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

  // Unknown appended last so it appears at the bottom of the sidebar
  if (unknownCount > 0) {
    categories.push({
      name: 'unknown',
      color: getCategoryColorHex('unknown'),
      count: unknownCount,
      subcategories: [],
    });
  }

  return categories;
}
