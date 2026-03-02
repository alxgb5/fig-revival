# 🎯 Fig Local Revival

**Privacy-First Terminal Autocomplete • 100% Local • Zero Cloud**

> Revival of the original Fig autocomplete experience, completely freed from cloud dependencies.

[![Website](https://img.shields.io/badge/Website-fig--revival-blue)](https://alxgb5.github.io/fig-revival/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-green)](LICENSE)

---

## ✨ What is Fig Local Revival?

Fig Local Revival brings back the beloved Fig terminal autocomplete experience with one key difference: **everything runs locally**. No cloud, no telemetry, no data collection. Just pure, fast, privacy-first autocomplete for your terminal.

### Why This Fork?

- 🔒 **Privacy First**: Your commands never leave your machine
- ⚡ **Lightning Fast**: Zero network latency, instant suggestions
- 🌐 **100% Offline**: Works anywhere, no internet required
- 📦 **Lightweight**: Removed 200,000+ lines of cloud SDK code
- 🎨 **Original UI**: The beautiful Fig interface you know and love

---

## 🚀 Quick Start

### Prerequisites

- **macOS** 10.15+ (Catalina or later)
- Xcode Command Line Tools: `xcode-select --install`
- Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Installation

```bash
# Clone the repository
git clone https://github.com/alxgb5/fig-revival.git
cd fig-revival

# Run the installer
./install.sh
```

That's it! Restart your terminal and start typing. 🎉

---

## 🎯 Features

| Feature            | Description                                            |
| ------------------ | ------------------------------------------------------ |
| **🔒 Privacy**     | Zero telemetry, zero cloud calls, zero data collection |
| **⚡ Speed**       | Instant suggestions with no network latency            |
| **🌐 Offline**     | Works 100% offline, perfect for air-gapped systems     |
| **🎨 UI**          | Beautiful transparent overlay with keyboard navigation |
| **🛠️ Commands**    | Built-in support for git, npm, docker, cd, and more    |
| **📦 Lightweight** | Minimal footprint, no bloated SDKs                     |

---

## 📖 How It Works

```
Shell (zsh/bash) → figterm (PTY interceptor) → fig_proto (IPC)
                                                    ↓
                   fig_desktop (UI) ← fig_local_provider (suggestions)
```

**Architecture:**

- `figterm`: Intercepts your terminal buffer to detect what you're typing
- `fig_local_provider`: Generates suggestions locally (no network calls)
- `fig_proto`: IPC communication via protobuf
- `fig_desktop`: Beautiful overlay UI using tao/wry
- `fig_cli`: Command-line interface

---

## 🛠️ Development

### Build from Source

```bash
# Build release binaries
cargo build --release --bin fig_cli

# Build all workspace crates
cargo build --release

# Run tests
cargo test
```

### Run Locally

```bash
# Run the CLI
cargo run --bin fig_cli

# Run with specific commands
cargo run --bin fig_cli -- login

# Run tests for specific crate
cargo test -p fig_cli
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --locked --workspace -- -D warnings
```

---

## 📁 Project Structure

```
fig-revival/
├── crates/
│   ├── fig_cli/           # CLI entry point (renamed from q_cli)
│   ├── figterm/           # Terminal interceptor (PTY)
│   ├── fig_desktop/       # UI overlay (tao + wry)
│   ├── fig_local_provider/ # Local suggestion engine (NEW)
│   ├── fig_proto/         # IPC protocol (protobuf)
│   ├── fig_auth/          # Authentication (stubbed for local)
│   └── fig_telemetry/     # Telemetry (no-op stubs)
├── packages/
│   ├── autocomplete/      # React autocomplete app
│   └── dashboard-app/     # Dashboard React app
├── extensions/
│   ├── vscode/           # VSCode extension
│   └── jetbrains/        # JetBrains plugin
└── docs/                 # GitHub Pages landing page
```

---

## 🔧 Troubleshooting

### Binary not found

```bash
# Check installation
which fig
ls -la /usr/local/bin/fig*

# Reinstall
./install.sh
```

### Shell integration not working

```bash
# Reinstall integration
fig integrations uninstall zsh
fig integrations install zsh

# Restart shell
exec zsh
```

### Autocomplete not appearing

1. Make sure `fig_desktop` is running: `ps aux | grep fig_desktop`
2. Check logs: `fig debug logs`
3. Try restarting: `fig restart`

---

## 📝 What Changed from Original?

### ❌ Removed

- All AWS/Cloud dependencies (7 crates, 200,000+ lines)
- Amazon Q AI features (Bedrock API)
- Cloud telemetry and analytics
- Network authentication requirements

### ✅ Added

- `fig_local_provider` - Pure Rust local suggestion engine
- Privacy-first architecture
- Stubbed auth/telemetry (no-op)
- Simplified codebase

### 🔄 Renamed

- `q_cli` → `fig_cli` (binary and crate)
- Removed all Amazon Q branding

---

## 🙏 Credits

This project is based on:

- [Fig](https://github.com/withfig/autocomplete) - The original terminal autocomplete tool
- [Amazon Q Developer CLI](https://github.com/aws/amazon-q-developer-cli) - The fork that became this revival

Special thanks to the original Fig team for creating such an amazing tool.

---

## ⚠️ Disclaimer

This is a **community fork** focused on privacy and local-only operation. All cloud/AI features have been removed.

- ✅ Use this if you want: Privacy, offline operation, no telemetry
- ❌ Don't use this if you need: AWS integration, cloud AI features

For the official AWS-integrated version, see [Amazon Q Developer CLI](https://github.com/aws/amazon-q-developer-cli).

---

## 📜 License

Dual licensed under MIT OR Apache-2.0 (same as original project).

---

## 🌐 Links

- **Website**: https://alxgb5.github.io/fig-revival/
- **Repository**: https://github.com/alxgb5/fig-revival
- **Issues**: https://github.com/alxgb5/fig-revival/issues

---

<div align="center">
  <p><strong>Made with ❤️ for the terminal</strong></p>
  <p>Privacy-first, cloud-free terminal autocomplete</p>
</div>
