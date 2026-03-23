<template>
  <section class="section-dark px-6 py-32 text-center">
    <div class="max-w-3xl mx-auto">
      <h2 class="text-4xl sm:text-5xl font-bold mb-8 leading-tight">
        Tu as des centaines de samples<br />que tu n'utilises jamais.
      </h2>
      <p class="text-zinc-400 text-lg leading-relaxed mb-6">
        Pas parce qu'ils sont mauvais. Parce que tu ne les retrouves pas.
      </p>
      <p class="text-zinc-300 text-lg leading-relaxed mb-12">
        <strong class="text-white">CrAIte les organise tous. En quelques secondes. Directement dans ton DAW.</strong>
      </p>

      <UButton
        color="primary"
        label="Télécharger CrAIte gratuitement"
        icon="i-lucide-download"
        class="px-8 py-4 rounded-full text-base font-semibold"
        @click="triggerDownload"
      />

      <div class="mt-8 flex flex-wrap gap-3 justify-center">
        <span
          v-for="badge in badges"
          :key="badge"
          class="text-xs text-zinc-400 px-3 py-1.5 rounded-full border border-white/10 bg-white/3"
        >
          {{ badge }}
        </span>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const { triggerDownload } = useGithubRelease();
const badges = ['Gratuit', 'Multi-plateforme', '100% local'];
</script>
