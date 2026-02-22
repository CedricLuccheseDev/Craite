interface DawIconStyle {
  label: string;
  colors: [string, string];
}

const STYLES: Record<string, DawIconStyle> = {
  fl_studio: { label: 'FL', colors: ['#FF8C00', '#E65100'] },
  ableton: { label: 'Ab', colors: ['#9BE564', '#5B9A1A'] },
  bitwig: { label: 'Bw', colors: ['#FF6E40', '#D84315'] },
  studio_one: { label: 'S1', colors: ['#42A5F5', '#1565C0'] },
  logic: { label: 'LP', colors: ['#AB47BC', '#6A1B9A'] },
  reaper: { label: 'Rp', colors: ['#66BB6A', '#2E7D32'] },
  cubase: { label: 'Cb', colors: ['#EF5350', '#B71C1C'] },
  reason: { label: 'Re', colors: ['#FFA726', '#E65100'] },
  protools: { label: 'PT', colors: ['#7C4DFF', '#4527A0'] },
  renoise: { label: 'Rn', colors: ['#FFAB40', '#BF360C'] },
};

export function getDawIconSvg(dawId: string, size = 36): string {
  const s = STYLES[dawId];
  if (!s) return '';

  const r = Math.round(size * 0.22);
  const fs = Math.round(size * 0.36);

  return `<svg width="${size}" height="${size}" viewBox="0 0 ${size} ${size}" fill="none" xmlns="http://www.w3.org/2000/svg"><defs><linearGradient id="daw-${dawId}" x1="0" y1="0" x2="${size}" y2="${size}" gradientUnits="userSpaceOnUse"><stop stop-color="${s.colors[0]}"/><stop offset="1" stop-color="${s.colors[1]}"/></linearGradient></defs><rect width="${size}" height="${size}" rx="${r}" fill="url(#daw-${dawId})"/><text x="50%" y="53%" dominant-baseline="middle" text-anchor="middle" fill="#fff" font-family="Inter,system-ui,sans-serif" font-weight="700" font-size="${fs}" letter-spacing="-0.5">${s.label}</text></svg>`;
}

export function getCustomFolderSvg(size = 36): string {
  const r = Math.round(size * 0.22);
  const c = size / 2;
  const l = size * 0.22;

  return `<svg width="${size}" height="${size}" viewBox="0 0 ${size} ${size}" fill="none" xmlns="http://www.w3.org/2000/svg"><rect width="${size}" height="${size}" rx="${r}" fill="#27272a" stroke="#3f3f46" stroke-width="1"/><line x1="${c}" y1="${c - l}" x2="${c}" y2="${c + l}" stroke="#71717a" stroke-width="2" stroke-linecap="round"/><line x1="${c - l}" y1="${c}" x2="${c + l}" y2="${c}" stroke="#71717a" stroke-width="2" stroke-linecap="round"/></svg>`;
}
