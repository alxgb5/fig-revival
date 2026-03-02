# 🚀 Fig Local Revival - Quick Start Guide

A **100% local, offline, privacy-first** terminal autocomplete tool.

## ⚡ One-Command Installation

```bash
git clone https://github.com/alxgb5/fig-revival.git
cd fig-revival
./install-simple.sh
```

That's it! The script will:

- ✅ Build the Fig CLI binary (takes 2-5 minutes)
- ✅ Install to `/usr/local/bin/fig`
- ✅ Configure shell integration (zsh/bash)
- ✅ Create backups of your shell config files

## 🎯 What You Get

### Current Features (CLI-Only Mode)

- ✅ **Command Completions**: git, npm, cd, docker, kubectl, etc.
- ✅ **Settings Management**: Customize appearance and behavior
- ✅ **Integration Checks**: `fig doctor` to verify setup
- ✅ **Issue Reporting**: `fig issue` to report bugs on GitHub

### Coming Soon

- 🚧 **figterm PTY Wrapper**: Advanced terminal integration
- 🚧 **Desktop App**: GUI for settings and diagnostics

> **Note**: This is a CLI-only release. The advanced PTY wrapper (`figterm`) is being rebuilt without AWS dependencies. Basic autocomplete works great!

## 📚 Usage

After installation, restart your shell:

```bash
exec zsh -l  # or: exec bash -l
```

Try autocomplete:

```bash
git <TAB>       # See git commands
npm run <TAB>   # See npm scripts
cd <TAB>        # Directory completions
```

Check your installation:

```bash
fig doctor      # Verify setup
fig --help      # See all commands
fig settings    # Customize Fig
```

## 🔧 Troubleshooting

### "fig: command not found"

Restart your shell: `exec zsh -l`

### "Integration check failed"

Run: `fig doctor` and follow the suggestions

### Autocomplete not working

1. Check: `echo $FIG_TERM` (should show version)
2. Verify integrations in `~/.zshrc` or `~/.bashrc`
3. Restart shell: `exec zsh -l`

## 🗑️ Uninstallation

```bash
./uninstall.sh
```

Or manually:

```bash
sudo rm /usr/local/bin/fig
# Then remove integration lines from ~/.zshrc or ~/.bashrc
```

## 🌐 Links

- **Full README**: [README.md](README.md)
- **Repository**: https://github.com/alxgb5/fig-revival
- **Report Issues**: `fig issue <description>`

## 🔒 Privacy

**100% local. Zero telemetry. No cloud connections.**

All suggestions are generated locally using the `fig_local_provider` crate. No AWS, no tracking, no data collection.

---

Made with ❤️ by removing all the AWS bloat from Amazon Q Developer CLI
