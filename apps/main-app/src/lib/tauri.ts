import { invoke } from '@tauri-apps/api/core';

export function invokeCommand<TResponse>(command: string, args?: Record<string, unknown>) {
  return invoke<TResponse>(command, args);
}

interface WindowWithTauriInternals extends Window {
  __TAURI_INTERNALS__?: unknown;
}

export function isTauriRuntime(): boolean {
  if (typeof window === 'undefined') {
    return false;
  }

  const runtimeWindow = window as WindowWithTauriInternals;
  return typeof runtimeWindow.__TAURI_INTERNALS__ !== 'undefined';
}
