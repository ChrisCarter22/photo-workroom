import { useEffect, useState } from 'react';
import './App.css';
import { requestHealthCheck, type HealthCheckResponse } from './lib/health';
import {
  createInitialShellState,
  getActiveTab,
  openFolderInShell,
  resetShellState,
  type ShellState,
} from './lib/shellState';
import { isTauriRuntime } from './lib/tauri';
import { consumeWindowFolderOpenRequest, openFolderInSeparateWindow } from './lib/windowing';

function App() {
  const [shellState, setShellState] = useState<ShellState>(() => createInitialShellState());
  const [folderPath, setFolderPath] = useState('/Volumes/Card-01/Assignments');
  const [healthResult, setHealthResult] = useState<HealthCheckResponse | null>(null);
  const [healthState, setHealthState] = useState<'idle' | 'loading' | 'error'>('idle');
  const [healthError, setHealthError] = useState<string | null>(null);
  const [windowOpenState, setWindowOpenState] = useState<'idle' | 'loading' | 'success' | 'error'>('idle');
  const [windowOpenMessage, setWindowOpenMessage] = useState<string | null>(null);

  const activeTab = getActiveTab(shellState);

  useEffect(() => {
    if (!isTauriRuntime()) {
      return;
    }

    let canceled = false;
    void consumeWindowFolderOpenRequest()
      .then((response) => {
        const startupFolderPath = response.folderPath;
        if (canceled || startupFolderPath === null) {
          return;
        }

        setShellState((current) => openFolderInShell(current, startupFolderPath));
      })
      .catch((error) => {
        if (canceled) {
          return;
        }

        setWindowOpenState('error');
        setWindowOpenMessage(
          error instanceof Error ? error.message : 'Failed to read startup folder for this window.',
        );
      });

    return () => {
      canceled = true;
    };
  }, []);

  async function runHealthCheck() {
    setHealthState('loading');
    setHealthError(null);

    try {
      const response = await requestHealthCheck({
        requestId: `health-${Date.now()}`,
        activeWorkspace: activeTab.title,
      });

      setHealthResult(response);
      setHealthState('idle');
    } catch (error) {
      setHealthResult(null);
      setHealthState('error');
      setHealthError(error instanceof Error ? error.message : 'Health check failed.');
    }
  }

  async function openFolderInNewWindow() {
    const trimmedPath = folderPath.trim();

    if (!trimmedPath) {
      setWindowOpenState('error');
      setWindowOpenMessage('Enter a folder path before opening a separate window.');
      return;
    }

    if (!isTauriRuntime()) {
      setWindowOpenState('error');
      setWindowOpenMessage('Separate windows require the desktop runtime (`npm run tauri:dev`).');
      return;
    }

    setWindowOpenState('loading');
    setWindowOpenMessage(null);

    try {
      const response = await openFolderInSeparateWindow({
        requestId: `window-open-${Date.now()}`,
        folderPath: trimmedPath,
        activeWorkspace: activeTab.title,
      });

      setWindowOpenState('success');
      setWindowOpenMessage(`${response.message} (${response.windowLabel})`);
    } catch (error) {
      setWindowOpenState('error');
      setWindowOpenMessage(error instanceof Error ? error.message : 'Failed to open a separate window.');
    }
  }

  return (
    <div className="shell">
      <aside className="sidebar">
        <div className="brand-block">
          <p className="eyebrow">Local-first ingest and browse shell</p>
          <h1>Photo Workroom</h1>
          <p className="lede">
            Phase 0 through Phase 2 baseline with a typed Tauri health check and a programmable
            contact-sheet shell.
          </p>
        </div>

        <section className="panel">
          <div className="panel-header">
            <h2>Search</h2>
            <span>planned</span>
          </div>
          <input aria-label="Search query" className="panel-input" placeholder="Search will land after indexing" />
        </section>

        <section className="panel">
          <div className="panel-header">
            <h2>Favorites</h2>
            <span>stub</span>
          </div>
          <ul className="panel-list">
            <li>Recent ingest destinations</li>
            <li>Saved folder worksets</li>
            <li>Quick reopen targets</li>
          </ul>
        </section>

        <section className="panel">
          <div className="panel-header">
            <h2>Navigator</h2>
            <span>interactive</span>
          </div>
          <label className="field">
            <span>Folder path</span>
            <input
              aria-label="Folder path"
              className="panel-input"
              value={folderPath}
              onChange={(event) => setFolderPath(event.target.value)}
              placeholder="/Volumes/Card-01/Assignments"
            />
          </label>
          <div className="button-row">
            <button
              className="primary-button"
              onClick={() => setShellState((current) => openFolderInShell(current, folderPath))}
            >
              Open Folder Tab
            </button>
            <button className="secondary-button" onClick={() => void openFolderInNewWindow()}>
              Open Folder in New Window
            </button>
            <button className="secondary-button" onClick={() => setShellState(resetShellState())}>
              Reset Shell
            </button>
          </div>
          <p className={windowOpenState === 'error' ? 'caption error-text' : 'caption'}>
            {windowOpenState === 'loading'
              ? 'Opening a separate workspace window...'
              : windowOpenMessage ??
                'The first folder replaces the blank Untitled workspace. Later folders open as new tabs, or explicitly in a separate window.'}
          </p>
        </section>

        <section className="panel">
          <div className="panel-header">
            <h2>Tasks</h2>
            <span>visible</span>
          </div>
          <button className="primary-button" onClick={() => void runHealthCheck()}>
            Run Backend Health Check
          </button>
          <p className="caption">Use the Navigator action to open a folder in a dedicated window.</p>
        </section>
      </aside>

      <main className="workspace">
        <header className="tab-strip" aria-label="Workspace tabs">
          {shellState.tabs.map((tab) => (
            <button
              key={tab.id}
              className={tab.id === shellState.activeTabId ? 'tab tab-active' : 'tab'}
              onClick={() => setShellState((current) => ({ ...current, activeTabId: tab.id }))}
              role="tab"
              aria-selected={tab.id === shellState.activeTabId}
            >
              {tab.title}
            </button>
          ))}
        </header>

        <section className="workspace-canvas">
          <div className="workspace-card">
            <div className="workspace-heading">
              <div>
                <p className="eyebrow">Active workspace</p>
                <h2>{activeTab.title}</h2>
              </div>
              <div className="badge-group">
                <span className="badge">{activeTab.kind === 'placeholder' ? 'Blank shell' : 'Contact sheet tab'}</span>
                <span className="badge">v{__APP_VERSION__}</span>
              </div>
            </div>
            <p className="workspace-copy">
              {activeTab.path
                ? `${activeTab.path} is mounted into the shell state and ready for scan, ingest, and thumbnail phases.`
                : 'Open a folder path from Navigator to convert this placeholder into the first contact-sheet tab.'}
            </p>
            <div className="workspace-grid">
              <article className="surface">
                <h3>Contact Sheet</h3>
                <p>Virtualized thumbnail browsing, sorting, and preview requests land in later phases.</p>
              </article>
              <article className="surface">
                <h3>Metadata</h3>
                <p>Caption, keyword, and IPTC editing remain planned behind the future metadata engine.</p>
              </article>
              <article className="surface">
                <h3>Task Priorities</h3>
                <p>Preview work stays ahead of bulk ingest, rename, and delivery queues.</p>
              </article>
            </div>
          </div>

          <div className="health-card">
            <div className="panel-header">
              <h2>Health Check</h2>
              <span>{healthState === 'loading' ? 'running' : healthResult?.healthy ? 'healthy' : 'idle'}</span>
            </div>

            {healthError ? <p className="error-text">{healthError}</p> : null}

            {healthResult ? (
              <>
                <p className="health-message">{healthResult.message}</p>
                <div className="health-metadata">
                  <span>Runtime: {healthResult.runtime}</span>
                  <span>Workspace: {healthResult.activeWorkspace}</span>
                </div>
                <div className="health-section">
                  <h3>Subsystems</h3>
                  <ul>
                    {healthResult.subsystems.map((subsystem) => (
                      <li key={subsystem.name}>
                        <strong>{subsystem.name}</strong>
                        <span>{subsystem.phase}</span>
                        <p>{subsystem.summary}</p>
                      </li>
                    ))}
                  </ul>
                </div>
                <div className="health-section">
                  <h3>Queues</h3>
                  <ul>
                    {healthResult.queues.map((queue) => (
                      <li key={queue.name}>
                        <strong>{queue.name}</strong>
                        <span>{queue.priority}</span>
                        <p>{queue.summary}</p>
                      </li>
                    ))}
                  </ul>
                </div>
              </>
            ) : (
              <p className="health-placeholder">
                No backend sample yet. Run the health check to exercise typed IPC through Tauri.
              </p>
            )}
          </div>
        </section>

        <footer className="status-bar">
          <span>Selection: {shellState.selectionCount}</span>
          <span>Active tab: {activeTab.title}</span>
          <span>Backend route: Tauri invoke</span>
          <span>Version: {__APP_VERSION__}</span>
        </footer>
      </main>
    </div>
  );
}

export default App;
