# Paanini (पाणिनि) — Sanskrit Programming Language for Node.js

Paanini is a modern Sanskrit programming language with Python-like syntax, packaged for Node.js developers. This npm wrapper downloads (or builds) the Rust-powered `paanini` CLI so you can run Paanini code on Windows, macOS, and Linux without installing Rust manually.

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

Paanini provides a familiar CLI workflow. After installing, run:

```bash
paanini                      # Start interactive REPL
paanini repl                 # Explicit REPL command
paanini run path/to/file.paanini
paanini run file.paanini --verbose
paanini build file.paanini
paanini build file.paanini -o myapp --release
paanini serve                # Launch web IDE (default port 8080)
paanini serve --port 3000
paanini example              # Show example Sanskrit source
paanini --help               # Display CLI help
paanini --version            # Show version
```

### Programmatic API

```javascript
const {
  getBinaryPath,
  runPaanini,
  runFile,
  transpile,
  startRepl,
  startServer,
} = require('paanini-lang');

(async () => {
  // Execute arbitrary Paanini CLI arguments
  const { code, stdout } = await runPaanini(['--version']);
  console.log(code, stdout.trim());

  // Run a Paanini file
  await runFile('examples/hello.paanini');

  // Transpile to Rust
  await transpile('examples/app.paanini', 'dist/app');

  // Start the REPL (inherits stdio)
  const repl = startRepl();

  // Start the web IDE on a custom port
  const serverProcess = startServer(3000);
})();
```

All functions throw helpful errors when the binary is missing or an argument is invalid.

### Language Example

```paanini
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
| Windows  | x64          | `x86_64-pc-windows-gnu` | `paanini-x86_64-pc-windows-gnu.exe` |
| macOS    | x64          | `x86_64-apple-darwin`   | `paanini-x86_64-apple-darwin` |
| macOS    | arm64        | `aarch64-apple-darwin`  | `paanini-aarch64-apple-darwin` |
| Linux    | x64          | `x86_64-unknown-linux-gnu` | `paanini-x86_64-unknown-linux-gnu` |

Prebuilt binaries must be uploaded to GitHub Releases at:
```
https://github.com/YOUR_USERNAME/paanini-lang/releases/download/v0.1.0/<binary-name>
```

## Building from Source

1. Ensure [Rust](https://www.rust-lang.org/tools/install) and Cargo are installed.
2. Clone the Paanini repository and build the release binary:
   ```bash
   cargo build --release
   ```
3. The npm installer copies the binary from `target/release/paanini[.exe]` into the package's `bin/` directory if downloads are unavailable.
4. On Windows the installer automatically switches to the `stable-x86_64-pc-windows-gnu` toolchain to avoid MSVC linker issues. Ensure `rustup` is installed.

## Troubleshooting

- **Binary missing:** Run `node install.js` inside the `npm/` folder or reinstall the package.
- **Rust not installed:** Install via `rustup` and rerun installation.
- **Custom builds:** Place your compiled `paanini` binary into `node_modules/paanini-lang/bin/`.
- **File permissions (macOS/Linux):** The installer applies `chmod 755`, but you can run `chmod +x` manually if needed.

## Links

- GitHub: https://github.com/rithu453/paanini/releases/tag/v0.1.0
- Crate: https://crates.io/crates/paanini-lang
- Issues: https://github.com/rithu453/paanini/issues

## License

MIT © Paanini Developers
