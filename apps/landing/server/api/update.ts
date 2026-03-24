const REPO = 'CedricLuccheseDev/Craite';

interface GitHubAsset {
  id: number;
  name: string;
}

interface GitHubRelease {
  tag_name: string;
  body: string;
  published_at: string;
  assets: GitHubAsset[];
}

export default defineEventHandler(async event => {
  const { githubToken } = useRuntimeConfig();

  const release = await $fetch<GitHubRelease>(`https://api.github.com/repos/${REPO}/releases/latest`, {
    headers: {
      Authorization: `Bearer ${githubToken}`,
      Accept: 'application/vnd.github+json',
    },
  }).catch(() => null);

  if (!release?.assets?.length) {
    throw createError({ statusCode: 503, message: 'No release found' });
  }

  const baseUrl = getRequestURL(event).origin;

  const winInstaller = release.assets.find(a => a.name.endsWith('.msi') || a.name.endsWith('.nsis.zip'));
  const winSig = winInstaller ? release.assets.find(a => a.name === `${winInstaller.name}.sig`) : null;

  if (!winInstaller || !winSig) {
    throw createError({ statusCode: 503, message: 'No Windows installer found' });
  }

  const signature = await $fetch<string>(`https://api.github.com/repos/${REPO}/releases/assets/${winSig.id}`, {
    headers: {
      Authorization: `Bearer ${githubToken}`,
      Accept: 'application/octet-stream',
    },
  });

  return {
    version: release.tag_name.replace(/^v/, ''),
    notes: release.body ?? '',
    pub_date: release.published_at,
    platforms: {
      'windows-x86_64': {
        signature,
        url: `${baseUrl}/api/download/${winInstaller.id}`,
      },
    },
  };
});
