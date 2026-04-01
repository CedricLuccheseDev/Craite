<script setup lang="ts">
const { track } = useTrack();
const tracked = new Set<string>();

function onSection(section: string) {
  if (tracked.has(section)) return;
  tracked.add(section);
  track('section_viewed', { section });
}

const { locale } = useI18n();
const url = 'https://craite.clhub.fr';
const ogImage = `${url}/og-image.png`;

const titles: Record<string, string> = {
  en: 'CrAIte — Stop digging through 40 folders for your samples.',
  fr: 'CrAIte — Arrête de chercher tes samples dans 40 dossiers.',
};

const descriptions: Record<string, string> = {
  en: 'CrAIte organizes your samples by sound type and makes them accessible directly in your DAW — without moving, copying, or breaking anything.',
  fr: 'CrAIte organise tes samples par type de son et les rend accessibles directement dans ton DAW — sans rien déplacer, sans rien casser.',
};

const title = computed(() => titles[locale.value] ?? titles.en);
const description = computed(() => descriptions[locale.value] ?? descriptions.en);

useHead({
  title,
  htmlAttrs: { lang: locale },
  link: [{ rel: 'canonical', href: url }],
});

useSeoMeta({
  description,
  ogTitle: title,
  ogDescription: description,
  ogImage,
  ogImageWidth: 1200,
  ogImageHeight: 630,
  ogType: 'website',
  ogUrl: url,
  ogSiteName: 'CrAIte',
  twitterCard: 'summary_large_image',
  twitterTitle: title,
  twitterDescription: description,
  twitterImage: ogImage,
});
</script>

<template>
  <div class="min-h-screen bg-[#0a0a0a] text-white">
    <LandingSiteHeader />

    <LandingHeroSection />

    <LandingScrollReveal @visible="onSection('how_it_works')">
      <LandingHowItWorksSection />
    </LandingScrollReveal>

    <LandingScrollReveal @visible="onSection('trust')">
      <LandingTrustSection />
    </LandingScrollReveal>

    <LandingScrollReveal @visible="onSection('faq')">
      <LandingFaqSection />
    </LandingScrollReveal>

    <LandingScrollReveal @visible="onSection('download')">
      <LandingCloserSection />
    </LandingScrollReveal>

    <LandingSiteFooter />
  </div>
</template>
