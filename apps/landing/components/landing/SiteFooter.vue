<template>
  <footer class="section-dark border-t border-zinc-800 py-16 px-6">
    <div class="max-w-5xl mx-auto grid grid-cols-1 sm:grid-cols-4 gap-12">
      <!-- Branding -->
      <div class="sm:col-span-1 flex flex-col gap-4">
        <span class="text-xl font-bold tracking-tight"> Cr<span class="text-[#ff6b35]">AI</span>te </span>
        <p class="text-sm text-zinc-500 leading-relaxed">
          Organise tes samples par type de son, directement dans ton DAW.
        </p>
      </div>

      <!-- Produit -->
      <div class="flex flex-col gap-3">
        <span class="text-xs font-semibold uppercase tracking-widest text-zinc-400 mb-1">Produit</span>
        <a href="#how-it-works" class="text-sm text-zinc-500 hover:text-white transition-colors">Comment ça marche</a>
        <a href="#faq" class="text-sm text-zinc-500 hover:text-white transition-colors">FAQ</a>
        <a href="#download" class="text-sm text-zinc-500 hover:text-white transition-colors">Télécharger</a>
      </div>

      <!-- Ressources -->
      <div class="flex flex-col gap-3">
        <span class="text-xs font-semibold uppercase tracking-widest text-zinc-400 mb-1">Ressources</span>
        <a
          href="https://github.com/CedricLuccheseDev/Craite"
          target="_blank"
          class="text-sm text-zinc-500 hover:text-white transition-colors"
          >GitHub</a
        >
        <a
          href="https://github.com/CedricLuccheseDev/Craite/issues"
          target="_blank"
          class="text-sm text-zinc-500 hover:text-white transition-colors"
          >Signaler un bug</a
        >
        <a
          href="https://github.com/CedricLuccheseDev/Craite/releases"
          target="_blank"
          class="text-sm text-zinc-500 hover:text-white transition-colors"
          >Changelog</a
        >
      </div>

      <!-- Compatibilité -->
      <div class="flex flex-col gap-3">
        <span class="text-xs font-semibold uppercase tracking-widest text-zinc-400 mb-1">Compatible avec</span>
        <span class="text-sm text-zinc-500">FL Studio</span>
        <span class="text-sm text-zinc-500">Ableton Live</span>
        <span class="text-sm text-zinc-500">Logic Pro</span>
        <span class="text-sm text-zinc-500">+ 6 autres DAWs</span>
      </div>
    </div>

    <!-- Bottom bar -->
    <div
      class="max-w-5xl mx-auto mt-12 pt-8 border-t border-zinc-800 flex flex-col sm:flex-row items-center justify-between gap-4"
    >
      <p class="text-xs text-zinc-600">&copy; {{ year }} CrAIte. Gratuit, open source, 100% local.</p>
      <div class="flex items-center gap-4">
        <a
          href="https://github.com/CedricLuccheseDev/Craite"
          target="_blank"
          class="text-zinc-600 hover:text-white transition-colors"
        >
          <UIcon name="i-lucide-github" class="text-lg" />
        </a>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
const year = new Date().getFullYear();
</script>
