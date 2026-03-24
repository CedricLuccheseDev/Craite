import { createRouter, createWebHistory } from 'vue-router';
import { isOnboardingCompleted } from '@/composables/useOnboarding';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'onboarding',
      component: () => import('@/views/OnboardingView.vue'),
    },
    {
      path: '/library',
      name: 'library',
      component: () => import('@/views/LibraryView.vue'),
    },
  ],
});

router.beforeEach(to => {
  if (to.name === 'onboarding' && isOnboardingCompleted()) {
    return { name: 'library' };
  }
});

export default router;
