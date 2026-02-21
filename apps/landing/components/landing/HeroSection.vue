<template>
  <section class="relative min-h-screen px-6 flex flex-col items-center justify-center overflow-hidden pt-16 pb-24">
    <!-- Grid background -->
    <div class="absolute inset-0 hero-grid pointer-events-none" />
    <!-- Orange radial glow -->
    <div class="absolute top-0 left-1/2 -translate-x-1/2 w-[700px] h-[500px] rounded-full bg-[#ff6b35]/8 blur-[120px] pointer-events-none" />

    <!-- Hero text -->
    <div class="relative z-10 flex flex-col items-center text-center max-w-4xl mx-auto">
      <h1
        class="text-5xl font-bold tracking-tight sm:text-7xl leading-tight fade-in-up"
        style="animation-delay: 100ms"
      >
        Arrête de chercher tes samples dans 40&nbsp;dossiers.
      </h1>

      <p
        class="mt-6 max-w-2xl text-lg text-zinc-400 sm:text-xl fade-in-up"
        style="animation-delay: 200ms"
      >
        CrAIte organise tes samples par type de son et les rend accessibles directement dans le
        browser — sans rien déplacer, sans rien casser.
      </p>

      <div
        class="mt-10 fade-in-up"
        style="animation-delay: 300ms"
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

      <div
        class="mt-6 flex flex-wrap gap-2 justify-center fade-in-up"
        style="animation-delay: 400ms"
      >
        <UBadge v-for="badge in badges" :key="badge" color="neutral" variant="subtle" :label="badge" />
      </div>
    </div>

    <!-- 3-step visual -->
    <div
      class="relative z-10 mt-20 w-full max-w-5xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-4 fade-in-up"
      style="animation-delay: 500ms"
    >
      <div
        v-for="(step, i) in steps"
        :key="step.number"
        class="flex flex-col gap-3"
      >
        <div class="relative rounded-xl bg-zinc-900/80 border border-zinc-800 aspect-video flex flex-col items-center justify-center gap-3 text-zinc-600 backdrop-blur-sm">
          <UIcon :name="step.icon" class="text-4xl" />
          <span class="text-xs">Screenshot a venir</span>
          <div
            v-if="i < steps.length - 1"
            class="hidden md:flex absolute -right-5 top-1/2 -translate-y-1/2 z-10 w-10 h-10 items-center justify-center"
          >
            <UIcon name="i-lucide-arrow-right" class="text-zinc-600 text-lg" />
          </div>
        </div>
        <div>
          <span class="text-xs font-semibold text-[#ff6b35] uppercase tracking-widest">
            Étape {{ step.number }}
          </span>
          <h3 class="text-base font-semibold mt-1">
            {{ step.title }}
          </h3>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const { triggerDownload } = useGithubRelease();
const badges = ['Gratuit', 'Open source', '100% local', 'Windows'];

const steps = [
  { number: 1, title: 'Avant : le bordel', icon: 'i-lucide-folder-open' },
  { number: 2, title: 'CrAIte organise', icon: 'i-lucide-cpu' },
  { number: 3, title: 'Après : dans ton DAW', icon: 'i-lucide-music-2' },
];
</script>
