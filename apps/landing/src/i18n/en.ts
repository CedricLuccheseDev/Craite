import type { Messages } from './fr'

export const en: Messages = {
  nav: {
    how: 'How it works',
    faq: 'FAQ',
    download: 'Download',
  },
  hero: {
    badge: 'Free, open source, fully local',
    title: 'Stop hunting for samples across 40 folders.',
    intro:
      'craite sorts your samples by kind of sound and makes them reachable straight from your DAW.',
    download: 'Download for free',
    seeHow: 'See how it works',
    beforeAfter: 'From this to this, in seconds',
    before: 'Before',
    after: 'After',
    beforeCaption: 'Loose folders, unreadable file names',
    afterCaption: 'A library sorted by kind of sound',
  },
  how: {
    label: 'How it works',
    title: 'Three steps. Your samples are finally sorted.',
    intro: 'No setup. No account. No cloud.',
  },
  steps: {
    install: {
      title: 'Install craite',
      description:
        'The app finds your sample and project folders on its own. Nothing to configure.',
    },
    scan: {
      title: 'Run the scan',
      description:
        'craite identifies your samples from file names and metadata. Hundreds sorted in seconds.',
    },
    daw: {
      title: 'Open your DAW',
      description:
        'Add the craite library to your search paths. Kicks, snares, pads, vocals, all there, sorted.',
    },
  },
  trust: {
    label: 'With confidence',
    title: 'Built so you have nothing to fear.',
    intro: 'craite moves nothing, changes nothing, collects nothing.',
  },
  trustPoints: {
    inDaw: {
      title: 'Straight into your DAW',
      description:
        'craite builds a real folder tree on your disk, visible in your DAW file browser. No plugin, no second app to keep open.',
    },
    noMove: {
      title: 'Your files stay put',
      description:
        'craite creates hardlinks to your original samples. No extra disk space, and your files never leave their place.',
    },
    free: {
      title: 'Free, no subscription',
      description:
        'No hidden paid tier, no subscription, no account. Fully local, nothing ever leaves your machine.',
    },
  },
  daws: {
    label: 'Works with',
    title: 'Every DAW that can read a folder.',
  },
  faq: {
    label: 'FAQ',
    title: 'Common questions',
  },
  faqItems: {
    price: {
      question: 'Is it really free?',
      answer:
        'Yes, entirely free. No hidden paid tier, no subscription, no account required.',
    },
    moved: {
      question: 'Are my samples moved?',
      answer:
        'No. craite creates hardlinks to the original files. Nothing is copied or moved.',
    },
    projects: {
      question: 'Will it break my projects?',
      answer:
        'No. craite never touches the original files. Your projects keep pointing at the same samples in the same places.',
    },
    os: {
      question: 'Which systems does craite run on?',
      answer:
        'craite runs on Windows, macOS and Linux, and works the same way on all three.',
    },
  },
  download: {
    label: 'Download',
    title: 'You own hundreds of samples you never use.',
    lead: 'Not because they are bad. Because you cannot find them.',
    pitch:
      'craite sorts every one of them. In seconds. Straight into your DAW.',
    version: 'Version',
    soon: 'Coming soon',
    badges: ['Free', 'Cross-platform', 'Fully local'],
  },
  footer: {
    tagline: 'Sort your samples by kind of sound, straight into your DAW.',
    product: 'Product',
    resources: 'Resources',
    github: 'GitHub',
    issues: 'Report a bug',
    releases: 'Changelog',
    rights: 'Free, open source, fully local.',
    localeSwitch: 'Lire en français',
  },
  errors: {
    title: 'Something went wrong',
    tryReload: 'Try reloading the page.',
    reload: 'Reload',
    pageNotFound: 'This page does not exist.',
    backHome: 'Back home',
  },
}
