const REPO = 'CedricLuccheseDev/Craite';
const RELEASES_URL = `https://github.com/${REPO}/releases`;

interface GitHubAsset {
  name: string;
  browser_download_url: string;
}

interface GitHubRelease {
  tag_name: string;
  assets: GitHubAsset[];
}

export default defineEventHandler(async () => {
  const { githubToken } = useRuntimeConfig();

  const release = await $fetch<GitHubRelease>(`https://api.github.com/repos/${REPO}/releases/latest`, {
    headers: {
      Authorization: `Bearer ${githubToken}`,
      Accept: 'application/vnd.github+json',
    },
  }).catch(() => null);

  if (!release?.assets?.length) {
    return { version: null, downloadUrl: RELEASES_URL };
  }

  const installer = release.assets.find(a => a.name.endsWith('.exe') || a.name.endsWith('.msi'));

  return {
    version: release.tag_name,
    downloadUrl: installer?.browser_download_url ?? RELEASES_URL,
  };
});
