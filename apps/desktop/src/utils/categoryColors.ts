/// Centralized category color mapping used across components and builders.
/// Group-based: all subcategories within a group share the same color.
/// CSS variable version for components, hex version for data builders.

export const CATEGORY_COLORS_CSS: Record<string, string> = {
  // Drums
  kick: 'var(--color-kick)',
  snare: 'var(--color-snare)',
  hihat: 'var(--color-hihat)',
  clap: 'var(--color-clap)',
  tom: 'var(--color-tom)',
  cymbal: 'var(--color-cymbal)',
  perc: 'var(--color-perc)',
  // Bass
  bass: 'var(--color-bass)',
  // Melodic
  pad: 'var(--color-pad)',
  lead: 'var(--color-lead)',
  arp: 'var(--color-arp)',
  chord: 'var(--color-chord)',
  keys: 'var(--color-keys)',
  guitar: 'var(--color-guitar)',
  strings: 'var(--color-strings)',
  brass: 'var(--color-brass)',
  // Vocal
  vocal: 'var(--color-vocal)',
  // FX
  fx: 'var(--color-fx)',
  // Loops
  loop: 'var(--color-loop)',
  // Fallback
  unknown: 'var(--color-muted)',
};

export const CATEGORY_COLORS_HEX: Record<string, string> = {
  // Drums
  kick: '#ff6b35',
  snare: '#ff6b35',
  hihat: '#ff6b35',
  clap: '#ff6b35',
  tom: '#ff6b35',
  cymbal: '#ff6b35',
  perc: '#ff6b35',
  // Bass
  bass: '#ef4444',
  // Melodic
  pad: '#4ade80',
  lead: '#4ade80',
  arp: '#4ade80',
  chord: '#4ade80',
  keys: '#4ade80',
  guitar: '#4ade80',
  strings: '#4ade80',
  brass: '#4ade80',
  // Vocal
  vocal: '#f472b6',
  // FX
  fx: '#06b6d4',
  // Loops
  loop: '#818cf8',
  // Fallback
  unknown: '#6b7280',
};

export function getCategoryColor(name: string): string {
  return CATEGORY_COLORS_CSS[name.toLowerCase()] ?? 'var(--color-muted)';
}

export function getCategoryColorHex(name: string): string {
  return CATEGORY_COLORS_HEX[name.toLowerCase()] ?? '#888888';
}
