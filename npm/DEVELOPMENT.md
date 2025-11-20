# Paanini npm Wrapper — Development Guide

This document describes how to work on the `paanini-lang` npm package that wraps the Rust-based Paanini CLI.

## Prerequisites

- Rust toolchain installed via [rustup](https://rustup.rs/)
- Node.js v14 or newer
- GitHub account with access to the `paanini-lang` repository

## 1. Build the Rust Binary

1. From the repository root (one level above this `npm/` folder), build the release binary:
   ```bash
   cargo build --release
   ```
2. The resulting executable is at `target/release/Paanini` (or `Paanini.exe` on Windows).

## 2. Install npm Dependencies

1. Enter the npm wrapper directory:
   ```bash
   cd npm
   ```
2. Install dependencies (none today, but this ensures lockfiles are updated if added later):
   ```bash
   npm install
   ```

## 3. Run the Postinstall Script Manually (Optional)

Re-run the installer to copy/build the binary after local changes:
```bash
node install.js
```
This attempts to download a release binary and falls back to `cargo build --release`.

## 4. Test the Package

Execute the built-in smoke tests:
```bash
npm test
```
`test.js` verifies that the binary exists and that `Paanini --version` and `Paanini --help` succeed.

## 5. Test via npm Link

1. From the `npm/` directory run:
   ```bash
   npm link
   ```
2. In another terminal, invoke the CLI globally:
   ```bash
   Paanini --help
   ```
3. Remove the link after testing:
   ```bash
   npm unlink --global paanini-lang
   ```

## 6. Publish to npm

1. Ensure the binary is present in `bin/` and the version matches `package.json`.
2. Login to npm:
   ```bash
   npm login
   ```
3. Publish:
   ```bash
   npm publish
   ```
   Use `npm publish --access public` if publishing under a scope for the first time.

## 7. Create GitHub Releases with Prebuilt Binaries

1. Build Paanini for each target:
   - Windows: `cargo build --release --target x86_64-pc-windows-gnu`
   - macOS (Intel): `cargo build --release --target x86_64-apple-darwin`
   - macOS (Apple Silicon): `cargo build --release --target aarch64-apple-darwin`
   - Linux: `cargo build --release --target x86_64-unknown-linux-gnu`
2. Rename the binaries to match installer expectations:
   - `Paanini-x86_64-pc-windows-gnu.exe`
   - `Paanini-x86_64-apple-darwin`
   - `Paanini-aarch64-apple-darwin`
   - `Paanini-x86_64-unknown-linux-gnu`
3. Create a release tagged `v0.1.0` (update the tag for future versions) and upload the binaries.
4. Update the changelog/notes and publish the release.

## 8. Versioning Workflow

1. Update the version in both `Cargo.toml` and `npm/package.json`.
2. Commit the changes and tag the commit: `git tag v0.x.y`.
3. Push tags and publish the crate (`cargo publish`) and npm package (`npm publish`).

## 9. Windows Toolchain Notes

- The installer sets the default toolchain to `stable-x86_64-pc-windows-gnu` when building from source.
- Developers can switch back after publishing via:
  ```bash
  rustup default stable-x86_64-pc-windows-msvc
  ```
- Ensure the GNU toolchain is installed: `rustup toolchain install stable-x86_64-pc-windows-gnu`.

## 10. Support & Issue Reporting

- File bugs or feature requests at https://github.com/YOUR_USERNAME/paanini-lang/issues.
- Include platform, architecture, Node.js version, and installation logs when reporting installer problems.

