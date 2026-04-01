<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Sample } from '@/types/sample';
import AudioPlayer from '@/components/common/AudioPlayer.vue';
import { useTauri } from '@/composables/useTauri';
import { useNotify } from '@/composables/useNotify';
import { usePosthog } from '@/composables/usePosthog';
import { getCategoryIcon } from '@/utils/categoryIcons';
import { getCategoryColor } from '@/utils/categoryColors';
import { CATEGORY_COLORS_HEX } from '@/utils/categoryColors';

const PAGE_SIZE = 80;

const ALL_CATEGORIES = Object.keys(CATEGORY_COLORS_HEX);

interface Props {
  samples: Sample[];
}

const props = defineProps<Props>();
const emit = defineEmits<{ categoryChanged: [sample: Sample] }>();
const { t } = useI18n();
const tauri = useTauri();
const notify = useNotify();
const ph = usePosthog();
const playerRefs = new Map<number, InstanceType<typeof AudioPlayer>>();

const editingSampleId = ref<number | null>(null);

function setPlayerRef(id: number, el: InstanceType<typeof AudioPlayer> | null) {
  if (el) playerRefs.set(id, el);
  else playerRefs.delete(id);
}

function playSample(id: number) {
  playerRefs.get(id)?.togglePlay();
}

function parentDir(filePath: string): string {
  const sep = filePath.includes('\\') ? '\\' : '/';
  return filePath.substring(0, filePath.lastIndexOf(sep));
}

async function openInExplorer(sample: Sample) {
  try {
    await tauri.openFolder(parentDir(sample.path));
  } catch {
    notify.error('notify.openFolderFailed');
  }
}

async function toggleHidden(sample: Sample) {
  const newHidden = !sample.hidden;
  try {
    await tauri.setSampleHidden(sample.id, newHidden);
    sample.hidden = newHidden;
  } catch {
    // silently ignore
  }
}

function startEditing(sampleId: number) {
  editingSampleId.value = editingSampleId.value === sampleId ? null : sampleId;
}

async function changeCategory(sample: Sample, newCategory: string) {
  const oldCategory = sample.category;
  if (newCategory === oldCategory) {
    editingSampleId.value = null;
    return;
  }
  try {
    await tauri.updateSampleCategory(sample.id, newCategory);
    sample.category = newCategory;
    sample.confidence = 1.0;
    editingSampleId.value = null;
    ph.track('category_corrected', { from: oldCategory, to: newCategory });
    emit('categoryChanged', sample);
  } catch {
    notify.error('notify.categoryChangeFailed');
  }
}

const displayCount = ref(PAGE_SIZE);
const scrollContainer = ref<HTMLElement | null>(null);

const visibleSamples = computed(() => props.samples.slice(0, displayCount.value));

const hasMore = computed(() => displayCount.value < props.samples.length);

watch(
  () => props.samples.length,
  () => {
    displayCount.value = PAGE_SIZE;
  }
);

function onScroll() {
  const el = scrollContainer.value;
  if (!el || !hasMore.value) return;

  const nearBottom = el.scrollHeight - el.scrollTop - el.clientHeight < 200;
  if (nearBottom) {
    displayCount.value += PAGE_SIZE;
  }
}

onMounted(() => {
  scrollContainer.value?.addEventListener('scroll', onScroll, { passive: true });
});

onUnmounted(() => {
  scrollContainer.value?.removeEventListener('scroll', onScroll);
});
</script>

<template>
  <div ref="scrollContainer" class="flex flex-col gap-px overflow-y-auto overflow-x-hidden h-full pr-2 scrollbar-thin">
    <div
      v-for="sample in visibleSamples"
      :key="sample.id"
      class="sample-row"
      :class="{ hidden: sample.hidden }"
      @click="playSample(sample.id)"
    >
      <UIcon
        :name="getCategoryIcon(sample.category)"
        class="text-[13px] shrink-0"
        :style="{ color: getCategoryColor(sample.category) }"
      />
      <AudioPlayer
        :ref="(el: any) => setPlayerRef(sample.id, el)"
        :sample-path="sample.path"
        :sample-name="sample.name"
        class="flex-1 min-w-0"
      />
      <div class="flex items-center gap-1 shrink-0" @click.stop>
        <!-- Category correction -->
        <div class="relative">
          <UButton
            icon="i-lucide-tag"
            color="neutral"
            variant="ghost"
            size="xs"
            :class="editingSampleId === sample.id ? 'text-[#ff6b35]' : 'text-white/40'"
            @click="startEditing(sample.id)"
          />
          <div
            v-if="editingSampleId === sample.id"
            class="absolute right-0 top-full mt-1 z-50 bg-zinc-900 border border-zinc-700 rounded-lg p-1 shadow-xl max-h-48 overflow-y-auto w-32"
          >
            <button
              v-for="cat in ALL_CATEGORIES"
              :key="cat"
              class="flex items-center gap-2 w-full px-2 py-1.5 rounded text-xs hover:bg-zinc-800 transition-colors"
              :class="cat === sample.category ? 'text-white font-semibold' : 'text-zinc-400'"
              @click="changeCategory(sample, cat)"
            >
              <UIcon :name="getCategoryIcon(cat)" class="text-[11px]" :style="{ color: getCategoryColor(cat) }" />
              {{ cat }}
            </button>
          </div>
        </div>

        <UButton
          :icon="sample.hidden ? 'i-lucide-eye-off' : 'i-lucide-eye'"
          color="neutral"
          variant="ghost"
          size="xs"
          :class="sample.hidden ? 'text-muted' : 'text-white/60'"
          @click="toggleHidden(sample)"
        />
        <UButton
          icon="i-lucide-folder-open"
          color="neutral"
          variant="ghost"
          size="xs"
          @click="openInExplorer(sample)"
        />
      </div>
    </div>

    <p v-if="samples.length === 0" class="p-12 text-center text-muted">
      {{ t('browse.noSamples') }}
    </p>
  </div>
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.sample-row {
  @apply flex items-center justify-between py-3 px-1 bg-surface
    transition-colors duration-150;
}

.sample-row:hover {
  @apply bg-surface-hover;
}

.sample-row.hidden {
  @apply opacity-40;
}
</style>
