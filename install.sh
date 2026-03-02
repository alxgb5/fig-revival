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
cargo build --release --bin fig_cli -q 2>&1 | grep -E "(error|warning:)" || true

if [ ! -f "target/release/fig_cli" ]; then
    echo "❌ Build failed. Check errors above."
    exit 1
fi

# Install binaries
echo "📦 Installing binary to /usr/local/bin..."
sudo cp target/release/fig_cli /usr/local/bin/fig

# Detect shell
SHELL_TYPE=$(basename "$SHELL")
SHELL_RC=""

case "$SHELL_TYPE" in
    zsh)
        SHELL_RC="$HOME/.zshrc"
        ;;
    bash)
        SHELL_RC="$HOME/.bashrc"
        ;;
    fish)
        SHELL_RC="$HOME/.config/fish/config.fish"
        echo "⚠️  Fish shell integration needs manual setup. See README.md"
        ;;
    *)
        echo "⚠️  Unsupported shell: $SHELL_TYPE. Only zsh and bash are supported."
        SHELL_RC=""
        ;;
esac

# Add shell integration to rc file
if [ -n "$SHELL_RC" ] && [ "$SHELL_TYPE" != "fish" ]; then
    echo "🔗 Adding $SHELL_TYPE integration to $SHELL_RC..."
    
    # For bash, also handle .profile for pre integration
    PROFILE_FILE=""
    if [ "$SHELL_TYPE" = "bash" ]; then
        PROFILE_FILE="$HOME/.profile"
    elif [ "$SHELL_TYPE" = "zsh" ]; then
        PROFILE_FILE="$HOME/.zprofile"
    fi
    
    # Check if already installed
    if grep -q "fig init $SHELL_TYPE" "$SHELL_RC" 2>/dev/null; then
        echo "✓ Integration already present in $SHELL_RC"
    else
        # Backup original rc file
        cp "$SHELL_RC" "$SHELL_RC.backup.$(date +%Y%m%d_%H%M%S)" 2>/dev/null || true
        
        # Add fig post integration to RC file
        echo "" >> "$SHELL_RC"
        echo "# Fig Local Revival - Terminal Autocomplete (Post)" >> "$SHELL_RC"
        echo "eval \"\$(fig init $SHELL_TYPE post)\"" >> "$SHELL_RC"
        echo "✓ Post integration added to $SHELL_RC"
    fi
    
    # Add pre integration to profile file if it exists
    if [ -n "$PROFILE_FILE" ]; then
        if grep -q "fig init $SHELL_TYPE pre" "$PROFILE_FILE" 2>/dev/null; then
            echo "✓ Pre integration already present in $PROFILE_FILE"
        else
            # Create profile file if it doesn't exist
            touch "$PROFILE_FILE" 2>/dev/null || true
            
            # Backup if file exists and has content
            if [ -s "$PROFILE_FILE" ]; then
                cp "$PROFILE_FILE" "$PROFILE_FILE.backup.$(date +%Y%m%d_%H%M%S)" 2>/dev/null || true
            fi
            
            # Add fig pre integration
            echo "" >> "$PROFILE_FILE"
            echo "# Fig Local Revival - Terminal Autocomplete (Pre)" >> "$PROFILE_FILE"
            echo "eval \"\$(fig init $SHELL_TYPE pre)\"" >> "$PROFILE_FILE"
            echo "✓ Pre integration added to $PROFILE_FILE"
        fi
    fi
fi

echo ""
echo "✅ Installation complete!"
echo ""
echo "Next steps:"
echo "  1. Restart your terminal: exec $SHELL_TYPE -l"
echo "  2. Try autocomplete: cd [press Tab]"
echo "  3. Check status: fig doctor"
echo ""
