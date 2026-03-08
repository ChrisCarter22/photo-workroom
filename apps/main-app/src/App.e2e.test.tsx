import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import App from './App';

const invokeCommand = vi.fn();

vi.mock('./lib/tauri', () => ({
  invokeCommand: (...args: unknown[]) => invokeCommand(...args),
  isTauriRuntime: () => true,
}));

describe('App shell smoke flow', () => {
  beforeEach(() => {
    invokeCommand.mockReset();
  });

  it('opens folders into tabs, supports explicit separate windows, and renders a backend health result', async () => {
    invokeCommand.mockImplementation((command: string) => {
      if (command === 'consume_window_folder_open_request') {
        return Promise.resolve({
          windowLabel: 'main',
          folderPath: null,
        });
      }

      if (command === 'open_folder_in_separate_window') {
        return Promise.resolve({
          requestId: 'window-open-smoke',
          windowLabel: 'workspace-second-copy-1',
          folderPath: '/Assignments/Second-Copy',
          activeWorkspace: 'Second-Copy',
          opened: true,
          message: 'Opened folder in a separate window.',
        });
      }

      if (command === 'health_check') {
        return Promise.resolve({
          requestId: 'health-check-smoke',
          appVersion: '0.1.0',
          runtime: 'tauri-v2',
          activeWorkspace: 'Assignments',
          healthy: true,
          message: 'Desktop shell baseline is healthy.',
          subsystems: [
            { name: 'db', phase: 'Phase 3', summary: 'SQLite boundary scaffolded.' },
            { name: 'metadata', phase: 'Phase 9', summary: 'Metadata boundary scaffolded.' },
          ],
          queues: [{ name: 'preview', priority: 'high', summary: 'Visible previews first.' }],
        });
      }

      return Promise.reject(new Error(`Unexpected command in test: ${command}`));
    });

    render(<App />);

    fireEvent.change(screen.getByLabelText(/folder path/i), {
      target: { value: '/Assignments' },
    });
    fireEvent.click(screen.getByRole('button', { name: /open folder tab/i }));

    expect(screen.getByRole('tab', { name: 'Assignments' })).toBeInTheDocument();

    fireEvent.change(screen.getByLabelText(/folder path/i), {
      target: { value: '/Assignments/Second-Copy' },
    });
    fireEvent.click(screen.getByRole('button', { name: /open folder tab/i }));

    expect(screen.getByRole('tab', { name: 'Second-Copy' })).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: /open folder in new window/i }));

    await waitFor(() => {
      expect(screen.getByText(/opened folder in a separate window/i)).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: /run backend health check/i }));

    await waitFor(() => {
      expect(screen.getByText(/desktop shell baseline is healthy/i)).toBeInTheDocument();
    });

    expect(invokeCommand).toHaveBeenCalledWith(
      'open_folder_in_separate_window',
      expect.objectContaining({
        request: expect.objectContaining({
          folderPath: '/Assignments/Second-Copy',
          activeWorkspace: 'Second-Copy',
        }),
      }),
    );
    expect(screen.getByText('preview')).toBeInTheDocument();
  });

  it('hydrates a new window with its requested startup folder', async () => {
    invokeCommand.mockImplementation((command: string) => {
      if (command === 'consume_window_folder_open_request') {
        return Promise.resolve({
          windowLabel: 'workspace-detached-1',
          folderPath: '/Detached/Workspace',
        });
      }

      return Promise.reject(new Error(`Unexpected command in test: ${command}`));
    });

    render(<App />);

    await waitFor(() => {
      expect(screen.getByRole('tab', { name: 'Workspace' })).toBeInTheDocument();
    });
  });
});
