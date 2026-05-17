#!/usr/bin/env node
/**
 * Activate the repo-level .husky pre-commit hook by pointing
 * `core.hooksPath` at it. Runs as `npm prepare`, so any `npm install`
 * (after a fresh clone, or when deps change) wires the hook automatically.
 *
 * No-op when not in a git repo (e.g. installed from a tarball, CI image
 * without .git), or when the .husky directory is missing.
 */
import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';

function activate() {
  try {
    const repoRoot = execSync('git rev-parse --show-toplevel', {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore'],
    }).trim();

    const huskyDir = resolve(repoRoot, '.husky');
    if (!existsSync(huskyDir)) return;

    execSync(`git -C "${repoRoot}" config core.hooksPath .husky`);
    console.log('✓ Pre-commit hooks activated (.husky)');
  } catch {
    // Not a git repo or git not available — silent no-op.
  }
}

activate();
