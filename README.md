# Fig Local Revival

<div align="center">

**Privacy-First Terminal Autocomplete**

[![macOS](https://img.shields.io/badge/macOS-10.15+-lightgrey?style=flat-square&logo=apple)](https://github.com/alxgb5/fig-revival)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue?style=flat-square)](LICENSE)
[![Stars](https://img.shields.io/github/stars/alxgb5/fig-revival?style=flat-square)](https://github.com/alxgb5/fig-revival/stargazers)
[![Last Commit](https://img.shields.io/github/last-commit/alxgb5/fig-revival/main?style=flat-square)](https://github.com/alxgb5/fig-revival/commits/main)

The original Fig autocomplete experience — completely local, zero cloud, privacy-first.

</div>

---

## Quick Install

```bash
git clone https://github.com/alxgb5/fig-revival.git && cd fig-revival && ./install.sh
```

Restart your shell and start typing.

---

## Why Fig Local Revival?

|             | Fig Local Revival      | Amazon Q / Cloud         |
| ----------- | ---------------------- | ------------------------ |
| **Privacy** | ✅ Zero telemetry      | ❌ Cloud data collection |
| **Offline** | ✅ 100% local          | ❌ Requires internet     |
| **Speed**   | ✅ Instant suggestions | ❌ Network latency       |
| **Setup**   | ✅ No account required | ❌ AWS account needed    |
| **Size**    | ✅ ~50MB               | ❌ 200MB+                |

---

## Features

- **🔒 Privacy First** — Your commands never leave your machine
- **⚡ Lightning Fast** — Zero network latency, pure Rust performance
- **🌐 100% Offline** — Works anywhere, no internet required
- **🎨 Original UI** — Beautiful transparent overlay you know and love
- **🛠️ Built-in Commands** — git, npm, docker, cd, and more

---

## Tech Stack

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC107?style=for-the-badge&logo=tauri)
![React](https://img.shields.io/badge/React-61DAFB?style=for-the-badge&logo=react)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript)

</div>

---

## Architecture

```
Shell (zsh/bash) → figterm (PTY) → fig_local_provider → fig_desktop (UI)
```

- **figterm** — Intercepts terminal buffer, detects what you're typing
- **fig_local_provider** — Generates suggestions locally (pure Rust)
- **fig_proto** — IPC via protobuf
- **fig_desktop** — Overlay UI using Tauri (tao + wry)

---

## Development

### Build

```bash
# Build release binary
cargo build --release

# Or just the CLI
cargo build --release --bin fig_cli
```

### Run

```bash
cargo run --bin fig_cli
```

### Test

```bash
cargo test
```

### Code Quality

```bash
cargo fmt
cargo clippy --locked --workspace -- -D warnings
```

---

## What Changed from Original?

**Removed:**

- All AWS/Cloud dependencies (7 crates, 200K+ lines)
- Amazon Q AI features (Bedrock API)
- Cloud telemetry and analytics
- Network authentication

**Added:**

- `fig_local_provider` — Pure Rust local suggestion engine
- Privacy-first architecture
- Stubbed auth/telemetry (no-op)

---

## License

MIT OR Apache-2.0 — same as original project.

---

<div align="center">

**Based on [Fig](https://github.com/withfig/autocomplete) and [Amazon Q Developer CLI](https://github.com/aws/amazon-q-developer-cli)**

[Website](https://alxgb5.github.io/fig-revival) • [Issues](https://github.com/alxgb5/fig-revival/issues)

</div>
