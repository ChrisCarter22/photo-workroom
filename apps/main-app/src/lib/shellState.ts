export type WorkspaceTabKind = 'placeholder' | 'contact-sheet';

export interface WorkspaceTab {
  id: string;
  kind: WorkspaceTabKind;
  title: string;
  path: string | null;
}

export interface ShellState {
  tabs: WorkspaceTab[];
  activeTabId: string;
  selectionCount: number;
}

const UNTITLED_TAB_ID = 'workspace-untitled';

function basename(folderPath: string): string {
  const normalized = folderPath.trim().replace(/[\\/]+$/, '');
  const parts = normalized.split(/[\\/]+/).filter(Boolean);
  return parts.at(-1) ?? 'Folder';
}

function createContactSheetTab(folderPath: string): WorkspaceTab {
  const title = basename(folderPath);
  const slug = title.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/(^-|-$)/g, '') || 'folder';

  return {
    id: `workspace-${slug}`,
    kind: 'contact-sheet',
    title,
    path: folderPath.trim(),
  };
}

export function createInitialShellState(): ShellState {
  return {
    tabs: [
      {
        id: UNTITLED_TAB_ID,
        kind: 'placeholder',
        title: 'Untitled',
        path: null,
      },
    ],
    activeTabId: UNTITLED_TAB_ID,
    selectionCount: 0,
  };
}

export function getActiveTab(state: ShellState): WorkspaceTab {
  return state.tabs.find((tab) => tab.id === state.activeTabId) ?? state.tabs[0];
}

export function openFolderInShell(state: ShellState, folderPath: string): ShellState {
  const trimmedPath = folderPath.trim();

  if (!trimmedPath) {
    return state;
  }

  const nextTab = createContactSheetTab(trimmedPath);
  const activeTab = getActiveTab(state);

  if (activeTab.kind === 'placeholder' && state.tabs.length === 1) {
    return {
      ...state,
      tabs: [{ ...nextTab, id: UNTITLED_TAB_ID }],
      activeTabId: UNTITLED_TAB_ID,
    };
  }

  const duplicateCount = state.tabs.filter((tab) => tab.title === nextTab.title).length;
  const uniqueTab = duplicateCount === 0 ? nextTab : { ...nextTab, id: `${nextTab.id}-${duplicateCount + 1}` };

  return {
    ...state,
    tabs: [...state.tabs, uniqueTab],
    activeTabId: uniqueTab.id,
  };
}

export function resetShellState(): ShellState {
  return createInitialShellState();
}
