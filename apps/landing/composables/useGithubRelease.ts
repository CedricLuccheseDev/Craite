const FALLBACK_URL = 'https://github.com/CedricLuccheseDev/Craite/releases';

export function useGithubRelease() {
  const downloadUrl = ref(FALLBACK_URL);
  const version = ref('');
  const hasRelease = ref(false);

  useFetch('/api/release', {
    lazy: true,
    server: false,
    onResponse({ response }) {
      const data = response._data;
      if (!data?.downloadUrl) return;

      downloadUrl.value = data.downloadUrl;
      hasRelease.value = data.downloadUrl !== FALLBACK_URL;
      if (data.version) version.value = data.version;
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
    } else {
      window.open(FALLBACK_URL, '_blank');
    }
  }

  return { downloadUrl, version, hasRelease, triggerDownload };
}
