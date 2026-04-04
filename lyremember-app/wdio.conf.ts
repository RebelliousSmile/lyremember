import { config as baseConfig } from '@wdio/config';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export const config = {
  runner: 'local',
  specs: ['./tests/e2e/**/*.spec.ts'],
  maxInstances: 1,
  capabilities: [{
    'tauri:options': {
      application: path.resolve(
        __dirname,
        'src-tauri/target/release/lyremember-app'
      ),
    },
  }],
  logLevel: 'warn',
  bail: 0,
  waitforTimeout: 10000,
  connectionRetryTimeout: 120000,
  connectionRetryCount: 3,
  framework: 'mocha',
  reporters: ['spec'],
  mochaOpts: {
    ui: 'bdd',
    timeout: 60000,
  },

  // Start tauri-driver before tests
  onPrepare: function () {
    const { spawn } = require('child_process');
    const tauriDriver = spawn(
      path.resolve(__dirname, 'node_modules/.bin/tauri-driver'),
      [],
      { stdio: ['ignore', 'pipe', 'pipe'] }
    );
    (global as any).__tauriDriver = tauriDriver;
  },

  onComplete: function () {
    const tauriDriver = (global as any).__tauriDriver;
    if (tauriDriver) {
      tauriDriver.kill();
    }
  },
};
