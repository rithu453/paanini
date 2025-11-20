#!/usr/bin/env node

const fs = require('fs');
const { spawnSync } = require('child_process');
const { getBinaryPath } = require('./index');

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

function runCheck(args, description) {
  const binaryPath = getBinaryPath();
  const result = spawnSync(binaryPath, args, { encoding: 'utf8' });
  assert(result.error == null, `${description} failed to execute: ${result.error}`);
  assert(result.status === 0, `${description} exited with code ${result.status}. stderr: ${result.stderr}`);
  const combinedOutput = `${result.stdout || ''}${result.stderr || ''}`;
  assert(/Panini/i.test(combinedOutput), `${description} output does not mention Panini.`);
  console.log(`✔ ${description}`);
}

(function main() {
  try {
    const binaryPath = getBinaryPath();
    assert(fs.existsSync(binaryPath), `Panini binary not found at ${binaryPath}.`);
    console.log(`Panini binary located at ${binaryPath}`);

    runCheck(['--version'], 'panini --version');
    runCheck(['--help'], 'panini --help');

    console.log('All Panini tests passed.');
  } catch (error) {
    console.error('Panini npm package test failed.');
    console.error(error instanceof Error ? error.message : error);
    process.exit(1);
  }
})();
