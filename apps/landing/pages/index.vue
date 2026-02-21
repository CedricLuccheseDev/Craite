<script setup lang="ts">
useHead({
  title: 'CrAIte — Arrête de chercher tes samples dans 40 dossiers.',
  meta: [
    {
      name: 'description',
      content: 'CrAIte organise tes samples par type de son et les rend accessibles directement dans ton DAW — sans rien déplacer, sans rien casser.',
    },
  ],
});
</script>

<template>
  <div class="min-h-screen bg-[#0a0a0a] text-white">
    <LandingSiteHeader />

    <LandingHeroSection />

    <LandingScrollReveal id="comment-ca-marche">
      <LandingHowItWorksSection />
    </LandingScrollReveal>

    <LandingScrollReveal>
      <LandingTrustSection />
    </LandingScrollReveal>

    <LandingScrollReveal id="faq">
      <LandingFaqSection />
    </LandingScrollReveal>

    <LandingScrollReveal>
      <LandingCloserSection />
    </LandingScrollReveal>

    <LandingSiteFooter />
  </div>
</template>
