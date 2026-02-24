<template>
  <section class="px-6 py-24 bg-zinc-900/30 how-it-works-bg relative">
    <div class="max-w-4xl mx-auto">
      <h2 class="text-3xl font-bold text-center mb-4">
        3 étapes. Tes samples sont enfin organisés.
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
          <!-- Icon container with pulse background -->
          <div class="relative shrink-0 w-20 h-20">
            <!-- Pulse background -->
            <div class="absolute inset-0 rounded-full bg-[#ff6b35]/10 icon-pulse" />
            <!-- Icon with float animation -->
            <div class="relative w-full h-full rounded-full border border-[#ff6b35]/20 flex items-center justify-center bg-zinc-900/50 backdrop-blur-sm">
              <UIcon
                :name="step.icon"
                class="text-[#ff6b35] text-3xl icon-float"
              />
            </div>
            <!-- Step number badge -->
            <div class="absolute -bottom-1 -right-1 w-6 h-6 rounded-full bg-[#ff6b35] flex items-center justify-center">
              <span class="text-white text-xs font-bold">{{ step.number }}</span>
            </div>
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
    icon: 'i-lucide-download',
    title: 'Installe CrAIte',
    description: "L'app détecte automatiquement tes dossiers de samples et de projets. Rien à configurer.",
  },
  {
    number: 2,
    icon: 'i-lucide-scan',
    title: 'Lance le scan',
    description: 'CrAIte identifie tes samples par nom de fichier et par les métadonnées disponibles. Des centaines de samples classés en quelques secondes.',
  },
  {
    number: 3,
    icon: 'i-lucide-music-2',
    title: 'Ouvre ton DAW',
    description: 'Ajoute "CrAIte Library" à tes search paths. Tes kicks, snares, pads, vocals — tout est là, organisé.',
  },
];
</script>
