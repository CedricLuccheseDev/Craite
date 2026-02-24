<template>
  <section class="px-6 py-24 max-w-4xl mx-auto trust-section relative">
    <h2 class="text-3xl font-bold text-center mb-4">
      Conçu pour que tu n'aies rien à craindre.
    </h2>
    <p class="text-center text-zinc-400 mb-16 max-w-xl mx-auto">
      CrAIte ne déplace rien, ne modifie rien, ne collecte rien.
    </p>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-8">
      <div
        v-for="point in points"
        :key="point.title"
        class="text-center trust-card"
      >
        <div class="w-12 h-12 mx-auto mb-4 rounded-lg bg-[#ff6b35]/10 flex items-center justify-center trust-icon-wrapper">
          <UIcon
            :name="point.icon"
            class="text-[#ff6b35] text-2xl trust-icon"
          />
        </div>
        <h3 class="font-semibold mb-2">
          {{ point.title }}
        </h3>
        <p class="text-zinc-400 text-sm leading-relaxed">
          {{ point.description }}
        </p>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const points = [
  {
    icon: 'i-lucide-music-2',
    title: 'Directement dans ton DAW',
    description:
      "CrAIte crée une vraie arborescence sur ton disque, visible dans le file browser de ton DAW. Pas de plugin, pas d'app à ouvrir en parallèle.",
  },
  {
    icon: 'i-lucide-shield-check',
    title: 'Tes fichiers ne bougent pas',
    description:
      'CrAIte crée des liens intelligents (hardlinks) vers tes samples originaux — comme des raccourcis, mais mieux. Zéro espace disque supplémentaire, tes fichiers restent exactement où ils sont.',
  },
  {
    icon: 'i-lucide-shield-check',
    title: 'Gratuit, sans abonnement',
    description:
      "Pas de version payante cachée, pas d'abonnement, pas de compte requis. 100% local, aucune donnée ne sort de ton PC.",
  },
];
</script>
