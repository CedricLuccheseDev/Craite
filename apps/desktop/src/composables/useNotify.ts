// @ts-expect-error — useToast is auto-imported in .vue but needs explicit import in .ts
import { useToast } from '@nuxt/ui/dist/runtime/composables/useToast';
import { useI18n } from 'vue-i18n';

export function useNotify() {
  const toast = useToast();
  const { t } = useI18n();

  function error(descriptionKey: string, params?: Record<string, unknown>) {
    toast.add({
      title: t('notify.error'),
      description: t(descriptionKey, params ?? {}),
      icon: 'i-lucide-alert-circle',
      color: 'error',
      duration: 6000,
    });
  }

  function success(descriptionKey: string, params?: Record<string, unknown>) {
    toast.add({
      title: t('notify.success'),
      description: t(descriptionKey, params ?? {}),
      icon: 'i-lucide-check',
      color: 'success',
      duration: 3000,
    });
  }

  function info(descriptionKey: string, params?: Record<string, unknown>) {
    toast.add({
      description: t(descriptionKey, params ?? {}),
      icon: 'i-lucide-info',
      color: 'neutral',
      duration: 3000,
    });
  }

  return { error, success, info };
}
