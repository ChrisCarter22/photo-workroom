import { beforeEach, describe, expect, it, vi } from 'vitest';
import { invokeCommand } from './tauri';
import {
  consumeWindowFolderOpenRequest,
  openFolderInSeparateWindow,
  type OpenFolderInSeparateWindowResponse,
} from './windowing';

vi.mock('./tauri', () => ({
  invokeCommand: vi.fn(),
}));

describe('windowing IPC helpers', () => {
  beforeEach(() => {
    vi.mocked(invokeCommand).mockReset();
  });

  it('opens a folder in a separate window through a typed command', async () => {
    const response: OpenFolderInSeparateWindowResponse = {
      requestId: 'window-open-1',
      windowLabel: 'workspace-assignments-1',
      folderPath: '/Assignments',
      activeWorkspace: 'Untitled',
      opened: true,
      message: 'Opened folder in a separate window.',
    };
    vi.mocked(invokeCommand).mockResolvedValue(response);

    const request = {
      requestId: 'window-open-1',
      folderPath: '/Assignments',
      activeWorkspace: 'Untitled',
    };
    const result = await openFolderInSeparateWindow(request);

    expect(vi.mocked(invokeCommand)).toHaveBeenCalledWith('open_folder_in_separate_window', {
      request,
    });
    expect(result).toEqual(response);
  });

  it('consumes a pending startup folder open request for the current window', async () => {
    vi.mocked(invokeCommand).mockResolvedValue({
      windowLabel: 'workspace-assignments-1',
      folderPath: '/Assignments',
    });

    const result = await consumeWindowFolderOpenRequest();

    expect(vi.mocked(invokeCommand)).toHaveBeenCalledWith('consume_window_folder_open_request');
    expect(result).toEqual({
      windowLabel: 'workspace-assignments-1',
      folderPath: '/Assignments',
    });
  });
});
