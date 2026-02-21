<template>
  <section class="px-6 py-24 bg-zinc-900/30">
    <div class="max-w-3xl mx-auto">
      <h2 class="text-3xl font-bold text-center mb-16">
        Questions fréquentes
      </h2>

      <UAccordion
        :items="faq"
        color="neutral"
        variant="ghost"
        class="divide-y divide-zinc-800"
      >
        <template #body="{ item }">
          <p class="text-zinc-400 leading-relaxed pb-4">
            {{ item.content }}
          </p>
        </template>
      </UAccordion>
    </div>
  </section>
</template>

<script setup lang="ts">
const faq = [
  {
    label: "C'est vraiment gratuit ?",
    content: "Oui. Open source, licence MIT. Pas de version payante cachée, pas d'abonnement, pas de compte requis.",
  },
  {
    label: 'Mes samples sont-ils déplacés ?',
    content: "Non. CrAIte crée des liens (hardlinks) vers les fichiers originaux. Rien n'est copié ni déplacé.",
  },
  {
    label: 'Ça va casser mes projets ?',
    content: "Non. CrAIte ne modifie jamais les fichiers originaux. Les projets FL continuent de pointer vers les mêmes samples aux mêmes emplacements.",
  },
  {
    label: 'CrAIte fonctionne sur Mac ?',
    content: "Pas encore. Windows d'abord. Le support Mac est envisagé plus tard.",
  },
];
</script>
