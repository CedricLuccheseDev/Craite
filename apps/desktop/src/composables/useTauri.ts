import { invoke } from '@tauri-apps/api/core';
import type { Sample, Source, ScanResult, DawInfo } from '@/types/sample';

export function useTauri() {
  async function scanDirectories(sources: Source[]): Promise<ScanResult> {
    return await invoke<ScanResult>('scan_directories', {
      sources: sources.filter(s => s.enabled).map(s => s.path),
    });
  }

  async function createLinks(outputDir: string): Promise<number> {
    return await invoke<number>('create_links', { outputDir });
  }

  async function detectSources(): Promise<Source[]> {
    return await invoke<Source[]>('detect_sources');
  }

  async function previewSample(path: string): Promise<void> {
    return await invoke<void>('preview_sample', { path });
  }

  async function stopPreview(): Promise<void> {
    return await invoke<void>('stop_preview');
  }

  async function readAudioFile(path: string): Promise<number[]> {
    return await invoke<number[]>('read_audio_file', { path });
  }

  // Persistence
  async function loadSamples(): Promise<Sample[]> {
    return await invoke<Sample[]>('load_samples');
  }

  async function loadSources(): Promise<Source[]> {
    return await invoke<Source[]>('load_sources');
  }

  async function saveSource(source: Source): Promise<void> {
    return await invoke<void>('save_source', { source });
  }

  async function updateSourceEnabled(path: string, enabled: boolean): Promise<void> {
    return await invoke<void>('update_source_enabled', { path, enabled });
  }

  async function saveSetting(key: string, value: string): Promise<void> {
    return await invoke<void>('save_setting', { key, value });
  }

  async function loadAllSettings(): Promise<[string, string][]> {
    return await invoke<[string, string][]>('load_all_settings');
  }

  // DAW
  async function detectInstalledDaws(): Promise<DawInfo[]> {
    return await invoke<DawInfo[]>('detect_installed_daws');
  }

  async function createDawLibraryFolder(path: string): Promise<string> {
    return await invoke<string>('create_daw_library_folder', { path });
  }

  // Reset
  async function resetApp(): Promise<void> {
    return await invoke<void>('reset_app');
  }

  // Background scan
  async function setBackgroundScanEnabled(enabled: boolean): Promise<void> {
    return await invoke<void>('set_background_scan_enabled', { enabled });
  }

  async function setScanInterval(minutes: number): Promise<void> {
    return await invoke<void>('set_scan_interval', { minutes });
  }

  async function getBackgroundScanStatus(): Promise<[boolean, number, boolean]> {
    return await invoke<[boolean, number, boolean]>('get_background_scan_status');
  }

  return {
    scanDirectories,
    createLinks,
    detectSources,
    previewSample,
    stopPreview,
    readAudioFile,
    loadSamples,
    loadSources,
    saveSource,
    updateSourceEnabled,
    saveSetting,
    loadAllSettings,
    detectInstalledDaws,
    createDawLibraryFolder,
    resetApp,
    setBackgroundScanEnabled,
    setScanInterval,
    getBackgroundScanStatus,
  };
}
