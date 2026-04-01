<script setup lang="ts">
useHead({
  title: 'Privacy Policy — CrAIte',
});

useSeoMeta({
  description: 'CrAIte privacy policy. No data collection, no cloud, no accounts. 100% local.',
});
</script>

<template>
  <div class="min-h-screen bg-[#0a0a0a] text-white">
    <LandingSiteHeader />

    <main class="max-w-3xl mx-auto px-6 py-32">
      <h1 class="text-4xl font-bold tracking-tight mb-8">Privacy Policy</h1>
      <p class="text-sm text-zinc-500 mb-12">Last updated: April 2026</p>

      <div class="space-y-10 text-zinc-300 leading-relaxed">
        <section>
          <h2 class="text-xl font-semibold text-white mb-4">The short version</h2>
          <p>
            CrAIte does not collect, store, or transmit any of your data. Everything runs locally on your machine. There
            are no accounts, no cloud, no analytics inside the app, and no network calls.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-white mb-4">What CrAIte does</h2>
          <ul class="list-disc list-inside space-y-2 text-zinc-400">
            <li>Scans audio files in folders you explicitly select</li>
            <li>Classifies samples by filename and audio characteristics</li>
            <li>Creates an organized mirror of your library using hardlinks</li>
            <li>Stores its database locally on your device</li>
          </ul>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-white mb-4">What CrAIte does NOT do</h2>
          <ul class="list-disc list-inside space-y-2 text-zinc-400">
            <li>Does not send any data to external servers</li>
            <li>Does not require an account or login</li>
            <li>Does not include telemetry, analytics, or tracking in the app</li>
            <li>Does not move, copy, or modify your original files</li>
            <li>Does not access files outside the folders you select</li>
          </ul>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-white mb-4">Auto-update</h2>
          <p>
            CrAIte checks for updates by contacting our update server. This request contains only the current app
            version and your operating system. No personal data is transmitted.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-white mb-4">Landing page</h2>
          <p>
            This website uses
            <a href="https://posthog.com" target="_blank" class="text-[#ff6b35] hover:underline">PostHog</a>
            for product analytics. PostHog is configured without cookies and with in-memory persistence only (no data
            persists across sessions). You can learn more about their privacy approach at
            <a href="https://posthog.com/privacy" target="_blank" class="text-[#ff6b35] hover:underline"
              >posthog.com/privacy</a
            >.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-white mb-4">Open source</h2>
          <p>
            CrAIte is open source under the
            <a
              href="https://github.com/CedricLuccheseDev/Craite/blob/main/LICENSE"
              target="_blank"
              class="text-[#ff6b35] hover:underline"
              >AGPL v3 license</a
            >. You can audit the entire codebase to verify these claims.
          </p>
        </section>

        <section>
          <h2 class="text-xl font-semibold text-white mb-4">Contact</h2>
          <p>
            Questions about this policy? Open an issue on
            <a
              href="https://github.com/CedricLuccheseDev/Craite/issues"
              target="_blank"
              class="text-[#ff6b35] hover:underline"
              >GitHub</a
            >.
          </p>
        </section>
      </div>
    </main>

    <LandingSiteFooter />
  </div>
</template>
