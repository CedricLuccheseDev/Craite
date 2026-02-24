<template>
  <section class="px-6 py-32 text-center relative overflow-hidden">
    <div class="absolute inset-0 bg-gradient-to-b from-transparent via-[#ff6b35]/5 to-transparent pointer-events-none" />
    <div class="relative max-w-3xl mx-auto">
      <h2 class="text-4xl font-bold mb-6 sm:text-5xl">
        Tu as des centaines de samples que tu n'utilises jamais.
      </h2>
      <p class="text-zinc-400 text-lg leading-relaxed mb-4">
        Pas parce qu'ils sont mauvais. Parce que tu ne les retrouves pas.
      </p>
      <p class="text-zinc-300 text-lg leading-relaxed mb-10">
        Chaque session, tu réutilises les mêmes 20 sons parce que c'est les seuls que tu connais par cœur.
        Le reste dort dans des dossiers que tu ne parcourras jamais.
        <br><br>
        <strong class="text-white">CrAIte les organise tous. En quelques secondes. Directement dans ton DAW.</strong>
      </p>

      <UButton
        size="xl"
        color="primary"
        label="Télécharger CrAIte gratuitement"
        icon="i-lucide-download"
        class="btn-glass mb-8"
        @click="triggerDownload"
      />

      <div class="flex flex-wrap gap-3 justify-center">
        <UBadge
          v-for="badge in badges"
          :key="badge"
          color="neutral"
          variant="subtle"
          :label="badge"
        />
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const { triggerDownload } = useGithubRelease();
const badges = ['Gratuit', 'Multi-plateforme', '100% local'];
</script>
