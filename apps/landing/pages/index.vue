<script setup lang="ts">
useHead({
  title: 'CrAIte - AI-Powered Sample Organizer',
  meta: [
    { name: 'description', content: 'AI-powered sample organizer for FL Studio producers. Classify and organize your sounds automatically.' },
  ],
});
</script>

<template>
  <div class="min-h-screen bg-[#0a0a0a] text-white">
    <!-- Hero -->
    <section class="flex flex-col items-center justify-center min-h-screen px-6 text-center">
      <h1 class="text-6xl font-bold tracking-tight sm:text-7xl">
        Cr<span class="text-[#ff6b35]">AI</span>te
      </h1>
      <p class="mt-6 max-w-2xl text-lg text-zinc-400 sm:text-xl">
        AI-powered sample organizer for FL Studio producers.
        Scans your sample folders, classifies sounds by type, and creates a mirror folder structure for your DAW.
      </p>
      <div class="mt-10 flex gap-4">
        <UButton
          size="xl"
          color="primary"
          label="Download for Windows"
          icon="i-lucide-download"
        />
        <UButton
          size="xl"
          variant="outline"
          color="neutral"
          label="View on GitHub"
          icon="i-lucide-github"
          to="https://github.com/CedricLuccheseDev/Craite"
          target="_blank"
        />
      </div>
    </section>

    <!-- Features -->
    <section class="px-6 py-24 max-w-5xl mx-auto">
      <h2 class="text-3xl font-bold text-center mb-16">
        How it works
      </h2>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div class="text-center">
          <div class="w-12 h-12 mx-auto mb-4 rounded-lg bg-[#ff6b35]/10 flex items-center justify-center">
            <UIcon
              name="i-lucide-folder-search"
              class="text-[#ff6b35] text-2xl"
            />
          </div>
          <h3 class="text-lg font-semibold mb-2">
            Scan
          </h3>
          <p class="text-zinc-400">
            Point CrAIte to your sample folders and let it discover all your sounds.
          </p>
        </div>
        <div class="text-center">
          <div class="w-12 h-12 mx-auto mb-4 rounded-lg bg-[#ff6b35]/10 flex items-center justify-center">
            <UIcon
              name="i-lucide-brain"
              class="text-[#ff6b35] text-2xl"
            />
          </div>
          <h3 class="text-lg font-semibold mb-2">
            Classify
          </h3>
          <p class="text-zinc-400">
            AI classifies each sample by type &mdash; kicks, snares, pads, leads, and more.
          </p>
        </div>
        <div class="text-center">
          <div class="w-12 h-12 mx-auto mb-4 rounded-lg bg-[#ff6b35]/10 flex items-center justify-center">
            <UIcon
              name="i-lucide-folder-tree"
              class="text-[#ff6b35] text-2xl"
            />
          </div>
          <h3 class="text-lg font-semibold mb-2">
            Organize
          </h3>
          <p class="text-zinc-400">
            Creates a clean mirror structure via hardlinks for use in FL Studio&apos;s browser.
          </p>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer class="border-t border-zinc-800 py-8 text-center text-zinc-500 text-sm">
      <p>&copy; {{ new Date().getFullYear() }} CrAIte. Built with Tauri, Vue, and Rust.</p>
    </footer>
  </div>
</template>
