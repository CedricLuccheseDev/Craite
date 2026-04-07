import type { Category } from '@/types/sample';

export interface CategoryGroup {
  key: string;
  categories: Category[];
}

export const GROUPS: { key: string; keys: string[] }[] = [
  { key: 'drums', keys: ['kick', 'snare', 'hihat', 'clap', 'cymbal', 'tom', 'perc'] },
  { key: 'synths', keys: ['bass', 'lead', 'pad', 'chord', 'arp'] },
  { key: 'instruments', keys: ['keys', 'guitar', 'strings', 'brass'] },
  { key: 'vocal', keys: ['vocal'] },
  { key: 'fx', keys: ['fx'] },
  { key: 'loops', keys: ['loop'] },
];

export function groupCategories(categories: Category[]): CategoryGroup[] {
  const known = categories.filter(c => c.name !== 'unknown');
  const allGrouped = GROUPS.flatMap(g => g.keys);
  const ungrouped = known.filter(c => !allGrouped.includes(c.name.toLowerCase()));

  const groups = GROUPS.map(g => ({
    key: g.key,
    categories: known.filter(c => g.keys.includes(c.name.toLowerCase())),
  })).filter(g => g.categories.length > 0);

  if (ungrouped.length > 0) {
    groups.push({ key: 'other', categories: ungrouped });
  }

  return groups;
}
