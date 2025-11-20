#!/usr/bin/env node

const { spawn } = require('child_process');

let getBinaryPath;
try {
  ({ getBinaryPath } = require('../index'));
} catch (error) {
  console.error('Panini npm wrapper failed to load. Please ensure install.js completed successfully.');
  console.error(error instanceof Error ? error.message : error);
  process.exit(1);
}

function run() {
  const binaryPath = (() => {
    try {
      return getBinaryPath();
    } catch (error) {
      console.error('Panini binary not found. Try reinstalling the package or run `node install.js`.');
      console.error(error instanceof Error ? error.message : error);
      process.exit(1);
    }
  })();

  const args = process.argv.slice(2);
  const child = spawn(binaryPath, args, { stdio: 'inherit' });

  child.on('error', (error) => {
    console.error('Failed to start Panini CLI:', error.message);
    process.exit(error.code || 1);
  });

  child.on('close', (code) => {
    process.exit(code ?? 0);
  });
}

run();
