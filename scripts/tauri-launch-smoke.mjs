import { spawn } from 'node:child_process';

const START_MARKER = 'starting Photo Workroom desktop shell';
const START_TIMEOUT_MS = Number.parseInt(process.env.TAURI_LAUNCH_TIMEOUT_MS ?? '120000', 10);
const SHUTDOWN_GRACE_MS = 1500;
const FORCE_KILL_GRACE_MS = 3000;

const command = 'npm --workspace apps/main-app run tauri:dev -- --no-watch';
const child = spawn(command, {
  cwd: process.cwd(),
  env: { ...process.env, CI: '1' },
  detached: process.platform !== 'win32',
  shell: true,
  stdio: ['ignore', 'pipe', 'pipe'],
});

let sawStartMarker = false;
let requestedShutdown = false;
let timedOut = false;
let forceKillTimer;

function mirrorOutput(chunk) {
  const text = chunk.toString();
  process.stdout.write(text);

  if (!sawStartMarker && text.includes(START_MARKER)) {
    sawStartMarker = true;
    process.stdout.write(`\n[tauri-launch-smoke] Detected startup marker: "${START_MARKER}"\n`);
    setTimeout(() => {
      requestedShutdown = true;
      terminateChild();
    }, SHUTDOWN_GRACE_MS);
  }
}

function terminateChild() {
  if (child.exitCode !== null) {
    return;
  }

  if (process.platform === 'win32') {
    spawn('taskkill', ['/pid', `${child.pid}`, '/t', '/f'], {
      stdio: 'ignore',
      shell: true,
    });
    return;
  }

  try {
    process.kill(-child.pid, 'SIGINT');
  } catch {
    return;
  }
  clearTimeout(forceKillTimer);
  forceKillTimer = setTimeout(() => {
    if (child.exitCode === null) {
      try {
        process.kill(-child.pid, 'SIGTERM');
      } catch {
        // Child already exited.
      }
    }
  }, FORCE_KILL_GRACE_MS);
}

const startupTimeout = setTimeout(() => {
  timedOut = true;
  process.stderr.write(
    `\n[tauri-launch-smoke] Timed out after ${START_TIMEOUT_MS}ms waiting for "${START_MARKER}".\n`,
  );
  terminateChild();
}, START_TIMEOUT_MS);

child.stdout.on('data', mirrorOutput);
child.stderr.on('data', mirrorOutput);

child.on('close', (code, signal) => {
  clearTimeout(startupTimeout);
  clearTimeout(forceKillTimer);

  if (timedOut) {
    process.exit(1);
  }

  if (sawStartMarker && requestedShutdown) {
    process.stdout.write('[tauri-launch-smoke] Launch smoke test passed.\n');
    process.exit(0);
  }

  process.stderr.write(
    `[tauri-launch-smoke] Process exited before launch confirmation (code=${code}, signal=${signal ?? 'none'}).\n`,
  );
  process.exit(1);
});

process.on('SIGINT', () => {
  requestedShutdown = true;
  terminateChild();
});
