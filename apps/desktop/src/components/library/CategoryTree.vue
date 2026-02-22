<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { Category } from '@/types/sample';

interface Props {
  categories: Category[];
  selected: string | null;
}

defineProps<Props>();

const { t } = useI18n();
const emit = defineEmits<{ select: [name: string | null] }>();

function handleSelect(name: string) {
  emit('select', name);
}

function clearSelection() {
  emit('select', null);
}
</script>

<template>
  <div class="flex flex-col gap-0.5">
    <UButton
      :color="selected === null ? 'primary' : 'neutral'"
      :variant="selected === null ? 'soft' : 'ghost'"
      class="w-full justify-start"
      @click="clearSelection"
    >
      <span class="flex-1 capitalize text-left">{{ t('browse.allCategories') }}</span>
      <span class="font-mono text-xs text-muted ml-auto">{{ categories.reduce((sum, c) => sum + c.count, 0) }}</span>
    </UButton>

    <UButton
      v-for="cat in categories"
      :key="cat.name"
      :color="selected === cat.name ? 'primary' : 'neutral'"
      :variant="selected === cat.name ? 'soft' : 'ghost'"
      class="w-full justify-start"
      @click="handleSelect(cat.name)"
    >
      <span
        class="size-2 rounded-full shrink-0"
        :style="{ background: cat.color }"
      />
      <span class="flex-1 capitalize text-left">{{ cat.name }}</span>
      <span class="font-mono text-xs text-muted ml-auto">{{ cat.count }}</span>
    </UButton>
  </div>
</template>
