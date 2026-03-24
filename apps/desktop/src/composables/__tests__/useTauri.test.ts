import { describe, it, expect, vi, beforeEach, beforeAll } from 'vitest';
import { useTauri } from '../useTauri';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('useTauri', () => {
  let mockInvoke: ReturnType<typeof vi.fn>;

  beforeAll(async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    mockInvoke = vi.mocked(invoke);
  });

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('scanDirectories', () => {
    it('should call invoke with enabled sources', async () => {
      const mockResult = { samples: [], sources: [] };
      mockInvoke.mockResolvedValue(mockResult);

      const { scanDirectories } = useTauri();
      const sources = [
        { path: '/path1', label: 'Path 1', enabled: true, type: 'custom' as const, sampleCount: 0 },
        { path: '/path2', label: 'Path 2', enabled: false, type: 'custom' as const, sampleCount: 0 },
        { path: '/path3', label: 'Path 3', enabled: true, type: 'custom' as const, sampleCount: 0 },
      ];

      const result = await scanDirectories(sources);

      expect(mockInvoke).toHaveBeenCalledWith('scan_directories', {
        sources: ['/path1', '/path3'],
      });
      expect(result).toEqual(mockResult);
    });
  });

  describe('detectSources', () => {
    it('should detect sources', async () => {
      const mockSources = [{ path: '/samples', enabled: true }];
      mockInvoke.mockResolvedValue(mockSources);

      const { detectSources } = useTauri();
      const result = await detectSources();

      expect(mockInvoke).toHaveBeenCalledWith('detect_sources');
      expect(result).toEqual(mockSources);
    });
  });

  describe('previewSample', () => {
    it('should preview a sample', async () => {
      mockInvoke.mockResolvedValue(undefined);

      const { previewSample } = useTauri();
      await previewSample('/path/to/sample.wav');

      expect(mockInvoke).toHaveBeenCalledWith('preview_sample', { path: '/path/to/sample.wav' });
    });
  });

  describe('stopPreview', () => {
    it('should stop preview', async () => {
      mockInvoke.mockResolvedValue(undefined);

      const { stopPreview } = useTauri();
      await stopPreview();

      expect(mockInvoke).toHaveBeenCalledWith('stop_preview');
    });
  });

  describe('saveSetting', () => {
    it('should save a setting', async () => {
      mockInvoke.mockResolvedValue(undefined);

      const { saveSetting } = useTauri();
      await saveSetting('theme', 'dark');

      expect(mockInvoke).toHaveBeenCalledWith('save_setting', { key: 'theme', value: 'dark' });
    });
  });

  describe('setBackgroundScanEnabled', () => {
    it('should enable background scan', async () => {
      mockInvoke.mockResolvedValue(undefined);

      const { setBackgroundScanEnabled } = useTauri();
      await setBackgroundScanEnabled(true);

      expect(mockInvoke).toHaveBeenCalledWith('set_background_scan_enabled', { enabled: true });
    });
  });

  describe('getBackgroundScanStatus', () => {
    it('should get background scan status', async () => {
      const mockStatus: [boolean, number, boolean] = [true, 60, false];
      mockInvoke.mockResolvedValue(mockStatus);

      const { getBackgroundScanStatus } = useTauri();
      const result = await getBackgroundScanStatus();

      expect(mockInvoke).toHaveBeenCalledWith('get_background_scan_status');
      expect(result).toEqual(mockStatus);
    });
  });
});
