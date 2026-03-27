<script setup lang="ts">
import { ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';

type Section = 'browse' | 'sources' | 'export' | 'settings';

interface Props {
  activeSection: Section;
}

defineProps<Props>();
const emit = defineEmits<{ 'update:activeSection': [value: Section] }>();

const { t } = useI18n();
const collapsed = ref(localStorage.getItem('sidebar_collapsed') === 'true');

function toggleCollapsed() {
  collapsed.value = !collapsed.value;
  localStorage.setItem('sidebar_collapsed', String(collapsed.value));
}

const mainNavItems = computed(() => [
  { id: 'sources' as Section, label: t('nav.sources'), icon: 'i-lucide-hard-drive' },
  { id: 'browse' as Section, label: t('nav.browse'), icon: 'i-lucide-music-2' },
  { id: 'export' as Section, label: t('nav.export'), icon: 'i-lucide-link' },
]);

const settingsItem = computed(() => (
  { id: 'settings' as Section, label: t('nav.settings'), icon: 'i-lucide-settings' }
));
</script>

<template>
  <aside
    class="sidebar"
    :class="{ collapsed }"
  >
    <div class="flex items-center h-8" :class="collapsed ? 'justify-center' : 'justify-between px-4'">
      <span v-if="!collapsed" class="font-display text-[22px] font-bold tracking-tight">CrAIte</span>
      <UTooltip :text="collapsed ? t('nav.expand') : t('nav.collapse')">
        <UButton
          :icon="collapsed ? 'i-lucide-panel-left-open' : 'i-lucide-panel-left-close'"
          color="neutral"
          variant="ghost"
          size="xs"
          @click="toggleCollapsed"
        />
      </UTooltip>
    </div>

    <nav class="flex flex-col gap-1" :class="collapsed ? 'items-center' : ''">
      <UTooltip v-for="item in mainNavItems" :key="item.id" :text="item.label" :disabled="!collapsed">
        <button
          class="nav-item"
          :class="[
            { active: activeSection === item.id },
            collapsed ? 'collapsed-item' : '',
          ]"
          @click="emit('update:activeSection', item.id)"
        >
          <UIcon :name="item.icon" class="text-[16px] shrink-0" />
          <span v-if="!collapsed">{{ item.label }}</span>
        </button>
      </UTooltip>
    </nav>

    <div class="mt-auto" :class="collapsed ? 'flex justify-center' : ''">
      <UTooltip :text="settingsItem.label" :disabled="!collapsed">
        <button
          class="nav-item"
          :class="[
            { active: activeSection === 'settings' },
            collapsed ? 'collapsed-item' : '',
          ]"
          @click="emit('update:activeSection', 'settings')"
        >
          <UIcon :name="settingsItem.icon" class="text-[16px] shrink-0" />
          <span v-if="!collapsed">{{ settingsItem.label }}</span>
        </button>
      </UTooltip>
    </div>
  </aside>
</template>

<style scoped>
@reference "../../assets/styles/variables.css";

.sidebar {
  @apply flex flex-col gap-8 py-9 px-3 w-[220px] shrink-0 bg-black/10
    transition-all duration-200 overflow-hidden;
}

.sidebar.collapsed {
  @apply w-[60px] px-1.5;
}

.nav-item {
  @apply flex items-center gap-3 w-full py-3 px-4 rounded-full
    text-[14px] font-medium text-muted bg-transparent border-none
    cursor-pointer transition-colors duration-150
    hover:bg-surface-hover hover:text-white;
}

.nav-item.active {
  @apply bg-white/10 text-white;
}

.nav-item.collapsed-item {
  @apply px-0 size-10 justify-center rounded-full;
}
</style>
