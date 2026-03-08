import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import App from './App';

const invokeCommand = vi.fn();

vi.mock('./lib/tauri', () => ({
  invokeCommand: (...args: unknown[]) => invokeCommand(...args),
}));

describe('App shell smoke flow', () => {
  beforeEach(() => {
    invokeCommand.mockReset();
  });

  it('opens folders into tabs and renders a backend health result', async () => {
    invokeCommand.mockResolvedValue({
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

    fireEvent.click(screen.getByRole('button', { name: /run backend health check/i }));

    await waitFor(() => {
      expect(screen.getByText(/desktop shell baseline is healthy/i)).toBeInTheDocument();
    });

    expect(screen.getByText('preview')).toBeInTheDocument();
  });
});
