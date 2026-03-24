<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Sample } from '@/types/sample';
import AudioPlayer from '@/components/common/AudioPlayer.vue';

const PAGE_SIZE = 80;

interface Props {
  samples: Sample[];
}

const props = defineProps<Props>();
const { t } = useI18n();

const displayCount = ref(PAGE_SIZE);
const scrollContainer = ref<HTMLElement | null>(null);

const visibleSamples = computed(() => props.samples.slice(0, displayCount.value));

const hasMore = computed(() => displayCount.value < props.samples.length);

// Reset pagination when samples change (filter, search, category)
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
  <div ref="scrollContainer" class="flex flex-col gap-px overflow-y-auto h-full">
    <div
      v-for="sample in visibleSamples"
      :key="sample.id"
      class="flex items-center justify-between py-3 px-5 bg-surface transition-colors duration-150 hover:bg-surface-hover"
    >
      <AudioPlayer :sample-path="sample.path" :sample-name="sample.name" />
      <UBadge :label="sample.category" color="neutral" variant="subtle" size="sm" />
    </div>

    <p v-if="samples.length === 0" class="p-12 text-center text-muted">
      {{ t('browse.noSamples') }}
    </p>
  </div>
</template>
