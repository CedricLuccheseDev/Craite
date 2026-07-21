import type { FaqId, StepId, TrustId } from '~/content/landing'

// French is the source dictionary here: the product copy was written in French
// first, and en.ts is the translation of it.
export const fr = {
  nav: {
    how: 'Comment ça marche',
    faq: 'FAQ',
    download: 'Télécharger',
  },
  hero: {
    badge: 'Gratuit, open source, 100% local',
    title: 'Arrête de chercher tes samples dans 40 dossiers.',
    intro:
      'craite organise tes samples par type de son et les rend accessibles directement dans ton DAW.',
    download: 'Télécharger gratuitement',
    seeHow: 'Voir comment ça marche',
    beforeAfter: 'De ceci à ceci, en quelques secondes',
    before: 'Avant',
    after: 'Après',
    beforeCaption: 'Des dossiers en vrac, des noms illisibles',
    afterCaption: 'Une bibliothèque rangée par type de son',
  },
  how: {
    label: 'Comment ça marche',
    title: '3 étapes. Tes samples sont enfin organisés.',
    intro: 'Pas de configuration. Pas de compte. Pas de cloud.',
  },
  steps: {
    install: {
      title: 'Installe craite',
      description:
        "L'app détecte automatiquement tes dossiers de samples et de projets. Rien à configurer.",
    },
    scan: {
      title: 'Lance le scan',
      description:
        'craite identifie tes samples par nom de fichier et par métadonnées. Des centaines classés en quelques secondes.',
    },
    daw: {
      title: 'Ouvre ton DAW',
      description:
        'Ajoute la bibliothèque craite à tes search paths. Kicks, snares, pads, vocals, tout est là, organisé.',
    },
  } satisfies Record<StepId, { title: string; description: string }>,
  trust: {
    label: 'En toute confiance',
    title: "Conçu pour que tu n'aies rien à craindre.",
    intro: 'craite ne déplace rien, ne modifie rien, ne collecte rien.',
  },
  trustPoints: {
    inDaw: {
      title: 'Directement dans ton DAW',
      description:
        "craite crée une vraie arborescence sur ton disque, visible dans le navigateur de fichiers de ton DAW. Pas de plugin, pas d'app à ouvrir en parallèle.",
    },
    noMove: {
      title: 'Tes fichiers ne bougent pas',
      description:
        'craite crée des liens intelligents (hardlinks) vers tes samples originaux. Zéro espace disque supplémentaire, tes fichiers restent où ils sont.',
    },
    free: {
      title: 'Gratuit, sans abonnement',
      description:
        "Pas de version payante cachée, pas d'abonnement, pas de compte requis. 100% local, aucune donnée ne sort de ton poste.",
    },
  } satisfies Record<TrustId, { title: string; description: string }>,
  daws: {
    label: 'Compatible avec',
    title: 'Tous les DAWs qui savent lire un dossier.',
  },
  faq: {
    label: 'FAQ',
    title: 'Questions fréquentes',
  },
  faqItems: {
    price: {
      question: "C'est vraiment gratuit ?",
      answer:
        "Oui, complètement gratuit. Pas de version payante cachée, pas d'abonnement, pas de compte requis.",
    },
    moved: {
      question: 'Mes samples sont-ils déplacés ?',
      answer:
        "Non. craite crée des liens (hardlinks) vers les fichiers originaux. Rien n'est copié ni déplacé.",
    },
    projects: {
      question: 'Ça va casser mes projets ?',
      answer:
        'Non. craite ne modifie jamais les fichiers originaux. Tes projets continuent de pointer vers les mêmes samples aux mêmes emplacements.',
    },
    os: {
      question: 'Sur quels systèmes craite fonctionne ?',
      answer:
        "craite est disponible sur Windows, macOS et Linux. L'app fonctionne de la même façon sur tous les systèmes.",
    },
  } satisfies Record<FaqId, { question: string; answer: string }>,
  download: {
    label: 'Télécharger',
    title: "Tu as des centaines de samples que tu n'utilises jamais.",
    lead: "Pas parce qu'ils sont mauvais. Parce que tu ne les retrouves pas.",
    pitch:
      'craite les organise tous. En quelques secondes. Directement dans ton DAW.',
    version: 'Version',
    soon: 'Bientôt disponible',
    badges: ['Gratuit', 'Multi-plateforme', '100% local'],
  },
  footer: {
    tagline: 'Organise tes samples par type de son, directement dans ton DAW.',
    product: 'Produit',
    resources: 'Ressources',
    github: 'GitHub',
    issues: 'Signaler un bug',
    releases: 'Changelog',
    rights: 'Gratuit, open source, 100% local.',
    localeSwitch: 'Read in English',
  },
  errors: {
    title: 'Une erreur est survenue',
    tryReload: 'Essayez de recharger la page.',
    reload: 'Recharger',
    pageNotFound: "Cette page n'existe pas.",
    backHome: "Retour à l'accueil",
  },
}

export type Messages = typeof fr
