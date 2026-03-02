#!/usr/bin/env bash
set -e

echo "🎉 Fig Local Revival - Complete Installation"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found${NC}"
    echo "Please install Rust from: https://rustup.rs/"
    exit 1
fi

echo -e "${BLUE}📦 Step 1/4: Building Fig CLI binary...${NC}"
echo "This may take a few minutes on first build..."
cargo build --release --locked -p fig_cli -q

if [ ! -f "target/release/fig_cli" ]; then
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Fig CLI built successfully ($(du -h target/release/fig_cli | cut -f1))${NC}"
echo ""

echo -e "${BLUE}📦 Step 2/4: Installing binary to /usr/local/bin...${NC}"
sudo cp target/release/fig_cli /usr/local/bin/fig
sudo chmod +x /usr/local/bin/fig

echo -e "${GREEN}✅ Binary installed${NC}"
echo ""

# Detect shell
SHELL_TYPE=$(basename "$SHELL")
SHELL_RC=""
PROFILE_FILE=""

case "$SHELL_TYPE" in
    zsh)
        SHELL_RC="$HOME/.zshrc"
        PROFILE_FILE="$HOME/.zprofile"
        ;;
    bash)
        SHELL_RC="$HOME/.bashrc"
        PROFILE_FILE="$HOME/.profile"
        ;;
    fish)
        SHELL_RC="$HOME/.config/fish/config.fish"
        echo -e "${YELLOW}⚠️  Fish shell support is experimental${NC}"
        ;;
    *)
        echo -e "${YELLOW}⚠️  Unsupported shell: $SHELL_TYPE${NC}"
        echo "Only zsh and bash are fully supported"
        SHELL_RC=""
        ;;
esac

if [ -n "$SHELL_RC" ] && [ "$SHELL_TYPE" != "fish" ]; then
    echo -e "${BLUE}📦 Step 3/4: Setting up shell integration ($SHELL_TYPE)...${NC}"
    
    # Backup existing files
    if [ -f "$SHELL_RC" ]; then
        cp "$SHELL_RC" "$SHELL_RC.backup.$(date +%Y%m%d_%H%M%S)" 2>/dev/null || true
        echo "  → Backed up $SHELL_RC"
    fi
    
    if [ -n "$PROFILE_FILE" ] && [ -f "$PROFILE_FILE" ]; then
        cp "$PROFILE_FILE" "$PROFILE_FILE.backup.$(date +%Y%m%d_%H%M%S)" 2>/dev/null || true
        echo "  → Backed up $PROFILE_FILE"
    fi
    
    # Add pre integration to profile
    if [ -n "$PROFILE_FILE" ]; then
        touch "$PROFILE_FILE" 2>/dev/null || true
        if ! grep -q "fig init $SHELL_TYPE pre" "$PROFILE_FILE" 2>/dev/null; then
            echo "" >> "$PROFILE_FILE"
            echo "# Fig Local Revival - Terminal Autocomplete (Pre)" >> "$PROFILE_FILE"
            echo "eval \"\$(fig init $SHELL_TYPE pre)\"" >> "$PROFILE_FILE"
            echo "  → Added pre integration to $PROFILE_FILE"
        else
            echo "  → Pre integration already in $PROFILE_FILE"
        fi
    fi
    
    # Add post integration to RC file
    if ! grep -q "fig init $SHELL_TYPE post" "$SHELL_RC" 2>/dev/null; then
        echo "" >> "$SHELL_RC"
        echo "# Fig Local Revival - Terminal Autocomplete (Post)" >> "$SHELL_RC"
        echo "eval \"\$(fig init $SHELL_TYPE post)\"" >> "$SHELL_RC"
        echo "  → Added post integration to $SHELL_RC"
    else
        echo "  → Post integration already in $SHELL_RC"
    fi
    
    echo -e "${GREEN}✅ Shell integration configured${NC}"
else
    echo -e "${YELLOW}⚠️  Skipping shell integration (unsupported shell)${NC}"
fi

echo ""
echo -e "${BLUE}📦 Step 4/4: Verifying installation...${NC}"

# Test binary
if fig --version &> /dev/null; then
    FIG_VERSION=$(fig --version 2>&1 | head -1)
    echo -e "${GREEN}✅ Fig CLI is working: $FIG_VERSION${NC}"
else
    echo -e "${RED}❌ Fig binary test failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Installation complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BLUE}📖 Next steps:${NC}"
echo ""
echo "  1. Restart your shell:"
echo -e "     ${YELLOW}exec $SHELL_TYPE -l${NC}"
echo ""
echo "  2. Try the autocomplete:"
echo -e "     ${YELLOW}cd [press Tab]${NC}"
echo -e "     ${YELLOW}git [press Tab]${NC}"
echo -e "     ${YELLOW}npm [press Tab]${NC}"
echo ""
echo "  3. Check installation:"
echo -e "     ${YELLOW}fig doctor${NC}"
echo ""
echo "  4. Get help:"
echo -e "     ${YELLOW}fig --help${NC}"
echo ""
echo -e "${BLUE}🌐 Documentation:${NC} https://github.com/alxgb5/fig-revival"
echo -e "${BLUE}🐛 Report issues:${NC} fig issue <description>"
echo ""

# Note about figterm
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}ℹ️  Note: This is a CLI-only installation${NC}"
echo ""
echo "Fig Local Revival currently runs in CLI-only mode."
echo "Advanced features (figterm PTY wrapper) are being rebuilt."
echo ""
echo "Current features:"
echo "  ✅ Shell command completions (git, npm, cd, etc.)"
echo "  ✅ Settings management"
echo "  ✅ Integration checks"
echo "  ✅ Issue reporting"
echo ""
echo "This is sufficient for basic autocomplete functionality!"
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
