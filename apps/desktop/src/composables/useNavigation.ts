import { inject, provide, type Ref } from 'vue';

export type Section = 'browse' | 'sources' | 'export' | 'settings';

const NAV_KEY = Symbol('navigation');

export function provideNavigation(activeSection: Ref<Section>) {
  const navigateTo = (section: Section) => {
    activeSection.value = section;
  };
  provide(NAV_KEY, navigateTo);
}

export function useNavigation() {
  const navigateTo = inject<(section: Section) => void>(NAV_KEY, () => {});
  return { navigateTo };
}
