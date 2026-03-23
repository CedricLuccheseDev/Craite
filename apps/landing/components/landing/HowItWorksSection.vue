<template>
  <section id="comment-ca-marche" class="section-light px-6 py-24">
    <div class="max-w-5xl mx-auto">
      <p class="text-center text-sm uppercase tracking-widest text-zinc-400 mb-4 font-medium">Comment ça marche</p>
      <h2 class="text-3xl sm:text-4xl font-bold text-center mb-6 text-zinc-900">
        3 étapes. Tes samples sont enfin organisés.
      </h2>
      <p class="text-center text-zinc-500 mb-20 max-w-xl mx-auto">Pas de configuration. Pas de compte. Pas de cloud.</p>

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-10">
        <div v-for="step in steps" :key="step.number" class="text-center">
          <div class="w-14 h-14 mx-auto mb-6 rounded-2xl bg-[#ff6b35]/10 flex items-center justify-center">
            <UIcon :name="step.icon" class="text-[#ff6b35] text-2xl" />
          </div>
          <div
            class="inline-flex items-center justify-center w-7 h-7 rounded-full bg-[#ff6b35] text-white text-xs font-bold mb-4"
          >
            {{ step.number }}
          </div>
          <h3 class="text-lg font-semibold mb-3 text-zinc-900">
            {{ step.title }}
          </h3>
          <p class="text-zinc-500 leading-relaxed text-sm">
            {{ step.description }}
          </p>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const steps = [
  {
    number: 1,
    icon: 'i-lucide-download',
    title: 'Installe CrAIte',
    description: "L'app détecte automatiquement tes dossiers de samples et de projets. Rien à configurer.",
  },
  {
    number: 2,
    icon: 'i-lucide-scan',
    title: 'Lance le scan',
    description:
      'CrAIte identifie tes samples par nom de fichier et par les métadonnées. Des centaines classés en quelques secondes.',
  },
  {
    number: 3,
    icon: 'i-lucide-music-2',
    title: 'Ouvre ton DAW',
    description: 'Ajoute "CrAIte Library" à tes search paths. Kicks, snares, pads, vocals — tout est là, organisé.',
  },
];
</script>
