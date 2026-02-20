import { invoke } from '@tauri-apps/api/core';
import type { Source, ScanResult } from '@/types/sample';

export function useTauri() {
  async function scanDirectories(sources: Source[]): Promise<ScanResult> {
    return await invoke<ScanResult>('scan_directories', {
      sources: sources.filter(s => s.enabled).map(s => s.path),
    });
  }

  async function classifySamples(scanId: number): Promise<ScanResult> {
    return await invoke<ScanResult>('classify_samples', { scanId });
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

  return {
    scanDirectories,
    classifySamples,
    createLinks,
    detectSources,
    previewSample,
    stopPreview,
  };
}
