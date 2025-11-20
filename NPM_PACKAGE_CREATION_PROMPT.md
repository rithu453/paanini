# Complete NPM Package Creation Prompt for Paanini Language

## Context
I have a complete Rust-based CLI application called **Paanini** - a Sanskrit programming language with Python-like syntax. The Rust package is already published on crates.io as `paanini-lang`. Now I need to create a complete npm package wrapper so JavaScript/Node.js developers can use it without needing Rust installed.

## Project Overview

### What is Paanini?
- **Name**: Paanini (पाणिनि) - Sanskrit Programming Language
- **Type**: CLI tool + interpreter + transpiler + web IDE
- **Language**: Written in Rust
- **Published on crates.io**: `paanini-lang` (package name) with `Paanini` as the executable name
- **Version**: 0.1.0
- **License**: MIT

### Key Features
1. **Sanskrit Keywords**: Uses Devanagari script (e.g., `यदि` for if, `अन्यथा` for else, `दर्श` for print)
2. **Python-like Syntax**: Indentation-based, familiar structure
3. **Multiple Interfaces**: 
   - CLI commands (`Paanini run`, `Paanini build`, etc.)
   - Interactive REPL (`Paanini repl`)
   - Web IDE server (`Paanini serve --port 8080`)
   - File execution and transpilation to Rust binaries
4. **Cross-platform**: Windows, macOS, Linux support

### Current Rust CLI Commands
```bash
Paanini                          # Start REPL (default)
Paanini repl                     # Interactive REPL
Paanini run file.Paanini          # Execute Sanskrit file
Paanini run file.Paanini --verbose
Paanini build file.Paanini        # Transpile to Rust and build
Paanini build file.Paanini -o myapp --release
Paanini serve                    # Start web IDE on port 8080
Paanini serve --port 3000        # Custom port
Paanini --help                   # Show help
Paanini --version                # Show version
Paanini example                  # Show example code
```

### Cargo.toml (Rust Package Metadata)
```toml
[package]
name = "paanini-lang"
version = "0.1.0"
edition = "2021"
description = "Paanini - Sanskrit programming language with Python-like syntax"
license = "MIT"
repository = "https://github.com/YOUR_USERNAME/paanini-lang"
keywords = ["sanskrit", "programming-language", "interpreter", "devanagari", "cli"]
categories = ["command-line-utilities", "development-tools"]

[[bin]]
name = "Paanini"
path = "src/main.rs"

[dependencies]
clap = { version = "4.4", features = ["derive"] }
axum = { version = "0.7", features = ["macros", "json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
tower-http = { version = "0.5", features = ["cors", "fs", "trace"] }
anyhow = "1"
colored = "2.0"
```

## NPM Package Requirements

### Directory Structure
Create in: `c:\Users\marpa\Desktop\Paanini\npm\`

```
npm/
├── package.json          # npm package manifest
├── index.js              # Programmatic API for Node.js users
├── install.js            # Post-install script (downloads/builds binary)
├── bin/
│   └── Paanini.js        # CLI wrapper script
├── README.md            # User documentation
├── LICENSE              # MIT license
├── .gitignore           # Ignore node_modules, binaries, logs
├── test.js              # Test script to verify installation
└── DEVELOPMENT.md       # Developer guide
```

### Package Details
- **NPM Package Name**: `paanini-lang` (same as crates.io)
- **Executable Command**: `Paanini` (for consistency)
- **Version**: 0.1.0 (match Rust version)
- **License**: MIT
- **Repository**: https://github.com/YOUR_USERNAME/paanini-lang

### Platform Support
- **Windows**: `win32` - Binary: `Paanini.exe` - Target: `x86_64-pc-windows-gnu`
- **macOS Intel**: `darwin` + `x64` - Binary: `Paanini` - Target: `x86_64-apple-darwin`
- **macOS Apple Silicon**: `darwin` + `arm64` - Binary: `Paanini` - Target: `aarch64-apple-darwin`
- **Linux**: `linux` - Binary: `Paanini` - Target: `x86_64-unknown-linux-gnu`

### Installation Strategy (Important!)
The npm package should support multiple installation methods:

1. **Try to download pre-built binary** from GitHub releases first:
   - URL format: `https://github.com/YOUR_USERNAME/paanini-lang/releases/download/v{VERSION}/{BINARY_NAME}`
   - Binary names: `Paanini-x86_64-pc-windows-gnu.exe`, `Paanini-x86_64-apple-darwin`, etc.
   
2. **Build from source as fallback** if download fails:
   - Requires Rust/Cargo installed on user's machine
   - Run `cargo build --release` from parent directory
   - Copy binary from `../target/release/Paanini[.exe]` to `./bin/`
   - **CRITICAL FOR WINDOWS**: Must handle MSVC linker issue:
     ```javascript
     // On Windows, switch to GNU toolchain if MSVC linker fails
     if (process.platform === 'win32') {
       execSync('rustup default stable-x86_64-pc-windows-gnu', { stdio: 'pipe' });
     }
     ```
   
3. **Handle binary permissions**:
   - On Unix systems (macOS/Linux): `fs.chmodSync(binaryPath, 0o755)`
   - Windows doesn't need chmod

### package.json Requirements

```json
{
  "name": "paanini-lang",
  "version": "0.1.0",
  "description": "Paanini - Sanskrit programming language with Python-like syntax. CLI tool for running, building, and developing in Sanskrit.",
  "main": "index.js",
  "bin": {
    "Paanini": "./bin/Paanini.js"
  },
  "scripts": {
    "postinstall": "node install.js",
    "test": "node test.js"
  },
  "keywords": [
    "sanskrit",
    "programming-language",
    "cli",
    "devanagari",
    "interpreter",
    "transpiler",
    "Paanini",
    "indian-languages"
  ],
  "author": "Paanini Developers",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/YOUR_USERNAME/paanini-lang.git"
  },
  "engines": {
    "node": ">=14.0.0"
  },
  "os": ["win32", "darwin", "linux"],
  "files": [
    "bin/",
    "install.js",
    "index.js",
    "README.md",
    "LICENSE"
  ]
}
```

### CLI Wrapper (bin/Paanini.js)

Must:
1. Start with shebang: `#!/usr/bin/env node`
2. Detect platform and find correct binary path
3. Use `child_process.spawn()` to execute the binary
4. Pass through all arguments: `process.argv.slice(2)`
5. Inherit stdio for interactive features (REPL)
6. Handle errors gracefully with helpful messages
7. Exit with same code as binary

### Programmatic API (index.js)

Provide these functions for Node.js developers:

```javascript
// Get platform-specific binary path
function getBinaryPath()

// Execute Paanini CLI with arguments (returns Promise)
function runPaanini(args) // Returns: {code, stdout, stderr}

// Run a Paanini file
async function runFile(filePath)

// Transpile to Rust
async function transpile(inputPath, outputPath)

// Start REPL (returns ChildProcess)
function startRepl()

// Start web server (returns ChildProcess)
function startServer(port = 8080)

// Export all functions
module.exports = { getBinaryPath, runPaanini, runFile, transpile, startRepl, startServer }
```

### Install Script (install.js)

Must handle:
1. **Platform detection**: `process.platform` and `process.arch`
2. **Download pre-built binary**:
   - Use `https` module (Node.js built-in)
   - Handle redirects (302/301 status codes)
   - Save to `./bin/Paanini[.exe]`
   - Set executable permissions on Unix
3. **Build from source fallback**:
   - Check if Cargo is installed: `execSync('cargo --version')`
   - Navigate to parent directory (where Cargo.toml is)
   - **Switch to GNU toolchain on Windows** before building
   - Run `cargo build --release`
   - Copy binary from `../target/release/` to `./bin/`
   - Remove existing binary before copying to avoid permission errors
4. **Error handling**:
   - Clear error messages
   - Suggest alternatives (install Rust, use `cargo install paanini-lang`)
   - Exit with code 1 on failure

### Test Script (test.js)

Should verify:
1. Binary exists at correct path
2. Can execute `--version` command
3. Can execute `--help` command
4. Output contains "Paanini"
5. Exit codes are correct

### README.md for npm

Must include:
1. **Installation**: `npm install -g paanini-lang` or `npx paanini-lang`
2. **CLI Usage**: All commands with examples
3. **Programmatic API**: JavaScript examples
4. **Language examples**: Sanskrit code samples
5. **Platform support**: List supported OS
6. **Building from source**: Explain Rust requirement if pre-built fails
7. **Links**: GitHub, crates.io, documentation

### Example Paanini Code (for README)

Include these examples:

```Paanini
!! Variables and Math
चर x = 10
चर y = 20
दर्श("योग:", x + y)

!! Conditionals
यदि x < y:
    दर्श("x is smaller")
अन्यथा:
    दर्श("y is smaller")

!! Loops
चर i = 0
पुनः i < 5:
    दर्श(i)
    i = i + 1

!! Functions (if supported)
कार्य greet(name):
    दर्श("नमस्ते", name)
```

### .gitignore

```
node_modules/
*.log
npm-debug.log*
.DS_Store
bin/Paanini
bin/Paanini.exe
test-output/
```

### DEVELOPMENT.md

Include instructions for:
1. Building Rust binary first
2. Installing npm dependencies
3. Running postinstall script
4. Testing locally with `npm link`
5. Publishing to npm
6. Creating GitHub releases with pre-built binaries

## Important Notes

### Critical Issues to Handle

1. **Windows MSVC Linker Problem**:
   - Windows users may not have Visual Studio installed
   - MSVC toolchain requires `link.exe` from VS Build Tools
   - Solution: Switch to GNU toolchain before building
   - Command: `rustup default stable-x86_64-pc-windows-gnu`
   - This has been tested and works on the development machine

2. **Binary Permission on Windows**:
   - When copying binary on Windows, may get EPERM error if binary already exists
   - Solution: Delete existing binary first with `fs.unlinkSync()` before copying

3. **Path Separators**:
   - Use `path.join()` for cross-platform path handling
   - Don't hardcode forward slashes or backslashes

4. **GitHub Releases**:
   - Pre-built binaries should be uploaded to GitHub releases
   - Use version tags: `v0.1.0`, `v0.1.1`, etc.
   - Binary naming convention: `Paanini-{rust-target}{.exe}`

### Testing Checklist

Before publishing:
- [ ] Test on Windows (x64)
- [ ] Test on macOS (Intel and Apple Silicon if possible)
- [ ] Test on Linux
- [ ] Test `npm install -g` locally
- [ ] Test `npx paanini-lang` without installation
- [ ] Test CLI commands: `Paanini --version`, `Paanini --help`, `Paanini repl`
- [ ] Test programmatic API in Node.js script
- [ ] Verify binary downloads work (if GitHub releases are set up)
- [ ] Verify build from source works
- [ ] Check README renders correctly on npm

### Publishing Steps

1. Build Rust binary: `cargo build --release`
2. Test npm package locally: `cd npm && npm test`
3. Test with npm link: `npm link` then `Paanini --help`
4. Login to npm: `npm login`
5. Publish: `npm publish`
6. (Optional) Create GitHub release with pre-built binaries for all platforms

## AI Assistant Instructions

Please create the complete npm package in the `c:\Users\marpa\Desktop\Paanini\npm\` directory with:

1. **All files listed above** with complete implementations
2. **Robust error handling** for all edge cases
3. **Clear console messages** during installation
4. **Cross-platform compatibility** for Windows, macOS, and Linux
5. **Both CLI and programmatic API** fully implemented
6. **Complete documentation** in README.md and DEVELOPMENT.md
7. **Working test script** that verifies installation
8. **Handle Windows MSVC linker issue** in install.js
9. **Binary download with fallback to source build**
10. **Proper executable permissions** on Unix systems

Make sure everything follows npm best practices and the package is ready to publish after testing.

