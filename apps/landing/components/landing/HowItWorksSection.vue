<template>
  <section id="how-it-works" class="section-light px-6 py-24">
    <div class="max-w-5xl mx-auto">
      <p class="text-center text-sm uppercase tracking-widest text-zinc-400 mb-4 font-medium">
        {{ t('howItWorks.sectionLabel') }}
      </p>
      <h2 class="text-3xl sm:text-4xl font-bold text-center mb-6 text-zinc-900">
        {{ t('howItWorks.title') }}
      </h2>
      <p class="text-center text-zinc-500 mb-20 max-w-xl mx-auto">{{ t('howItWorks.subtitle') }}</p>

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-10">
        <div
          v-for="step in steps"
          :key="step.number"
          class="text-center rounded-2xl border border-zinc-200 bg-white p-8 transition-all duration-300 hover:-translate-y-1"
        >
          <div class="w-14 h-14 mx-auto mb-6 rounded-2xl bg-[#ff6b35]/10 flex items-center justify-center">
            <UIcon :name="step.icon" class="text-[#ff6b35] text-2xl" />
          </div>
          <div
            class="inline-flex items-center justify-center w-7 h-7 rounded-full bg-[#ff6b35] text-white text-xs font-bold mb-4"
          >
            {{ step.number }}
          </div>
          <h3 class="text-lg font-semibold mb-3 text-zinc-900">
            {{ step.title }}
          </h3>
          <p class="text-zinc-500 leading-relaxed text-sm">
            {{ step.description }}
          </p>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const { t } = useI18n();

const steps = computed(() => [
  {
    number: 1,
    icon: 'i-lucide-download',
    title: t('howItWorks.step1Title'),
    description: t('howItWorks.step1Desc'),
  },
  {
    number: 2,
    icon: 'i-lucide-scan',
    title: t('howItWorks.step2Title'),
    description: t('howItWorks.step2Desc'),
  },
  {
    number: 3,
    icon: 'i-lucide-music-2',
    title: t('howItWorks.step3Title'),
    description: t('howItWorks.step3Desc'),
  },
]);
</script>
