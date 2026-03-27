const icons: Record<string, string> = {
  kick: 'i-lucide-circle-dot',
  snare: 'i-lucide-drum',
  hihat: 'i-lucide-disc',
  clap: 'i-lucide-hand',
  cymbal: 'i-lucide-circle',
  tom: 'i-lucide-disc-3',
  perc: 'i-lucide-triangle',
  bass: 'i-lucide-audio-waveform',
  pad: 'i-lucide-waves',
  lead: 'i-lucide-audio-lines',
  arp: 'i-lucide-repeat',
  chord: 'i-lucide-music-3',
  keys: 'i-lucide-piano',
  guitar: 'i-lucide-guitar',
  strings: 'i-lucide-music-2',
  brass: 'i-lucide-megaphone',
  vocal: 'i-lucide-mic-vocal',
  fx: 'i-lucide-sparkles',
  loop: 'i-lucide-repeat-2',
};

export function getCategoryIcon(name: string): string {
  return icons[name.toLowerCase()] ?? 'i-lucide-music';
}
