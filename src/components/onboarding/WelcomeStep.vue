<script setup lang="ts">
const emit = defineEmits<{ next: [] }>();
</script>

<template>
  <div class="welcome-step slide-up">
    <h1 class="logo">CrAIte</h1>
    <p class="tagline">Your AI-powered sample organizer</p>
    <UButton
      color="primary"
      variant="solid"
      size="xl"
      class="cta-button"
      @click="emit('next')"
    >
      C'est parti
    </UButton>
  </div>
</template>

<style scoped>
.welcome-step {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-lg);
  z-index: 1;
}

.logo {
  font-size: 64px;
  font-weight: 800;
  letter-spacing: -2px;
  background: linear-gradient(135deg, var(--color-accent-orange), #ff9a5c);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.tagline {
  font-size: 18px;
  color: var(--color-text-muted);
}

.cta-button {
  margin-top: var(--space-xl);
}
</style>
