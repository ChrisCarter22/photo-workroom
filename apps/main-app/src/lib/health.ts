import { invokeCommand } from './tauri';

export interface HealthCheckRequest {
  requestId: string;
  activeWorkspace: string;
}

export interface SubsystemSnapshot {
  name: string;
  phase: string;
  summary: string;
}

export interface QueueSnapshot {
  name: string;
  priority: string;
  summary: string;
}

export interface HealthCheckResponse {
  requestId: string;
  appVersion: string;
  runtime: string;
  activeWorkspace: string;
  healthy: boolean;
  message: string;
  subsystems: SubsystemSnapshot[];
  queues: QueueSnapshot[];
}

export function requestHealthCheck(request: HealthCheckRequest) {
  return invokeCommand<HealthCheckResponse>('health_check', { request });
}

