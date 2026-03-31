<script setup lang="ts">
const isScrolled = ref(false);

onMounted(() => {
  const handleScroll = () => {
    isScrolled.value = window.scrollY > 10;
  };
  window.addEventListener('scroll', handleScroll, { passive: true });
  onUnmounted(() => window.removeEventListener('scroll', handleScroll));
});

function scrollTo(id: string) {
  document.getElementById(id)?.scrollIntoView({ behavior: 'smooth' });
}
</script>

<template>
  <header
    :class="[
      'fixed top-0 left-0 right-0 z-50 border-b transition-all duration-300',
      isScrolled
        ? 'bg-[#0a0a0a]/90 backdrop-blur-md border-zinc-800/50 shadow-lg shadow-black/20'
        : 'bg-transparent border-transparent',
    ]"
  >
    <nav class="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between">
      <NuxtLink to="/" class="text-xl font-bold tracking-tight"> Cr<span class="text-[#ff6b35]">AI</span>te </NuxtLink>

      <div class="hidden sm:flex items-center gap-8">
        <button class="text-sm text-zinc-400 hover:text-white transition-colors" @click="scrollTo('how-it-works')">
          Comment ça marche
        </button>
        <button class="text-sm text-zinc-400 hover:text-white transition-colors" @click="scrollTo('faq')">FAQ</button>
        <button class="text-sm text-zinc-400 hover:text-white transition-colors" @click="scrollTo('download')">
          Télécharger
        </button>
      </div>

      <UButton
        color="neutral"
        variant="outline"
        label="Télécharger"
        icon="i-lucide-download"
        class="px-5 py-2.5 rounded-full text-sm font-medium sm:hidden"
        @click="scrollTo('download')"
      />
    </nav>
  </header>
</template>
