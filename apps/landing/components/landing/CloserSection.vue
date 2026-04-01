<template>
  <section id="download" class="section-dark px-6 py-32 text-center">
    <div class="max-w-3xl mx-auto">
      <!-- eslint-disable-next-line vue/no-v-html -- safe: content from i18n translation files only -->
      <h2 class="text-4xl sm:text-5xl font-bold mb-8 leading-tight" v-html="t('download.title', { br: '<br />' })" />
      <p class="text-zinc-400 text-lg leading-relaxed mb-6">
        {{ t('download.subtitle') }}
      </p>
      <p class="text-zinc-300 text-lg leading-relaxed mb-12">
        <strong class="text-white">{{ t('download.cta') }}</strong>
      </p>

      <ClientOnly>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 max-w-2xl mx-auto">
          <button
            v-for="item in downloadItems"
            :key="item.key"
            :disabled="!item.available"
            class="group flex flex-col items-center gap-3 p-6 rounded-2xl border transition-all duration-200"
            :class="
              item.available
                ? 'border-white/10 bg-white/3 hover:border-[#ff6b35]/40 hover:bg-[#ff6b35]/5 cursor-pointer'
                : 'border-white/5 bg-white/2 opacity-40 cursor-not-allowed'
            "
            @click="item.available && handleDownload(item.key)"
          >
            <UIcon :name="item.icon" class="text-2xl" :class="item.available ? 'text-[#ff6b35]' : 'text-zinc-600'" />
            <span class="font-semibold">{{ item.label }}</span>
            <span v-if="item.available && item.size" class="text-xs text-zinc-500">{{ formatSize(item.size) }}</span>
            <span v-else-if="!item.available" class="text-xs text-zinc-600">{{ t('download.comingSoon') }}</span>
          </button>
        </div>

        <p v-if="version" class="mt-6 text-sm text-zinc-500">{{ t('download.version', { version }) }}</p>

        <p v-if="trackingCopied" class="mt-4 text-xs text-[#ff6b35] transition-opacity duration-300">
          {{ t('download.trackingCopied') }}
        </p>
      </ClientOnly>

      <div class="mt-8 flex flex-wrap gap-3 justify-center">
        <span
          v-for="badge in badges"
          :key="badge"
          class="text-xs text-zinc-400 px-3 py-1.5 rounded-full border border-white/10 bg-white/3"
        >
          {{ badge }}
        </span>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const { t } = useI18n();
const { version, platforms, triggerDownload } = useRelease();
const trackingCopied = ref(false);

const badges = computed(() => [t('download.badgeFree'), t('download.badgePlatform'), t('download.badgeLocal')]);

const downloadItems = computed(() => [
  {
    key: 'windows',
    label: 'Windows',
    icon: 'i-lucide-monitor',
    available: !!platforms.value.windows,
    size: platforms.value.windows?.size,
  },
  {
    key: 'mac_arm',
    label: 'macOS',
    icon: 'i-lucide-laptop',
    available: !!platforms.value.mac_arm || !!platforms.value.mac_intel,
    size: platforms.value.mac_arm?.size ?? platforms.value.mac_intel?.size,
  },
  {
    key: 'linux',
    label: 'Linux',
    icon: 'i-lucide-terminal',
    available: !!platforms.value.linux,
    size: platforms.value.linux?.size,
  },
]);

async function handleDownload(key: string) {
  const { $posthog } = useNuxtApp();
  $posthog?.capture('download_clicked', { platform: key });

  const distinctId = $posthog?.get_distinct_id?.();
  if (distinctId) {
    try {
      await navigator.clipboard.writeText(distinctId);
      trackingCopied.value = true;
      setTimeout(() => (trackingCopied.value = false), 5000);
    } catch {
      // Clipboard not available — ignore silently
    }
  }

  triggerDownload(key);
}

function formatSize(bytes: number): string {
  const mb = bytes / (1024 * 1024);
  return `${mb.toFixed(1)} MB`;
}
</script>
