# 🎉 Fig Local Revival

**100% Local, Zero AWS, Pure Offline Terminal Autocomplete**

This is a fork of [amazon-q-developer-cli](https://github.com/aws/amazon-q-developer-cli) with **all AWS/Bedrock AI dependencies removed**. It brings back the original Fig experience: fast, local, privacy-first shell autocomplete.

## ✨ What Changed?

### ❌ Removed

- All Amazon Q / AWS Bedrock AI functionality
- All cloud API calls and telemetry
- 7 AWS crates (`amzn-*`, `fig_aws_common`, `fig_api_client`)
- 200,000+ lines of AWS SDK code

### ✅ Added

- `fig_local_provider`: Pure Rust local suggestion engine
- Privacy-first architecture (no network calls)
- Lightweight binaries (no AWS SDK bloat)

## 🚀 Quick Start

### Prerequisites

- **macOS** 10.15+ (Catalina or later)
- Xcode Command Line Tools: `xcode-select --install`
- Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Installation

```bash
# Clone and build
git clone https://github.com/YOUR_USERNAME/fig-local-revival.git
cd fig-revival
cargo build --release

# Install binaries
sudo cp target/release/q /usr/local/bin/fig
sudo cp target/release/figterm /usr/local/bin/
sudo cp target/release/fig_desktop /usr/local/bin/

# Setup shell integration (ZSH)
fig integrations install zsh

# Restart your terminal
exec zsh
```

## 🎯 Features

- **Local Autocomplete**: Built-in suggestions for `git`, `npm`, `cd`, `docker`, etc.
- **No Cloud Deps**: Works 100% offline
- **Privacy First**: Zero telemetry, zero AWS
- **Fast**: No network latency
- **Original Fig UI**: Transparent overlay window with keyboard navigation

## 📦 Architecture

```
Shell → figterm (PTY) → fig_proto (IPC) → fig_local_provider → fig_desktop (UI)
```

**Core Components:**

- `figterm/`: Shell interceptor and PTY manager
- `fig_local_provider/`: Local suggestion engine (NEW)
- `fig_desktop/`: UI overlay (tao + wry webview)
- `fig_proto/`: IPC protocol (protobuf)
- `q_cli/`: CLI entry point

## 🛠️ Development

```bash
# Build everything
cargo build --release

# Run tests
cargo test

# Build specific crate
cargo build -p fig_local_provider

# Run desktop overlay
./target/release/fig_desktop
```

## 📝 Git History

**Commits:**

1. `🔪 Purge Amazon Q/AWS dependencies` - Remove all AWS crates
2. `🛡️ Add local_provider` - Create offline suggestion engine
3. `📚 Documentation` - README + installation guide

## 🔧 Troubleshooting

**Binary not found:**

```bash
# Check installation
which fig
ls -la /usr/local/bin/fig*
```

**Shell integration not working:**

```bash
# Reinstall integration
fig integrations uninstall zsh
fig integrations install zsh
exec zsh
```

## 📜 License

MIT OR Apache-2.0 (same as original project)

## 🙏 Credits

Based on [Amazon Q Developer CLI](https://github.com/aws/amazon-q-developer-cli), originally forked from the [Fig](https://github.com/withfig/autocomplete) project.

---

**⚠️ Note:** This fork removes all AI/cloud features. For AWS integration, use the [official Amazon Q CLI](https://github.com/aws/amazon-q-developer-cli).

- [Ubuntu/Debian](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-installing.html#command-line-installing-ubuntu)
- [AppImage](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-installing.html#command-line-installing-appimage)
- [Alternative Linux builds](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-installing.html#command-line-installing-alternative-linux)
- **Windows**:
  - Follow the discussions for
    [Windows](https://github.com/aws/q-command-line-discussions/discussions/15)
  - Or [use it on Windows with WSL](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-installing.html#command-line-installing-windows)
- **Remote machines**
  - [Autocomplete in SSH](https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line-autocomplete-ssh.html)

## 🚀 Start Contributing

### Prerequisites

- MacOS
  - Xcode 13 or later
  - Brew

### 1. Clone repo

```shell
git clone https://github.com/aws/amazon-q-developer-cli-autocomplete.git
```

### 2. Setup

Hassle-free setup:

```shell
npm run setup
```

Or if you'd like to DIY:

<details>
<summary>Manual Setup</summary>
<div>

### 1. Install platform dependencies

For Debian/Ubuntu:

```shell
sudo apt update
sudo apt install build-essential pkg-config jq dpkg curl wget cmake clang libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libdbus-1-dev libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev valac libibus-1.0-dev libglib2.0-dev sqlite3 libxdo-dev protobuf-compiler
```

### 2. Install Rust toolchain using [Rustup](https://rustup.rs):

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

For pre-commit hooks, the following commands are required:

```shell
rustup toolchain install nightly
cargo install typos-cli
```

For MacOS development make sure the right targets are installed:

```shell
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### 3. Setup Python and Node using [`mise`](https://mise.jdx.dev)

Add mise integrations to your shell:

For zsh:

```shell
echo 'eval "$(mise activate zsh)"' >> "${ZDOTDIR-$HOME}/.zshrc"
```

For bash:

```shell
echo 'eval "$(mise activate bash)"' >> ~/.bashrc
```

For fish:

```shell
echo 'mise activate fish | source' >> ~/.config/fish/config.fish
```

Install the Python and Node toolchains using:

```shell
mise trust
mise install
```

### 4. Setup precommit hooks

Run `pnpm` in root directory to add pre-commit hooks:

```shell
pnpm install --ignore-scripts
```

</div>
</details>

### 3. Start Local Development

To compile and view changes made to `q chat`:

```shell
cargo run --bin q_cli
```

> If you are working on other q commands, just append `-- <command name>`. For example, to run `q login`, you can run `cargo run --bin q_cli -- login`

To run tests for the Q CLI crate:

```shell
cargo test -p q_cli
```

To format Rust files:

```shell
cargo +nightly fmt
```

To run clippy:

```shell
cargo clippy --locked --workspace --color always -- -D warnings
```

### 💡 Quick Tip for Onboarding

Use Q CLI to help you onboard Q CLI!

Start a `q chat` session:

```shell
q chat
```

Once inside `q chat`, you can supply project context by adding the [`codebase-summary.md`](codebase-summary.md) file:

```shell
/context add codebase-summary.md
```

This enables Q to answer onboarding questions like:

- “What does this crate do?”

- “Where is X implemented?”

- “How do these components interact?”

Great for speeding up your ramp-up and navigating the repo more effectively.

## 🏗️ Project Layout

Several projects live here:

- [`autocomplete`](packages/autocomplete/) - The autocomplete react app
- [`dashboard`](packages/dashboard-app/) - The dashboard react app
- [`figterm`](crates/figterm/) - figterm, our headless terminal/pseudoterminal that
  intercepts the user’s terminal edit buffer.
- [`q_cli`](crates/q_cli/) - the `q` CLI, allows users to interface with Amazon Q Developer from
  the command line
- [`fig_desktop`](crates/fig_desktop/) - the Rust desktop app, uses
  [`tao`](https://docs.rs/tao/latest/tao/)/[`wry`](https://docs.rs/wry/latest/wry/)
  for windowing/webviews
- [`fig_input_method`](crates/fig_input_method/) - The input method used to get cursor
  position on macOS
- [`vscode`](extensions/vscode/) - Contains the VSCode plugin needed
  for the Amazon Q Developer for command line to work in VSCode
- [`jetbrains`](extensions/jetbrains/) - Contains the VSCode plugin
  needed for the Amazon Q Developer for command line to work in Jetbrains IDEs

Other folder to be aware of

- [`build-scripts/`](build-scripts/) - Contains all python scripts to build,
  sign, and test the project on macOS and Linux
- [`crates/`](crates/) - Contains all internal rust crates
- [`packages/`](packages/) - Contains all internal npm packages
- [`proto/`](proto/) -
  [protocol buffer](https://developers.google.com/protocol-buffers/) message
  specification for inter-process communication
- [`tests/`](tests/) - Contain integration tests for the projects

Below is a high level architecture of how the different components of the app and
their IPC:

![architecture](docs/assets/architecture.svg)

## 🛡️ Security

See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## 📜 Licensing

This repo is dual licensed under MIT and Apache 2.0 licenses.

“Amazon Web Services” and all related marks, including logos, graphic designs, and service names, are trademarks or trade dress of AWS in the U.S. and other countries. AWS’s trademarks and trade dress may not be used in connection with any product or service that is not AWS’s, in any manner that is likely to cause confusion among customers, or in any manner that disparages or discredits AWS.
