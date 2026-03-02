#!/bin/bash
# Fig Local Revival - Quick Install Script

set -e

echo "🎉 Fig Local Revival - Installation"
echo ""

# Check prerequisites
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust toolchain not found. Install from https://rustup.rs/"
    exit 1
fi

if ! command -v git &> /dev/null; then
    echo "❌ Git not found. Please install git first."
    exit 1
fi

# Clone repo if not already in it
if [ ! -f "Cargo.toml" ]; then
    echo "📦 Cloning fig-local-revival..."
    git clone https://github.com/YOUR_USERNAME/fig-local-revival.git
    cd fig-local-revival
fi

# Build release binaries
echo "🔨 Building release binaries (this may take a few minutes)..."
cargo build --release --bin fig_cli 2>&1 | grep -E "(Compiling|Finished|error)" || true

if [ ! -f "target/release/fig_cli" ]; then
    echo "❌ Build failed. Check errors above."
    exit 1
fi

# Install binaries
echo "📦 Installing binaries to /usr/local/bin..."
sudo cp target/release/fig_cli /usr/local/bin/fig
sudo cp target/release/figterm /usr/local/bin/ 2>/dev/null || echo "⚠️  figterm not built yet"
sudo cp target/release/fig_desktop /usr/local/bin/ 2>/dev/null || echo "⚠️  fig_desktop not built yet"

# Detect shell
SHELL_TYPE=$(basename "$SHELL")

# Install shell integration
echo "🔗 Installing $SHELL_TYPE integration..."
case "$SHELL_TYPE" in
    zsh)
        /usr/local/bin/fig integrations install zsh || echo "⚠️  Integration install failed"
        ;;
    bash)
        /usr/local/bin/fig integrations install bash || echo "⚠️  Integration install failed"
        ;;
    fish)
        /usr/local/bin/fig integrations install fish || echo "⚠️  Integration install failed"
        ;;
    *)
        echo "⚠️  Unsupported shell: $SHELL_TYPE"
        ;;
esac

echo ""
echo "✅ Installation complete!"
echo ""
echo "Next steps:"
echo "  1. Restart your terminal: exec $SHELL_TYPE -l"
echo "  2. Try autocomplete: cd [press Tab]"
echo "  3. Check status: fig doctor"
echo ""
