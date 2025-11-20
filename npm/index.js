const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const PLATFORM_BIN = process.platform === 'win32' ? 'paanini.exe' : 'paanini';

function getBinaryPath() {
  const binPath = path.join(__dirname, 'bin', PLATFORM_BIN);
  if (!fs.existsSync(binPath)) {
    throw new Error(`Paanini binary not found at ${binPath}. Try reinstalling the package or run \`node install.js\`.`);
  }
  return binPath;
}

function runPaanini(args = []) {
  return new Promise((resolve, reject) => {
    const normalizedArgs = Array.isArray(args) ? args.map(String) : [String(args)];
    let stdout = '';
    let stderr = '';

    let child;
    try {
      child = spawn(getBinaryPath(), normalizedArgs, { stdio: ['ignore', 'pipe', 'pipe'] });
    } catch (error) {
      return reject(error);
    }

    child.stdout.on('data', (data) => {
      stdout += data.toString();
    });

    child.stderr.on('data', (data) => {
      stderr += data.toString();
    });

    child.on('error', (error) => {
      reject(error);
    });

    child.on('close', (code) => {
      resolve({ code, stdout, stderr });
    });
  });
}

async function runFile(filePath) {
  if (!filePath) {
    throw new Error('runFile requires a file path argument.');
  }
  return runPaanini(['run', path.resolve(filePath)]);
}

async function transpile(inputPath, outputPath) {
  if (!inputPath) {
    throw new Error('transpile requires an input path argument.');
  }
  if (!outputPath) {
    throw new Error('transpile requires an output path argument.');
  }
  return runPaanini(['build', path.resolve(inputPath), '-o', path.resolve(outputPath)]);
}

function startRepl(options = {}) {
  const child = spawn(getBinaryPath(), ['repl'], { stdio: 'inherit', ...options });
  return child;
}

function startServer(port = 8080, options = {}) {
  const args = ['serve', '--port', String(port)];
  const child = spawn(getBinaryPath(), args, { stdio: 'inherit', ...options });
  return child;
}

module.exports = {
  getBinaryPath,
  runPaanini,
  runFile,
  transpile,
  startRepl,
  startServer,
};
