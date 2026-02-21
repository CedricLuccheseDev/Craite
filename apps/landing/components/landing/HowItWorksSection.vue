<template>
  <section class="px-6 py-24 bg-zinc-900/30">
    <div class="max-w-4xl mx-auto">
      <h2 class="text-3xl font-bold text-center mb-4">
        3 clics. Tes samples sont organisés.
      </h2>
      <p class="text-center text-zinc-400 mb-16">
        Pas de configuration. Pas de compte. Pas de cloud.
      </p>

      <div class="flex flex-col gap-12">
        <div
          v-for="step in steps"
          :key="step.number"
          class="flex gap-8 items-start"
        >
          <div class="flex-shrink-0 w-14 h-14 rounded-full bg-[#ff6b35]/10 border border-[#ff6b35]/20 flex items-center justify-center">
            <span class="text-[#ff6b35] font-bold text-xl">{{ step.number }}</span>
          </div>
          <div class="pt-2">
            <h3 class="text-xl font-semibold mb-2">
              {{ step.title }}
            </h3>
            <p class="text-zinc-400 leading-relaxed">
              {{ step.description }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const steps = [
  {
    number: 1,
    title: 'Installe CrAIte',
    description: "L'app détecte automatiquement tes dossiers de samples et de projets. Rien à configurer.",
  },
  {
    number: 2,
    title: 'Lance le scan',
    description: 'CrAIte identifie tes samples par nom de fichier et par les métadonnées disponibles. Des centaines de samples classés en quelques secondes.',
  },
  {
    number: 3,
    title: 'Ouvre ton DAW',
    description: 'Ajoute "CrAIte Library" à tes search paths. Tes kicks, snares, pads, vocals — tout est là, organisé.',
  },
];
</script>
