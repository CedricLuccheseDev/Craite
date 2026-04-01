<template>
  <footer class="section-dark border-t border-zinc-800 py-16 px-6">
    <div class="max-w-5xl mx-auto grid grid-cols-1 sm:grid-cols-3 gap-12">
      <!-- Branding -->
      <div class="flex flex-col gap-4">
        <span class="text-xl font-bold tracking-tight"> Cr<span class="text-[#ff6b35]">AI</span>te </span>
        <p class="text-sm text-zinc-500 leading-relaxed">
          {{ t('footer.tagline') }}
        </p>
      </div>

      <!-- Product -->
      <div class="flex flex-col gap-3">
        <span class="text-xs font-semibold uppercase tracking-widest text-zinc-400 mb-1">{{
          t('footer.product')
        }}</span>
        <button
          class="text-sm text-zinc-500 hover:text-white transition-colors text-left"
          @click="scrollTo('how-it-works')"
        >
          {{ t('header.howItWorks') }}
        </button>
        <button class="text-sm text-zinc-500 hover:text-white transition-colors text-left" @click="scrollTo('faq')">
          {{ t('header.faq') }}
        </button>
        <button
          class="text-sm text-zinc-500 hover:text-white transition-colors text-left"
          @click="scrollTo('download')"
        >
          {{ t('header.download') }}
        </button>
      </div>

      <!-- Resources -->
      <div class="flex flex-col gap-3">
        <span class="text-xs font-semibold uppercase tracking-widest text-zinc-400 mb-1">
          {{ t('footer.resources') }}
        </span>
        <a
          href="https://github.com/CedricLuccheseDev/Craite"
          target="_blank"
          class="text-sm text-zinc-500 hover:text-white transition-colors"
          >GitHub</a
        >
        <NuxtLink :to="localePath('/privacy')" class="text-sm text-zinc-500 hover:text-white transition-colors">
          {{ t('footer.privacy') }}
        </NuxtLink>
      </div>
    </div>

    <!-- Bottom bar -->
    <div
      class="max-w-5xl mx-auto mt-12 pt-8 border-t border-zinc-800 flex flex-col sm:flex-row items-center justify-between gap-4"
    >
      <p class="text-xs text-zinc-600">{{ t('footer.copyright', { year }) }}</p>
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
const { t } = useI18n();
const localePath = useLocalePath();
const year = new Date().getFullYear();

function scrollTo(id: string) {
  document.getElementById(id)?.scrollIntoView({ behavior: 'smooth' });
}
</script>
