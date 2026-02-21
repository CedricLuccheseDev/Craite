<template>
  <section class="px-6 py-24 max-w-4xl mx-auto">
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
        class="text-center"
      >
        <div class="w-12 h-12 mx-auto mb-4 rounded-lg bg-[#ff6b35]/10 flex items-center justify-center">
          <UIcon
            :name="point.icon"
            class="text-[#ff6b35] text-2xl"
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
      "CrAIte crée une vraie arborescence sur ton disque, visible dans le browser FL. Pas de plugin, pas d'app à ouvrir en parallèle.",
  },
  {
    icon: 'i-lucide-shield-check',
    title: 'Tes fichiers ne bougent pas',
    description:
      'Hardlinks, pas de copies. Zéro espace disque supplémentaire. Tes projets FL continuent de pointer vers les mêmes samples.',
  },
  {
    icon: 'i-lucide-github',
    title: 'Gratuit et open source',
    description:
      "Licence MIT. Pas de version payante cachée, pas d'abonnement, pas de compte. 100% local, aucune donnée ne sort de ton PC.",
  },
];
</script>
