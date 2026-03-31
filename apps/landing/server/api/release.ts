export default defineEventHandler(async () => {
  const { releaseUrl } = useRuntimeConfig();

  const data = await $fetch<{ version: string; platforms: Record<string, unknown> }>(`${releaseUrl}/latest.json`).catch(
    err => {
      console.error('Failed to fetch release info:', err);
      return null;
    }
  );

  if (!data?.version) {
    return { version: null, platforms: {} };
  }

  return data;
});
