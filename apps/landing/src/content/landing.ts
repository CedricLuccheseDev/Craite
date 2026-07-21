// Structural data only. Every string a visitor reads lives in src/i18n,
// keyed by the ids below so a missing translation is a type error.

export type StepId = 'install' | 'scan' | 'daw'
export type TrustId = 'inDaw' | 'noMove' | 'free'
export type FaqId = 'price' | 'moved' | 'projects' | 'os'

export const steps: Array<StepId> = ['install', 'scan', 'daw']
export const trustPoints: Array<TrustId> = ['inDaw', 'noMove', 'free']
export const faqItems: Array<FaqId> = ['price', 'moved', 'projects', 'os']

export type PlatformKey = 'windows' | 'mac_arm' | 'linux'

export const platforms: Array<{ key: PlatformKey; label: string }> = [
  { key: 'windows', label: 'Windows' },
  { key: 'mac_arm', label: 'macOS' },
  { key: 'linux', label: 'Linux' },
]

// Logos live in public/daws, kept from the Nuxt landing.
export const daws = [
  { slug: 'ableton', name: 'Ableton Live' },
  { slug: 'flstudio', name: 'FL Studio' },
  { slug: 'logicpro', name: 'Logic Pro' },
  { slug: 'cubase', name: 'Cubase' },
  { slug: 'studioone', name: 'Studio One' },
  { slug: 'bitwig', name: 'Bitwig' },
  { slug: 'reaper', name: 'Reaper' },
  { slug: 'protools', name: 'Pro Tools' },
  { slug: 'renoise', name: 'Renoise' },
]

export const REPO = 'https://github.com/CedricLuccheseDev/craite'
