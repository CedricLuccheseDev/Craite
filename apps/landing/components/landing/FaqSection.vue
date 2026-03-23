<template>
  <section class="section-light px-6 py-24">
    <div class="max-w-3xl mx-auto">
      <p class="text-center text-sm uppercase tracking-widest text-zinc-400 mb-4 font-medium">FAQ</p>
      <h2 class="text-3xl sm:text-4xl font-bold text-center mb-16 text-zinc-900">Questions fréquentes</h2>

      <div class="flex flex-col gap-4">
        <details
          v-for="item in faq"
          :key="item.label"
          class="group rounded-2xl border border-zinc-200 bg-white px-6 py-5"
        >
          <summary class="flex items-center justify-between cursor-pointer list-none text-zinc-900 font-medium">
            {{ item.label }}
            <UIcon
              name="i-lucide-chevron-down"
              class="text-zinc-400 transition-transform duration-200 group-open:rotate-180"
            />
          </summary>
          <p class="mt-4 text-zinc-500 leading-relaxed text-sm">
            {{ item.content }}
          </p>
        </details>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const faq = [
  {
    label: "C'est vraiment gratuit ?",
    content: "Oui, complètement gratuit. Pas de version payante cachée, pas d'abonnement, pas de compte requis.",
  },
  {
    label: 'Mes samples sont-ils déplacés ?',
    content: "Non. CrAIte crée des liens (hardlinks) vers les fichiers originaux. Rien n'est copié ni déplacé.",
  },
  {
    label: 'Ça va casser mes projets ?',
    content:
      'Non. CrAIte ne modifie jamais les fichiers originaux. Tes projets continuent de pointer vers les mêmes samples aux mêmes emplacements.',
  },
  {
    label: "Sur quels systèmes d'exploitation CrAIte fonctionne ?",
    content:
      "CrAIte est disponible sur Windows, macOS et Linux. L'app fonctionne de la même façon sur tous les systèmes.",
  },
];
</script>
