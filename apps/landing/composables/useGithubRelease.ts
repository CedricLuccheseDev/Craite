const REPO = 'CedricLuccheseDev/Craite';
const FALLBACK_URL = `https://github.com/${REPO}/releases`;
const API_URL = `https://api.github.com/repos/${REPO}/releases/latest`;

interface GitHubAsset {
  name: string;
  browser_download_url: string;
}

interface GitHubRelease {
  tag_name: string;
  name: string;
  assets: GitHubAsset[];
}

export function useGithubRelease() {
  const downloadUrl = ref(FALLBACK_URL);
  const version = ref('');
  const hasRelease = ref(false);

  useFetch<GitHubRelease>(API_URL, {
    lazy: true,
    server: false,
    onResponse({ response }) {
      const release = response._data;
      if (!release?.assets?.length) return;

      const installer = release.assets.find(
        (a: GitHubAsset) => a.name.endsWith('.exe') || a.name.endsWith('.msi'),
      );

      if (installer) {
        downloadUrl.value = installer.browser_download_url;
        version.value = release.tag_name;
        hasRelease.value = true;
      }
    },
  });

  function triggerDownload() {
    if (hasRelease.value) {
      const link = document.createElement('a');
      link.href = downloadUrl.value;
      link.download = '';
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
    }
    else {
      window.open(FALLBACK_URL, '_blank');
    }
  }

  return { downloadUrl, version, hasRelease, triggerDownload };
}
