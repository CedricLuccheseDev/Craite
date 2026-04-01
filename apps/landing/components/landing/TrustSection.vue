<template>
  <section class="section-dark px-6 py-24">
    <div class="max-w-5xl mx-auto">
      <p class="text-center text-sm uppercase tracking-widest text-zinc-500 mb-4 font-medium">
        {{ t('trust.sectionLabel') }}
      </p>
      <h2 class="text-3xl sm:text-4xl font-bold text-center mb-6">{{ t('trust.title') }}</h2>
      <p class="text-center text-zinc-400 mb-20 max-w-xl mx-auto">
        {{ t('trust.subtitle') }}
      </p>

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-8">
        <div
          v-for="point in points"
          :key="point.title"
          class="rounded-2xl border border-white/10 bg-white/3 p-8 text-center transition-all duration-300 hover:-translate-y-1"
        >
          <div class="w-12 h-12 mx-auto mb-5 rounded-xl bg-[#ff6b35]/10 flex items-center justify-center">
            <UIcon :name="point.icon" class="text-[#ff6b35] text-2xl" />
          </div>
          <h3 class="font-semibold mb-3 text-lg">
            {{ point.title }}
          </h3>
          <p class="text-zinc-400 text-sm leading-relaxed">
            {{ point.description }}
          </p>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const { t } = useI18n();

const points = computed(() => [
  {
    icon: 'i-lucide-music-2',
    title: t('trust.card1Title'),
    description: t('trust.card1Desc'),
  },
  {
    icon: 'i-lucide-shield-check',
    title: t('trust.card2Title'),
    description: t('trust.card2Desc'),
  },
  {
    icon: 'i-lucide-lock',
    title: t('trust.card3Title'),
    description: t('trust.card3Desc'),
  },
]);
</script>
