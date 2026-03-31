<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import type { Category } from '@/types/sample';
import { getCategoryColor } from '@/utils/categoryColors';
import { getCategoryIcon } from '@/utils/categoryIcons';
import { groupCategories } from '@/utils/categoryGroups';

interface Props {
  totalSamples: number;
  categoryCount: number;
  categories: Category[];
}

defineProps<Props>();
const { t } = useI18n();
const emit = defineEmits<{ finish: [] }>();
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex-1 min-h-0 overflow-y-auto scrollbar-thin">
      <div class="flex flex-col items-center justify-center gap-7 min-h-full py-4">
        <div class="flex flex-col items-center gap-2 text-center">
          <div class="flex items-center justify-center size-12 rounded-2xl bg-green-500/10 mb-1">
            <UIcon name="i-lucide-check" class="text-[22px] text-green-500" />
          </div>
          <h2 class="font-display text-[28px] font-bold tracking-tight">
            {{ t('onboarding.ready.title') }}
          </h2>
          <p class="text-sm text-muted">
            {{ t('onboarding.ready.subtitle', { samples: totalSamples.toLocaleString(), categories: categoryCount }) }}
          </p>
        </div>

        <!-- Category grid — grouped by family, no labels -->
        <div class="w-full max-w-2xl flex flex-wrap justify-center gap-2">
          <template v-for="group in groupCategories(categories)" :key="group.key">
            <div v-for="cat in group.categories" :key="cat.name" class="cat-card">
              <UIcon
                :name="getCategoryIcon(cat.name)"
                class="text-[14px]"
                :style="{ color: getCategoryColor(cat.name) }"
              />
              <span class="text-[11px] font-medium capitalize">{{ cat.name }}</span>
              <span class="text-[13px] font-bold font-mono tabular-nums text-white/80">{{
                cat.count.toLocaleString()
              }}</span>
            </div>
          </template>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="shrink-0 flex flex-col items-center gap-2 py-5">
      <UButton color="primary" variant="solid" size="lg" class="px-10" @click="emit('finish')">
        {{ t('onboarding.ready.openLibrary') }}
      </UButton>
    </div>
  </div>
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.cat-card {
  @apply flex items-center gap-1.5 py-1.5 px-2.5
    rounded-lg bg-zinc-900/50 border border-zinc-800/40
    transition-colors duration-150;
}
</style>
