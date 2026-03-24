<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Category } from '@/types/sample';

interface Props {
  categories: Category[];
  selected: string | null;
}

const props = defineProps<Props>();

const { t } = useI18n();
const emit = defineEmits<{ select: [name: string | null] }>();

const GROUPS: { key: string; keys: string[] }[] = [
  { key: 'drums', keys: ['kick', 'snare', 'hihat', 'clap', 'cymbal', 'tom', 'perc'] },
  { key: 'bass', keys: ['bass'] },
  { key: 'synths', keys: ['pad', 'lead', 'arp', 'chord', 'keys', 'guitar', 'strings', 'brass'] },
  { key: 'vocal', keys: ['vocal'] },
  { key: 'fx', keys: ['fx'] },
  { key: 'loops', keys: ['loop'] },
]

const knownCategories = computed(() => props.categories.filter(c => c.name !== 'unknown'));
const unknownCategory = computed(() => props.categories.find(c => c.name === 'unknown') ?? null);

const groupedCategories = computed(() =>
  GROUPS.map(group => ({
    ...group,
    categories: knownCategories.value.filter(c => group.keys.includes(c.name.toLowerCase())),
  })).filter(g => g.categories.length > 0)
);

const ungroupedCategories = computed(() => {
  const grouped = GROUPS.flatMap(g => g.keys);
  return knownCategories.value.filter(c => !grouped.includes(c.name.toLowerCase()));
});

function handleSelect(name: string) {
  emit('select', name);
}

function clearSelection() {
  emit('select', null);
}
</script>

<template>
  <div class="flex flex-col gap-1">
    <UButton
      :color="selected === null ? 'primary' : 'neutral'"
      :variant="selected === null ? 'soft' : 'ghost'"
      class="w-full justify-start"
      @click="clearSelection"
    >
      <span class="flex-1 capitalize text-left">{{ t('browse.allCategories') }}</span>
      <span class="font-mono text-xs text-muted ml-auto">{{ categories.reduce((sum, c) => sum + c.count, 0) }}</span>
    </UButton>

    <template v-for="group in groupedCategories" :key="group.key">
      <div class="px-3 pt-3 pb-0.5">
        <span class="text-[10px] font-semibold uppercase tracking-[0.08em] text-muted/60">
          {{ t(`browse.group.${group.key}`) }}
        </span>
      </div>
      <UButton
        v-for="cat in group.categories"
        :key="cat.name"
        :color="selected === cat.name ? 'primary' : 'neutral'"
        :variant="selected === cat.name ? 'soft' : 'ghost'"
        class="w-full justify-start"
        @click="handleSelect(cat.name)"
      >
        <span class="size-2 rounded-full shrink-0" :style="{ background: cat.color }" />
        <span class="flex-1 capitalize text-left">{{ cat.name }}</span>
        <span class="font-mono text-xs text-muted ml-auto">{{ cat.count }}</span>
      </UButton>
    </template>

    <template v-if="ungroupedCategories.length > 0">
      <div class="px-3 pt-3 pb-0.5">
        <span class="text-[10px] font-semibold uppercase tracking-[0.08em] text-muted/60">
          {{ t('browse.group.other') }}
        </span>
      </div>
      <UButton
        v-for="cat in ungroupedCategories"
        :key="cat.name"
        :color="selected === cat.name ? 'primary' : 'neutral'"
        :variant="selected === cat.name ? 'soft' : 'ghost'"
        class="w-full justify-start"
        @click="handleSelect(cat.name)"
      >
        <span class="size-2 rounded-full shrink-0" :style="{ background: cat.color }" />
        <span class="flex-1 capitalize text-left">{{ cat.name }}</span>
        <span class="font-mono text-xs text-muted ml-auto">{{ cat.count }}</span>
      </UButton>
    </template>

    <!-- Unknown category — separated and visually muted -->
    <template v-if="unknownCategory">
      <div class="my-1 border-t border-white/5" />
      <UButton
        :color="selected === 'unknown' ? 'neutral' : 'neutral'"
        :variant="selected === 'unknown' ? 'soft' : 'ghost'"
        class="w-full justify-start opacity-50 hover:opacity-75"
        @click="handleSelect('unknown')"
      >
        <span class="size-2 rounded-full shrink-0 border border-white/20" />
        <span class="flex-1 text-left text-sm italic">{{ t('browse.unclassified') }}</span>
        <span class="font-mono text-xs text-muted ml-auto">{{ unknownCategory.count }}</span>
      </UButton>
    </template>
  </div>
</template>
