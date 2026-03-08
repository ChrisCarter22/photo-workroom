import { describe, expect, it } from 'vitest';
import { createInitialShellState, openFolderInShell } from './shellState';

describe('openFolderInShell', () => {
  it('replaces the blank untitled workspace with the first folder tab', () => {
    const state = openFolderInShell(createInitialShellState(), '/Volumes/Card-01/Matchday');

    expect(state.tabs).toHaveLength(1);
    expect(state.tabs[0]).toMatchObject({
      kind: 'contact-sheet',
      title: 'Matchday',
      path: '/Volumes/Card-01/Matchday',
    });
  });

  it('opens subsequent folders as additional tabs', () => {
    const first = openFolderInShell(createInitialShellState(), '/Archive/2026-03-08');
    const second = openFolderInShell(first, '/Archive/2026-03-09');

    expect(second.tabs).toHaveLength(2);
    expect(second.tabs.map((tab) => tab.title)).toEqual(['2026-03-08', '2026-03-09']);
    expect(second.activeTabId).toBe('workspace-2026-03-09');
  });
});

