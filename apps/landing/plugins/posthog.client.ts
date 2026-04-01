import posthog from 'posthog-js';

export default defineNuxtPlugin(() => {
  const { posthogKey, posthogHost } = useRuntimeConfig().public;
  const isDev = import.meta.dev;

  if (!posthogKey) return;

  posthog.init(posthogKey as string, {
    api_host: posthogHost as string,
    capture_pageview: true,
    capture_pageleave: true,
    persistence: 'memory',
  });

  posthog.register({ env: isDev ? 'dev' : 'production', app: 'landing' });

  return { provide: { posthog } };
});
