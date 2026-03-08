import { describe, expect, it } from 'vitest';
import { createInitialShellState, openFolderInShell } from './shellState';

describe('shellState', () => {
  it('starts with a single untitled placeholder tab', () => {
    const state = createInitialShellState();

    expect(state.tabs).toHaveLength(1);
    expect(state.tabs[0]).toMatchObject({
      kind: 'placeholder',
      title: 'Untitled',
      path: null,
    });
  });

  it('ignores empty folder requests', () => {
    const state = createInitialShellState();

    expect(openFolderInShell(state, '   ')).toEqual(state);
  });
});

