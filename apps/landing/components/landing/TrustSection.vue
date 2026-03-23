<template>
  <section class="section-dark px-6 py-24">
    <div class="max-w-5xl mx-auto">
      <p class="text-center text-sm uppercase tracking-widest text-zinc-500 mb-4 font-medium">En toute confiance</p>
      <h2 class="text-3xl sm:text-4xl font-bold text-center mb-6">Conçu pour que tu n'aies rien à craindre.</h2>
      <p class="text-center text-zinc-400 mb-20 max-w-xl mx-auto">
        CrAIte ne déplace rien, ne modifie rien, ne collecte rien.
      </p>

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-8">
        <div
          v-for="point in points"
          :key="point.title"
          class="rounded-2xl border border-white/10 bg-white/3 p-8 text-center transition-all duration-300 hover:-translate-y-1"
        >
          <div class="w-12 h-12 mx-auto mb-5 rounded-xl bg-[#ff6b35]/10 flex items-center justify-center">
            <UIcon :name="point.icon" class="text-[#ff6b35] text-2xl" />
          </div>
          <h3 class="font-semibold mb-3 text-lg">
            {{ point.title }}
          </h3>
          <p class="text-zinc-400 text-sm leading-relaxed">
            {{ point.description }}
          </p>
        </div>
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
      'CrAIte crée des liens intelligents (hardlinks) vers tes samples originaux. Zéro espace disque supplémentaire, tes fichiers restent où ils sont.',
  },
  {
    icon: 'i-lucide-lock',
    title: 'Gratuit, sans abonnement',
    description:
      "Pas de version payante cachée, pas d'abonnement, pas de compte requis. 100% local, aucune donnée ne sort de ton PC.",
  },
];
</script>
