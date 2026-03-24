<template>
  <section class="relative px-6 pt-32 pb-20 overflow-hidden">
    <!-- Vanta waves background -->
    <ClientOnly>
      <LandingVantaBackground />
    </ClientOnly>

    <div class="relative z-10 max-w-5xl mx-auto">
      <!-- Badge -->
      <div class="flex justify-center mb-8 fade-in-up" style="animation-delay: 0ms">
        <span
          class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-white/5 border border-white/10 text-sm text-zinc-300"
        >
          <UIcon name="i-lucide-sparkles" class="text-[#ff6b35]" />
          Gratuit, open source, 100% local
        </span>
      </div>

      <!-- Headline -->
      <h1
        class="text-4xl sm:text-6xl font-bold tracking-tight text-center leading-tight fade-in-up"
        style="animation-delay: 80ms"
      >
        Arrête de chercher tes samples<br />
        dans 40&nbsp;dossiers.
      </h1>

      <p class="mt-6 text-center text-lg text-zinc-400 max-w-2xl mx-auto fade-in-up" style="animation-delay: 160ms">
        CrAIte organise tes samples par type de son et les rend accessibles directement dans ton DAW.
      </p>

      <!-- CTA buttons -->
      <div
        class="mt-10 flex flex-col sm:flex-row items-center justify-center gap-4 fade-in-up"
        style="animation-delay: 240ms"
      >
        <UButton
          color="primary"
          label="Télécharger gratuitement"
          icon="i-lucide-download"
          class="px-8 py-4 rounded-full text-base font-semibold"
          @click="triggerDownload"
        />
        <UButton
          color="neutral"
          variant="ghost"
          label="Voir comment ça marche"
          icon="i-lucide-arrow-down"
          trailing
          class="px-8 py-4 rounded-full text-base"
          @click="scrollToSteps"
        />
      </div>

      <!-- Before / After -->
      <div class="mt-20 fade-in-up" style="animation-delay: 400ms">
        <p class="text-center text-sm text-zinc-500 uppercase tracking-widest mb-8 font-medium">
          De ceci &hellip; à ceci, en quelques secondes
        </p>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
          <!-- Before -->
          <div class="rounded-2xl border border-white/10 bg-white/3 p-6 backdrop-blur-sm">
            <div class="flex items-center gap-2 mb-4">
              <span class="w-3 h-3 rounded-full bg-red-500/80" />
              <span class="text-sm text-zinc-400 font-medium">Avant</span>
            </div>
            <div class="rounded-xl bg-zinc-900 aspect-video flex items-center justify-center">
              <div class="text-center px-6">
                <UIcon name="i-lucide-folder-tree" class="text-4xl text-zinc-600 mb-3" />
                <p class="text-zinc-500 text-sm">Screenshot dossiers en vrac</p>
              </div>
            </div>
          </div>

          <!-- After -->
          <div class="rounded-2xl border border-[#ff6b35]/20 bg-[#ff6b35]/3 p-6 backdrop-blur-sm">
            <div class="flex items-center gap-2 mb-4">
              <span class="w-3 h-3 rounded-full bg-emerald-500/80" />
              <span class="text-sm text-zinc-400 font-medium">Après</span>
            </div>
            <div class="rounded-xl bg-zinc-900 aspect-video flex items-center justify-center">
              <div class="text-center px-6">
                <UIcon name="i-lucide-music-2" class="text-4xl text-[#ff6b35]/60 mb-3" />
                <p class="text-zinc-500 text-sm">Screenshot library organisée</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- DAW compatibility -->
      <div class="mt-14 fade-in-up" style="animation-delay: 500ms">
        <p class="text-center text-xs text-zinc-500 uppercase tracking-widest mb-6">Compatible avec</p>
        <div class="flex flex-wrap items-center justify-center gap-8 opacity-50">
          <span v-for="daw in daws" :key="daw.name" class="relative group">
            <img :src="daw.logo" :alt="daw.name" class="h-6 w-auto transition-opacity duration-200 hover:opacity-100" />
            <span
              class="absolute -bottom-8 left-1/2 -translate-x-1/2 px-2 py-1 rounded-md bg-zinc-800 text-[11px] text-zinc-300 whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity duration-150 pointer-events-none"
            >
              {{ daw.name }}
            </span>
          </span>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const { triggerDownload } = useGithubRelease();

const daws = [
  { name: 'FL Studio', logo: '/daws/flstudio.svg' },
  { name: 'Ableton Live', logo: '/daws/ableton.svg' },
  { name: 'Logic Pro', logo: '/daws/logicpro.svg' },
  { name: 'Bitwig Studio', logo: '/daws/bitwig.svg' },
  { name: 'Studio One', logo: '/daws/studioone.svg' },
  { name: 'Cubase', logo: '/daws/cubase.svg' },
  { name: 'REAPER', logo: '/daws/reaper.svg' },
  { name: 'Renoise', logo: '/daws/renoise.svg' },
  { name: 'Pro Tools', logo: '/daws/protools.svg' },
];

function scrollToSteps() {
  document.getElementById('comment-ca-marche')?.scrollIntoView({ behavior: 'smooth' });
}
</script>
