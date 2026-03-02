#!/usr/bin/env bash
# Uninstall script for Fig Local Revival

echo "🗑️  Fig Local Revival - Uninstallation"
echo ""

# Remove binary
if [ -f "/usr/local/bin/fig" ]; then
    echo "Removing /usr/local/bin/fig..."
    sudo rm -f /usr/local/bin/fig
    echo "✅ Binary removed"
else
    echo "ℹ️  Binary not found at /usr/local/bin/fig"
fi

echo ""
echo "📝 Shell integration cleanup:"
echo ""
echo "To manually remove shell integration, edit these files:"
echo "  ~/.zshrc or ~/.bashrc"
echo "  ~/.zprofile or ~/.profile"
echo ""
echo "Remove lines containing:"
echo "  'Fig Local Revival - Terminal Autocomplete'"
echo "  'eval \"\$(fig init ...)'\"
echo ""
echo "Or restore from backup files (*.backup.*) if available"
echo ""
echo "✅ Uninstallation complete"
