import { ref } from 'vue';
import { defineStore } from 'pinia';
import { useI18n } from 'vue-i18n';
import { useTauri } from '@/composables/useTauri';
import type { SupportedLocale } from '@/plugins/i18n';

export const useSettingsStore = defineStore('settings', () => {
  const tauri = useTauri();
  const { locale: i18nLocale } = useI18n({ useScope: 'global' });

  const locale = ref<SupportedLocale>('en');

  function applyLocale(newLocale: SupportedLocale) {
    locale.value = newLocale;
    i18nLocale.value = newLocale;
  }

  async function setLocale(newLocale: SupportedLocale) {
    applyLocale(newLocale);
    await tauri.saveSetting('locale', newLocale);
  }

  return { locale, applyLocale, setLocale };
});
