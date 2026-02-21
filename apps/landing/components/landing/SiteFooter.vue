<template>
  <footer class="border-t border-zinc-800 py-8 px-6">
    <div class="max-w-5xl mx-auto flex flex-col sm:flex-row items-center justify-between gap-4 text-sm text-zinc-500">
      <p>&copy; {{ year }} CrAIte. Gratuit, open source, local.</p>
      <div class="flex gap-6">
        <UButton
          variant="link"
          color="neutral"
          label="GitHub"
          icon="i-lucide-github"
          to="https://github.com/CedricLuccheseDev/Craite"
          target="_blank"
          size="sm"
        />
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
const year = new Date().getFullYear();
</script>
