# Fig Local Revival - Git Patches

This document contains the full patches to transform Amazon Q CLI into Fig Local Revival.

## 📊 Summary

**4 commits, ~200k lines purged, 500+ lines added**

### Commit History

1. **🔪 Purge Amazon Q/AWS dependencies**
   - Removed 7 AWS crates (amzn-\*, fig_aws_common)
   - Updated workspace metadata
   - 1682 files deleted, 202k lines removed

2. **🛡️ Add local_provider for offline completions**
   - Created `crates/fig_local_provider/` with local suggestion engine
   - Integrated into `figterm/inline/mod.rs`
   - Removed `fig_api_client` crate
   - 4108 deletions, 326 insertions

3. **📚 Documentation for Fig Local Revival**
   - Rewrote README.md with installation guide
   - Architecture overview
   - Troubleshooting section

4. **🛠️ Stub AWS-dependent crates**
   - Stubbed `fig_telemetry_core` (no-op telemetry)
   - Stubbed `fig_auth` (local-only)
   - Preserved public APIs for compatibility

## 🎯 Applying Patches

### Option 1: Clone This Repo

```bash
git clone https://github.com/YOUR_USERNAME/fig-local-revival.git
cd fig-local-revival
cargo build --release
```

### Option 2: Apply Patches to Upstream

```bash
# Clone original Amazon Q CLI
git clone https://github.com/aws/amazon-q-developer-cli.git fig-local-revival
cd fig-local-revival

# Apply patches (copy commits from this repo)
git remote add fig-local https://github.com/YOUR_USERNAME/fig-local-revival.git
git fetch fig-local
git cherry-pick <commit-1>..<commit-4>
```

### Option 3: Generate Patch Files

```bash
# In fig-local-revival repo
git format-patch HEAD~4

# This creates:
# 0001-Purge-Amazon-Q-AWS-dependencies.patch
# 0002-Add-local_provider-for-offline-completions.patch
# 0003-Documentation-for-Fig-Local-Revival.patch
# 0004-Stub-AWS-dependent-crates.patch

# Apply in target repo:
git am *.patch
```

## 📦 Key Files Changed

### Created

- `crates/fig_local_provider/` - New local completion engine
- `install.sh` - Quick install script

### Modified (AWS removed)

- `Cargo.toml` - Workspace dependencies cleaned
- `crates/figterm/src/inline/mod.rs` - Local provider integration
- `crates/fig_telemetry_core/src/lib.rs` - Stubbed telemetry
- `crates/fig_auth/src/lib.rs` - Stubbed auth
- `crates/q_cli/Cargo.toml` - AWS deps removed
- `crates/figterm/Cargo.toml` - AWS deps removed

### Deleted

- `crates/amzn-codewhisperer-client/`
- `crates/amzn-codewhisperer-streaming-client/`
- `crates/amzn-consolas-client/`
- `crates/amzn-qdeveloper-streaming-client/`
- `crates/amzn-toolkit-telemetry-client/`
- `crates/aws-toolkit-telemetry-definitions/`
- `crates/fig_aws_common/`
- `crates/fig_api_client/`

## 🔍 Patch Details

### PATCH 1: Purge AWS Dependencies

```diff
diff --git a/Cargo.toml b/Cargo.toml
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -1,10 +1,7 @@
 [workspace.package]
-authors = ["Amazon Q CLI Team"]
-version = "1.19.7"
+authors = ["Fig Local Revival Contributors"]
+version = "1.0.0-local"

 [workspace.dependencies]
-amzn-codewhisperer-client = { path = "crates/amzn-codewhisperer-client" }
-amzn-qdeveloper-streaming-client = { path = "crates/amzn-qdeveloper-streaming-client" }
-aws-types = "1.3.0"
+# AWS crates removed
```

### PATCH 2: Local Provider

```rust
// crates/fig_local_provider/src/lib.rs (NEW)
pub struct LocalProvider { }

impl LocalProvider {
    pub fn generate_suggestions(&self, buffer: &str) -> Vec<LocalSuggestion> {
        // cd command
        if buffer.starts_with("cd ") {
            return vec![
                LocalSuggestion { text: "~/".to_string() },
                LocalSuggestion { text: "../".to_string() },
            ];
        }

        // git commands
        if buffer.starts_with("git ") {
            return vec![
                LocalSuggestion { text: "status".to_string() },
                LocalSuggestion { text: "add".to_string() },
                LocalSuggestion { text: "commit".to_string() },
            ];
        }

        vec![]
    }
}
```

### PATCH 3: Figterm Integration

```diff
diff --git a/crates/figterm/src/inline/mod.rs b/crates/figterm/src/inline/mod.rs
--- a/crates/figterm/src/inline/mod.rs
+++ b/crates/figterm/src/inline/mod.rs
@@ -1,5 +1,5 @@
-use fig_api_client::Client;
+use fig_local_provider::LocalProvider;

-let response = client.generate_recommendations(input).await?;
+let suggestions = provider.generate_suggestions(buffer);
```

## 🚧 Known Limitations

### Not Yet Implemented

- Full compilation of all crates (some AWS stubs incomplete)
- `fig_desktop` UI (needs webview assets)
- Shell integration testing
- CI/CD workflows

### Requires Manual Cleanup

- `q_cli` AWS credential flows (stub or remove)
- `fig_telemetry` AWS Cognito (already stubbed)
- `fig_auth` secret_store AWS integration (partially stubbed)

## 🎯 Next Steps

1. **Complete Stubbing**: Finish AWS removal in q_cli user flows
2. **Test Shell Integration**: Verify zsh/bash/fish plugins work
3. **Desktop UI**: Ensure fig_desktop webview loads without AWS
4. **CI/CD**: Add GitHub Actions for builds
5. **Homebrew Formula**: Package for `brew install fig-local`

## 📜 License

MIT OR Apache-2.0 (inherited from upstream)

---

**Questions?** Open an issue at https://github.com/YOUR_USERNAME/fig-local-revival/issues
