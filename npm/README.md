# Panini (पाणिनि) — Sanskrit Programming Language for Node.js

Panini is a modern Sanskrit programming language with Python-like syntax, packaged for Node.js developers. This npm wrapper downloads (or builds) the Rust-powered `panini` CLI so you can run Panini code on Windows, macOS, and Linux without installing Rust manually.

> **Rust crate**: [`paanini-lang`](https://crates.io/crates/paanini-lang) · **Version**: 0.1.0 · **License**: MIT

## Installation

```bash
# Global install
npm install -g paanini-lang

# Use instantly without global install
npx paanini-lang --version
```

During installation the wrapper tries to download a prebuilt binary from GitHub Releases. If the download fails, it automatically builds from source (requires Rust via `rustup`).

## Usage

### CLI Commands

Panini provides a familiar CLI workflow. After installing, run:

```bash
panini                      # Start interactive REPL
panini repl                 # Explicit REPL command
panini run path/to/file.panini
panini run file.panini --verbose
panini build file.panini
panini build file.panini -o myapp --release
panini serve                # Launch web IDE (default port 8080)
panini serve --port 3000
panini example              # Show example Sanskrit source
panini --help               # Display CLI help
panini --version            # Show version
```

### Programmatic API

```javascript
const {
  getBinaryPath,
  runPanini,
  runFile,
  transpile,
  startRepl,
  startServer,
} = require('paanini-lang');

(async () => {
  // Execute arbitrary Panini CLI arguments
  const { code, stdout } = await runPanini(['--version']);
  console.log(code, stdout.trim());

  // Run a Panini file
  await runFile('examples/hello.panini');

  // Transpile to Rust
  await transpile('examples/app.panini', 'dist/app');

  // Start the REPL (inherits stdio)
  const repl = startRepl();

  // Start the web IDE on a custom port
  const serverProcess = startServer(3000);
})();
```

All functions throw helpful errors when the binary is missing or an argument is invalid.

### Language Example

```panini
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

!! Functions
कार्य greet(name):
    दर्श("नमस्ते", name)
```

## Platform Support

| Platform | Architecture | Target Triple | Binary |
|----------|--------------|---------------|--------|
| Windows  | x64          | `x86_64-pc-windows-gnu` | `panini-x86_64-pc-windows-gnu.exe` |
| macOS    | x64          | `x86_64-apple-darwin`   | `panini-x86_64-apple-darwin` |
| macOS    | arm64        | `aarch64-apple-darwin`  | `panini-aarch64-apple-darwin` |
| Linux    | x64          | `x86_64-unknown-linux-gnu` | `panini-x86_64-unknown-linux-gnu` |

Prebuilt binaries must be uploaded to GitHub Releases at:
```
https://github.com/YOUR_USERNAME/paanini-lang/releases/download/v0.1.0/<binary-name>
```

## Building from Source

1. Ensure [Rust](https://www.rust-lang.org/tools/install) and Cargo are installed.
2. Clone the Panini repository and build the release binary:
   ```bash
   cargo build --release
   ```
3. The npm installer copies the binary from `target/release/panini[.exe]` into the package's `bin/` directory if downloads are unavailable.
4. On Windows the installer automatically switches to the `stable-x86_64-pc-windows-gnu` toolchain to avoid MSVC linker issues. Ensure `rustup` is installed.

## Troubleshooting

- **Binary missing:** Run `node install.js` inside the `npm/` folder or reinstall the package.
- **Rust not installed:** Install via `rustup` and rerun installation.
- **Custom builds:** Place your compiled `panini` binary into `node_modules/paanini-lang/bin/`.
- **File permissions (macOS/Linux):** The installer applies `chmod 755`, but you can run `chmod +x` manually if needed.

## Links

- GitHub: https://github.com/YOUR_USERNAME/paanini-lang
- Crate: https://crates.io/crates/paanini-lang
- Issues: https://github.com/YOUR_USERNAME/paanini-lang/issues

## License

MIT © Panini Developers
