const REPO = 'CedricLuccheseDev/Craite';

interface GitHubAsset {
  name: string;
  size: number;
  content_type: string;
}

export default defineEventHandler(async event => {
  const { githubToken } = useRuntimeConfig();
  const id = getRouterParam(event, 'id');

  const asset = await $fetch<GitHubAsset>(`https://api.github.com/repos/${REPO}/releases/assets/${id}`, {
    headers: {
      Authorization: `Bearer ${githubToken}`,
      Accept: 'application/vnd.github+json',
    },
  }).catch(() => null);

  if (!asset) {
    throw createError({ statusCode: 404, message: 'Asset not found' });
  }

  const response = await fetch(`https://api.github.com/repos/${REPO}/releases/assets/${id}`, {
    headers: {
      Authorization: `Bearer ${githubToken}`,
      Accept: 'application/octet-stream',
    },
  });

  setHeader(event, 'Content-Disposition', `attachment; filename="${asset.name}"`);
  setHeader(event, 'Content-Type', asset.content_type);
  setHeader(event, 'Content-Length', String(asset.size));

  return sendWebResponse(event, response);
});
