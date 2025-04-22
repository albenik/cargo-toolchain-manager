# Cargo Toolchain Manager

`cargo-toolchain-manager`

📦 A CLI utility to install all (or selected) Rust versions via `rustup`.

Useful for developers, CI/CD pipelines, and testing environments where multiple Rust versions are required.

## 🚀 Features

- ✅ Install all **stable** versions of Rust
- ℹ️ Use latest patch of stable version (e.g. install `1.85-<arch>-<target>` toolchain equals to `1.85.1` but not
  `1.85.0`)
- 🔢 Filter versions using `semver` ranges
- 🧪 `--dry-run` mode to preview without installing

## 📦 Installation

```bash
cargo install cargo-toolchain-manager

# Or install from Git:
cargo install --git https://github.com/your-username/cargo-toolchain
```

## 🔧 Usage Examples

```bash
# Install all stable versions
cargo toolchain-manager install

# Dry-run without installing
cargo toolchain-manager install --dry-run

# Install versions in a specific semver range
cargo toolchain-manager --range ">=1.60,<1.70"
```

## 📜 CLI Options

| Option    | Description                                |
|-----------|--------------------------------------------|
| --dry-run | Only list versions that would be installed |
| --range   | Filter versions with a semver range        |

## 🔐 Requirements

- Rustup
- Rust installed

## 📃 License

MIT