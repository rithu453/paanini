#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const packageJson = require('./package.json');
const VERSION = packageJson.version;
const REPO_OWNER = 'rithu453';
const REPO_NAME = 'paanini';
const BIN_DIR = path.join(__dirname, 'bin');
const PLATFORM_BIN = process.platform === 'win32' ? 'paanini.exe' : 'paanini';
const BINARY_PATH = path.join(BIN_DIR, PLATFORM_BIN);

const PLATFORM_TARGETS = {
  win32: {
    x64: { target: 'x86_64-pc-windows-gnu', asset: 'paanini-x86_64-pc-windows-gnu.exe' },
  },
  darwin: {
    x64: { target: 'x86_64-apple-darwin', asset: 'paanini-x86_64-apple-darwin' },
    arm64: { target: 'aarch64-apple-darwin', asset: 'paanini-aarch64-apple-darwin' },
  },
  linux: {
    x64: { target: 'x86_64-unknown-linux-gnu', asset: 'paanini-x86_64-unknown-linux-gnu' },
  },
};

function ensureDir(dirPath) {
  fs.mkdirSync(dirPath, { recursive: true });
}

function resolveTarget() {
  const platformInfo = PLATFORM_TARGETS[process.platform];
  if (!platformInfo) {
    throw new Error(`Unsupported platform: ${process.platform}. Supported: ${Object.keys(PLATFORM_TARGETS).join(', ')}`);
  }

  const archInfo = platformInfo[process.arch];
  if (!archInfo) {
    throw new Error(`Unsupported architecture: ${process.arch}. Supported for ${process.platform}: ${Object.keys(platformInfo).join(', ')}`);
  }

  return archInfo;
}

function setExecutablePermissions(filePath) {
  if (process.platform !== 'win32') {
    fs.chmodSync(filePath, 0o755);
  }
}

function downloadBinary(url, destination) {
  return new Promise((resolve, reject) => {
    const maxRedirects = 5;
    const tempDestination = `${destination}.download`;

    function requestBinary(currentUrl, redirectCount = 0) {
      if (redirectCount > maxRedirects) {
        reject(new Error('Too many redirects while downloading Paanini binary.'));
        return;
      }

      https
        .get(currentUrl, (response) => {
          const statusCode = response.statusCode || 0;

          if ([301, 302, 303, 307, 308].includes(statusCode)) {
            const location = response.headers.location;
            if (!location) {
              reject(new Error('Redirect location missing while downloading Paanini binary.'));
              return;
            }
            const redirectedUrl = new URL(location, currentUrl).toString();
            response.resume();
            requestBinary(redirectedUrl, redirectCount + 1);
            return;
          }

          if (statusCode !== 200) {
            reject(new Error(`Download failed with status code ${statusCode}.`));
            response.resume();
            return;
          }

          const fileStream = fs.createWriteStream(tempDestination);
          response.pipe(fileStream);

          fileStream.on('finish', () => {
            fileStream.close((closeErr) => {
              if (closeErr) {
                reject(closeErr);
                return;
              }
              try {
                setExecutablePermissions(tempDestination);
                if (fs.existsSync(destination)) {
                  fs.unlinkSync(destination);
                }
                fs.renameSync(tempDestination, destination);
                resolve();
              } catch (permissionError) {
                try {
                  if (fs.existsSync(tempDestination)) {
                    fs.unlinkSync(tempDestination);
                  }
                } catch (_) {
                  // ignore cleanup errors
                }
                reject(permissionError);
              }
            });
          });

          fileStream.on('error', (streamError) => {
            fileStream.close(() => {
              fs.unlink(tempDestination, () => reject(streamError));
            });
          });
        })
        .on('error', (error) => {
          reject(error);
        });
    }

    requestBinary(url);
  });
}

function cargoAvailable() {
  try {
    execSync('cargo --version', { stdio: 'ignore' });
    return true;
  } catch (_) {
    return false;
  }
}

function switchToGnuToolchain() {
  if (process.platform !== 'win32') {
    return;
  }

  console.log('Switching Rust toolchain to GNU to avoid MSVC linker issues...');
  execSync('rustup default stable-x86_64-pc-windows-gnu', { stdio: 'inherit' });
}

function buildFromSource() {
  if (!cargoAvailable()) {
    throw new Error('Cargo is not installed. Install Rust from https://rustup.rs/ or use `cargo install paanini-lang`.');
  }

  const projectRoot = path.resolve(__dirname, '..');
  const expectedCargo = path.join(projectRoot, 'Cargo.toml');

  if (!fs.existsSync(expectedCargo)) {
    throw new Error(`Cargo project not found. Expected Cargo.toml at ${expectedCargo}.`);
  }

  if (process.platform === 'win32') {
    switchToGnuToolchain();
  }

  console.log('Building Paanini from source with `cargo build --release`...');
  execSync('cargo build --release', { cwd: projectRoot, stdio: 'inherit' });

  const builtBinary = path.join(projectRoot, 'target', 'release', PLATFORM_BIN);
  if (!fs.existsSync(builtBinary)) {
    throw new Error(`Cargo build succeeded but binary not found at ${builtBinary}.`);
  }

  removeExistingBinary();
  fs.copyFileSync(builtBinary, BINARY_PATH);
  setExecutablePermissions(BINARY_PATH);
}

async function install() {
  ensureDir(BIN_DIR);

  const bundledBinaryExists = fs.existsSync(BINARY_PATH);

  const { asset } = resolveTarget();
  const downloadUrl = `https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/v${VERSION}/${asset}`;

  console.log(`Attempting to download Paanini binary from ${downloadUrl}`);

  try {
    await downloadBinary(downloadUrl, BINARY_PATH);
    console.log('Paanini binary downloaded successfully.');
    return;
  } catch (downloadError) {
    console.warn(`Binary download failed: ${downloadError.message}`);
  }

  if (bundledBinaryExists && fs.existsSync(BINARY_PATH)) {
    console.log('Using bundled Paanini binary included with the npm package.');
    return;
  }

  if (bundledBinaryExists) {
    console.log('Falling back to bundled Paanini binary included with the npm package.');
    return;
  }

  try {
    console.warn('Falling back to building Paanini from source. This may take a while.');
    buildFromSource();
    console.log('Paanini built from source successfully.');
  } catch (buildError) {
    throw new Error(`Failed to build Paanini from source: ${buildError.message}`);
  }
}

(async () => {
  console.log(`Installing Paanini v${VERSION} for ${process.platform} (${process.arch})`);
  try {
    await install();
    console.log('Paanini installation complete.');
  } catch (error) {
    console.error('Paanini installation failed.');
    console.error(error instanceof Error ? error.message : error);
    console.error('If the issue persists, ensure Rust is installed and try `cargo install paanini-lang` or open an issue on GitHub.');
    process.exit(1);
  }
})();
