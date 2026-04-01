<script setup lang="ts">
const { t, locale, locales } = useI18n();
const switchLocalePath = useSwitchLocalePath();
const localePath = useLocalePath();
const router = useRouter();
const isScrolled = ref(false);

const otherLocale = computed(() => {
  const available = locales.value as Array<{ code: string; name: string }>;
  return available.find(l => l.code !== locale.value);
});

onMounted(() => {
  const handleScroll = () => {
    isScrolled.value = window.scrollY > 10;
  };
  window.addEventListener('scroll', handleScroll, { passive: true });
  onUnmounted(() => window.removeEventListener('scroll', handleScroll));
});

function scrollTo(id: string) {
  const el = document.getElementById(id);
  if (el) {
    el.scrollIntoView({ behavior: 'smooth' });
  } else {
    router.push(localePath('/') + '#' + id);
  }
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
    <nav class="max-w-6xl mx-auto px-6 h-16 grid grid-cols-3 items-center">
      <!-- Left: logo -->
      <div class="flex items-center">
        <NuxtLink :to="localePath('/')" class="text-xl font-bold tracking-tight">
          Cr<span class="text-[#ff6b35]">AI</span>te
        </NuxtLink>
      </div>

      <!-- Center: nav links -->
      <div class="hidden sm:flex items-center justify-center gap-8">
        <button class="text-sm text-zinc-400 hover:text-white transition-colors" @click="scrollTo('how-it-works')">
          {{ t('header.howItWorks') }}
        </button>
        <button class="text-sm text-zinc-400 hover:text-white transition-colors" @click="scrollTo('faq')">
          {{ t('header.faq') }}
        </button>
      </div>
      <div class="sm:hidden" />

      <!-- Right: lang switch + download -->
      <div class="flex items-center justify-end gap-3">
        <NuxtLink
          v-if="otherLocale"
          :to="switchLocalePath(otherLocale.code)"
          class="text-xs text-zinc-500 hover:text-white transition-colors uppercase tracking-wide font-medium border border-white/10 rounded-full px-2.5 py-1"
        >
          {{ otherLocale.code }}
        </NuxtLink>
        <UButton
          color="primary"
          :label="t('header.download')"
          icon="i-lucide-download"
          class="px-5 py-2.5 rounded-full text-sm font-semibold"
          @click="scrollTo('download')"
        />
      </div>
    </nav>
  </header>
</template>
