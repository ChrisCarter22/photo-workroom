import { invokeCommand } from './tauri';

export interface OpenFolderInSeparateWindowRequest {
  requestId: string;
  folderPath: string;
  activeWorkspace: string;
}

export interface OpenFolderInSeparateWindowResponse {
  requestId: string;
  windowLabel: string;
  folderPath: string;
  activeWorkspace: string;
  opened: boolean;
  message: string;
}

export interface ConsumeWindowFolderOpenRequestResponse {
  windowLabel: string;
  folderPath: string | null;
}

export function openFolderInSeparateWindow(request: OpenFolderInSeparateWindowRequest) {
  return invokeCommand<OpenFolderInSeparateWindowResponse>('open_folder_in_separate_window', {
    request,
  });
}

export function consumeWindowFolderOpenRequest() {
  return invokeCommand<ConsumeWindowFolderOpenRequestResponse>('consume_window_folder_open_request');
}
