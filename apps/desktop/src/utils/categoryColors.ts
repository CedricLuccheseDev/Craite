/// Centralized category color mapping used across components and builders.
/// CSS variable version for components, hex version for data builders.

export const CATEGORY_COLORS_CSS: Record<string, string> = {
  kick: 'var(--color-kick)',
  snare: 'var(--color-snare)',
  hihat: 'var(--color-hihat)',
  clap: 'var(--color-clap)',
  pad: 'var(--color-pad)',
  lead: 'var(--color-lead)',
  bass: 'var(--color-bass)',
  fx: 'var(--color-fx)',
  vocal: 'var(--color-vocal)',
  loop: 'var(--color-loop)',
};

export const CATEGORY_COLORS_HEX: Record<string, string> = {
  kick: '#ff6b35',
  snare: '#4a9eff',
  hihat: '#fbbf24',
  clap: '#a78bfa',
  pad: '#4ade80',
  lead: '#f472b6',
  bass: '#ef4444',
  fx: '#06b6d4',
  vocal: '#fb923c',
  loop: '#818cf8',
};

export function getCategoryColor(name: string): string {
  return CATEGORY_COLORS_CSS[name.toLowerCase()] ?? 'var(--color-muted)';
}

export function getCategoryColorHex(name: string): string {
  return CATEGORY_COLORS_HEX[name.toLowerCase()] ?? '#888888';
}
