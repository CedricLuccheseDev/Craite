<script setup lang="ts">
import { computed, onUnmounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useLibraryStore } from '@/stores/library';
import { useLibraryConfigStore } from '@/stores/libraryConfig';
import { useLibraryActions } from '@/composables/useLibraryActions';
import { useTauri } from '@/composables/useTauri';
import { useNotify } from '@/composables/useNotify';
import { useNavigation } from '@/composables/useNavigation';
import { getCategoryIcon } from '@/utils/categoryIcons';
import { GROUPS } from '@/utils/categoryGroups';
import DawGuide from '@/components/library/DawGuide.vue';
import type { Category } from '@/types/sample';

const { t } = useI18n();
const libraryStore = useLibraryStore();
const { navigateTo } = useNavigation();
const configStore = useLibraryConfigStore();
const { pickOutputDir, generateLibrary } = useLibraryActions();
const tauri = useTauri();
const notify = useNotify();

const pathCopied = ref(false);

async function openOutputDir() {
  if (configStore.outputDir) {
    try {
      await tauri.openFolder(configStore.outputDir);
    } catch (e) {
      console.error('Failed to open folder:', e);
      notify.error('notify.openFolderFailed');
    }
  }
}

async function copyOutputPath() {
  if (!configStore.outputDir) return;
  await navigator.clipboard.writeText(configStore.outputDir);
  pathCopied.value = true;
  setTimeout(() => {
    pathCopied.value = false;
  }, 1500);
}

const knownCategories = computed(() => libraryStore.categories.filter(c => c.name !== 'unknown'));

const groupedTree = computed(() => {
  const allGrouped = GROUPS.flatMap(g => g.keys);
  const ungrouped = knownCategories.value.filter(c => !allGrouped.includes(c.name.toLowerCase()));

  const groups = GROUPS.map(g => ({
    key: g.key,
    categories: knownCategories.value.filter(c => g.keys.includes(c.name.toLowerCase())),
  })).filter(g => g.categories.length > 0);

  if (ungrouped.length > 0) {
    groups.push({ key: 'other', categories: ungrouped });
  }
  return groups;
});

const includedSamples = computed(() =>
  knownCategories.value.filter(c => !configStore.excludedCategories.has(c.name)).reduce((sum, c) => sum + c.count, 0)
);

function groupState(categories: Category[]): 'all' | 'none' | 'partial' {
  const included = categories.filter(c => !configStore.excludedCategories.has(c.name)).length;
  if (included === categories.length) return 'all';
  if (included === 0) return 'none';
  return 'partial';
}

function toggleGroup(categories: Category[]) {
  const state = groupState(categories);
  categories.forEach(c => {
    const isExcluded = configStore.excludedCategories.has(c.name);
    if (state === 'all' && !isExcluded) configStore.toggleCategory(c.name);
    if (state !== 'all' && isExcluded) configStore.toggleCategory(c.name);
  });
  scheduleRegen();
}

// Debounced auto-regeneration
let regenTimer: ReturnType<typeof setTimeout> | null = null;

function toggleWithRegen(name: string) {
  configStore.toggleCategory(name);
  scheduleRegen();
}

function scheduleRegen() {
  if (!configStore.outputDir) return;
  if (regenTimer) clearTimeout(regenTimer);
  regenTimer = setTimeout(() => generateLibrary(), 600);
}

onUnmounted(() => {
  if (regenTimer) clearTimeout(regenTimer);
});
</script>

<template>
  <section class="h-full flex flex-col gap-8 bg-surface rounded-2xl p-9 overflow-y-auto">
    <!-- Header -->
    <div class="shrink-0 flex items-start justify-between">
      <div>
        <h2 class="text-[22px] font-bold tracking-tight">
          {{ t('export.title') }}
        </h2>
        <p class="text-sm text-muted mt-1.5">
          {{ t('export.description') }}
        </p>
      </div>
    </div>

    <!-- Empty state: no samples -->
    <div
      v-if="libraryStore.samples.length === 0"
      class="flex-1 flex flex-col items-center justify-center gap-4 text-center"
    >
      <div class="flex items-center justify-center size-14 rounded-2xl bg-zinc-800">
        <UIcon name="i-lucide-link" class="text-[26px] text-muted" />
      </div>
      <div>
        <p class="text-sm font-medium text-white/80">{{ t('export.emptyTitle') }}</p>
        <p class="text-[12px] text-muted mt-1">{{ t('export.emptyHint') }}</p>
      </div>
      <UButton
        icon="i-lucide-hard-drive"
        color="primary"
        variant="solid"
        size="sm"
        class="mt-2"
        @click="navigateTo('sources')"
      >
        {{ t('export.goToSources') }}
      </UButton>
    </div>

    <template v-else>
      <!-- Output directory -->
      <div class="flex flex-col gap-2">
        <span class="section-label">{{ t('export.outputDir') }}</span>

        <button v-if="!configStore.outputDir" class="output-empty" @click="pickOutputDir">
          <div class="output-empty-icon">
            <UIcon name="i-lucide-folder-plus" class="text-[22px] text-muted" />
          </div>
          <p class="text-sm font-medium text-white/80">{{ t('export.choose') }}</p>
          <p class="text-[12px] text-muted">{{ t('export.chooseHint') }}</p>
        </button>

        <div v-else class="output-filled" @click="openOutputDir">
          <div class="flex items-center gap-3 flex-1 min-w-0">
            <UIcon name="i-lucide-folder-check" class="text-orange-500 text-[18px] shrink-0" />
            <div class="flex flex-col min-w-0">
              <span class="text-xs text-muted mb-0.5">{{ t('export.outputDir') }}</span>
              <span class="text-sm font-medium truncate">{{ configStore.outputDir }}</span>
            </div>
          </div>
          <div class="flex items-center gap-1 shrink-0" @click.stop>
            <UButton
              :icon="pathCopied ? 'i-lucide-check' : 'i-lucide-copy'"
              color="neutral"
              variant="ghost"
              size="sm"
              :class="pathCopied ? 'text-green-500' : ''"
              @click="copyOutputPath"
            />
            <UButton icon="i-lucide-pencil" color="neutral" variant="ghost" size="sm" @click="pickOutputDir">
              {{ t('export.change') }}
            </UButton>
          </div>
        </div>
      </div>

      <!-- Include categories -->
      <div class="flex flex-col gap-3">
        <div class="flex items-center justify-between">
          <span class="section-label">{{ t('export.includeCategories') }}</span>
          <div class="flex items-center gap-2">
            <span v-if="configStore.isGenerating" class="flex items-center gap-1.5 text-[12px] text-orange-500">
              <UIcon name="i-lucide-loader-2" class="animate-spin text-[12px]" />
              {{ t('export.syncing') }}
            </span>
            <span v-else class="text-[12px] text-muted tabular-nums">
              {{ includedSamples.toLocaleString() }} {{ t('stats.samples').toLowerCase() }}
            </span>
          </div>
        </div>

        <div class="flex flex-col gap-1">
          <div v-for="group in groupedTree" :key="group.key" class="group-block">
            <!-- Group header -->
            <button class="group-header" @click="toggleGroup(group.categories)">
              <div class="flex items-center gap-2 flex-1 min-w-0">
                <span
                  class="group-check"
                  :class="{
                    checked: groupState(group.categories) === 'all',
                    partial: groupState(group.categories) === 'partial',
                  }"
                >
                  <UIcon v-if="groupState(group.categories) === 'all'" name="i-lucide-check" class="text-[10px]" />
                  <span
                    v-else-if="groupState(group.categories) === 'partial'"
                    class="w-2 h-0.5 bg-current rounded-full"
                  />
                </span>
                <span class="text-[12px] font-semibold uppercase tracking-[0.07em] text-muted/80">
                  {{ t(`browse.group.${group.key}`) }}
                </span>
              </div>
              <span class="text-[11px] text-muted/50 font-mono tabular-nums">
                {{ group.categories.reduce((s, c) => s + c.count, 0).toLocaleString() }}
              </span>
            </button>

            <!-- Category rows -->
            <div class="flex flex-col">
              <button
                v-for="cat in group.categories"
                :key="cat.name"
                class="cat-row"
                :class="{ excluded: configStore.excludedCategories.has(cat.name) }"
                @click="toggleWithRegen(cat.name)"
              >
                <span
                  class="row-check"
                  :class="{ checked: !configStore.excludedCategories.has(cat.name) }"
                  :style="!configStore.excludedCategories.has(cat.name) ? { '--check-color': cat.color } : {}"
                >
                  <UIcon
                    v-if="!configStore.excludedCategories.has(cat.name)"
                    name="i-lucide-check"
                    class="text-[10px]"
                  />
                </span>
                <UIcon :name="getCategoryIcon(cat.name)" class="text-[13px] shrink-0" :style="{ color: cat.color }" />
                <span class="flex-1 capitalize text-[13px]">{{ cat.name }}</span>
                <span class="font-mono text-[11px] text-muted/50 tabular-nums">{{ cat.count.toLocaleString() }}</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- DAW guide -->
      <DawGuide />

      <!-- Footer status -->
      <div
        v-if="configStore.outputDir"
        class="mt-auto shrink-0 flex items-center justify-between pt-4 border-t border-zinc-800"
      >
        <div class="flex items-center gap-2">
          <span v-if="configStore.isGenerating" class="flex items-center gap-1.5 text-[12px] text-orange-500">
            <UIcon name="i-lucide-loader-2" class="animate-spin text-[12px]" />
            {{ t('export.syncing') }}
          </span>
          <template v-else-if="configStore.lastGeneratedAt">
            <UIcon name="i-lucide-check-circle" class="text-green-500 text-[13px]" />
            <span class="text-xs text-muted">
              {{ configStore.linkedCount.toLocaleString() }} {{ t('export.filesLinked') }}
            </span>
          </template>
          <span v-else class="text-xs text-muted">{{ t('export.neverGenerated') }}</span>
        </div>
        <UButton
          icon="i-lucide-refresh-cw"
          color="neutral"
          variant="ghost"
          size="xs"
          :disabled="configStore.isGenerating"
          @click="generateLibrary"
        />
      </div>
    </template>
  </section>
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.section-label {
  @apply text-[12px] font-semibold uppercase tracking-[0.07em] text-muted;
}

.output-empty {
  @apply flex flex-col items-center gap-2.5 py-10 px-6 w-full
    rounded-xl border-2 border-dashed border-zinc-700
    cursor-pointer bg-transparent text-center
    transition-all duration-200;
}

.output-empty:hover {
  @apply border-zinc-500 bg-white/2;
}

.output-empty:hover .output-empty-icon {
  @apply scale-110;
}

.output-empty-icon {
  @apply flex items-center justify-center size-12 rounded-2xl bg-zinc-800
    transition-transform duration-200;
}

.output-filled {
  @apply flex items-center gap-4 py-4 px-5
    rounded-xl border border-zinc-800 bg-zinc-950
    transition-colors duration-150;
}

.output-filled:hover {
  @apply border-zinc-700;
}

/* Tree */
.group-block {
  @apply rounded-xl overflow-hidden border border-zinc-800/60 mb-2;
}

.group-header {
  @apply flex items-center gap-3 w-full px-4 py-3 text-left
    bg-zinc-900/60 cursor-pointer
    transition-colors duration-100;
  border-radius: 0 !important;
}

.group-header:hover {
  @apply bg-zinc-800/60;
}

.group-check {
  @apply flex items-center justify-center size-4 rounded shrink-0
    border border-zinc-600 text-muted
    transition-all duration-100;
}

.group-check.checked {
  @apply bg-zinc-500 border-zinc-500 text-white;
}

.group-check.partial {
  @apply bg-zinc-700 border-zinc-600 text-muted;
}

.cat-row {
  @apply flex items-center gap-3 w-full px-4 py-2.5 text-left
    bg-transparent cursor-pointer border-t border-zinc-800/40
    transition-colors duration-100;
  border-radius: 0 !important;
}

.cat-row:hover {
  @apply bg-zinc-800/30;
}

.cat-row.excluded {
  @apply opacity-40;
}

.row-check {
  @apply flex items-center justify-center size-4 rounded shrink-0
    border border-zinc-600
    transition-all duration-100;
}

.row-check.checked {
  border-color: var(--check-color);
  background: var(--check-color);
  color: #0a0a0a;
}
</style>
