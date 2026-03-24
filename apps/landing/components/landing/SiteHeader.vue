<script setup lang="ts">
const isScrolled = ref(false);
const { triggerDownload } = useGithubRelease();

onMounted(() => {
  const handleScroll = () => {
    isScrolled.value = window.scrollY > 10;
  };
  window.addEventListener('scroll', handleScroll, { passive: true });
  onUnmounted(() => window.removeEventListener('scroll', handleScroll));
});
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

      <UButton
        color="neutral"
        variant="outline"
        label="Télécharger"
        icon="i-lucide-download"
        class="px-5 py-2.5 rounded-full text-sm font-medium"
        @click="triggerDownload"
      />
    </nav>
  </header>
</template>
