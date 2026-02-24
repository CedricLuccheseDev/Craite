<template>
  <section class="relative min-h-screen px-6 flex flex-col items-center justify-center overflow-hidden pt-16 pb-24 hero-background">
    <!-- Grid background -->
    <div class="absolute inset-0 hero-grid pointer-events-none" />
    <!-- Enhanced orange radial glow -->
    <div class="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[600px] rounded-full bg-[#ff6b35]/12 blur-[140px] pointer-events-none" />

    <!-- Hero text -->
    <div class="relative z-10 flex flex-col items-center text-center max-w-4xl mx-auto">
      <h1
        class="text-5xl font-bold tracking-tight sm:text-7xl leading-tight fade-in-up"
        style="animation-delay: 50ms"
      >
        Arrête de chercher tes samples dans 40&nbsp;dossiers.
      </h1>

      <p
        class="mt-8 max-w-xl text-lg text-zinc-400 fade-in-up"
        style="animation-delay: 150ms"
      >
        Organise tes samples par type de son et accède-y directement depuis ton DAW.
      </p>

      <div
        class="mt-10 fade-in-up"
        style="animation-delay: 250ms"
      >
        <UButton
          size="xl"
          color="primary"
          label="Télécharger gratuitement"
          icon="i-lucide-download"
          class="btn-glass"
          @click="triggerDownload"
        />
      </div>

      <!-- Waveforms animées -->
      <div
        class="relative mt-16 h-32 flex items-end justify-center gap-2 fade-in-up"
        style="animation-delay: 450ms"
      >
        <div
          v-for="i in 7"
          :key="i"
          class="waveform-bar rounded-full"
          :style="{
            animationDelay: `${i * 0.15}s`,
            height: waveformHeights[i - 1]
          }"
        ></div>
      </div>

      <div
        class="mt-8 flex flex-wrap gap-2 justify-center fade-in-up"
        style="animation-delay: 550ms"
      >
        <UBadge v-for="badge in badges" :key="badge" color="neutral" variant="subtle" :label="badge" class="badge-interactive" />
      </div>
    </div>

  </section>
</template>

<script setup lang="ts">
const { triggerDownload } = useGithubRelease();
const badges = ['Gratuit', 'Multi-plateforme', '100% local'];

// Waveforms heights (centre plus haut)
const waveformHeights = ['40px', '70px', '55px', '90px', '55px', '70px', '40px'];
</script>
