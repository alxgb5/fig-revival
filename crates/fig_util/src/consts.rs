pub const APP_BUNDLE_ID: &str = "com.fig.local-revival";
pub const APP_BUNDLE_NAME: &str = "Fig.app";

#[cfg(target_os = "macos")]
pub const APP_PROCESS_NAME: &str = "fig_desktop";
#[cfg(target_os = "linux")]
pub const APP_PROCESS_NAME: &str = "fig-desktop";

#[cfg(windows)]
pub const APP_PROCESS_NAME: &str = "fig_desktop.exe";

/// The name configured under `"package.productName"` in the tauri.conf.json file.
pub const TAURI_PRODUCT_NAME: &str = "fig_desktop";

pub const CLI_BINARY_NAME: &str = "fig";
pub const CLI_BINARY_NAME_MINIMAL: &str = "fig-minimal";
pub const CHAT_BINARY_NAME: &str = "figchat";
pub const PTY_BINARY_NAME: &str = "figterm";

pub const CLI_CRATE_NAME: &str = "fig_cli";

pub const URL_SCHEMA: &str = "fig";

pub const PRODUCT_NAME: &str = "Fig Local Revival";

pub const RUNTIME_DIR_NAME: &str = "figrun";

/// Data directory name used in paths like ~/.local/share/{DATA_DIR_NAME}
#[cfg(unix)]
pub const DATA_DIR_NAME: &str = "fig";
#[cfg(windows)]
pub const DATA_DIR_NAME: &str = "Fig";

/// Backup directory name
pub const BACKUP_DIR_NAME: &str = ".fig.dotfiles.bak";

// These are the old "Amazon Q" and "CodeWhisperer" branding, used for migration
pub const OLD_PRODUCT_NAME: &str = "Amazon Q";
pub const OLD_CLI_BINARY_NAMES: &[&str] = &["q", "cw"];
pub const OLD_PTY_BINARY_NAMES: &[&str] = &["qterm", "cwterm"];

pub const GITHUB_REPO_NAME: &str = "alxgb5/fig-revival";

pub mod url {
    pub const USER_MANUAL: &str = "https://github.com/alxgb5/fig-revival#readme";
    pub const AUTOCOMPLETE_WIKI: &str = "https://github.com/alxgb5/fig-revival#features";
    pub const AUTOCOMPLETE_SSH_WIKI: &str = "https://github.com/alxgb5/fig-revival#ssh-support";
    pub const CHAT_WIKI: &str = "https://github.com/alxgb5/fig-revival#readme";
    pub const TRANSLATE_WIKI: &str = "https://github.com/alxgb5/fig-revival#readme";
    pub const TELEMETRY_WIKI: &str = "https://github.com/alxgb5/fig-revival#privacy";
}

/// Build time env vars
pub mod build {
    /// The target of the current build, e.g. "aarch64-unknown-linux-musl"
    pub const TARGET_TRIPLE: Option<&str> = option_env!("FIG_BUILD_TARGET_TRIPLE");

    /// The variant of the current build
    pub const VARIANT: Option<&str> = option_env!("FIG_BUILD_VARIANT");

    /// A git full sha hash of the current build
    pub const HASH: Option<&str> = option_env!("FIG_BUILD_HASH");

    /// The datetime in rfc3339 format of the current build
    pub const DATETIME: Option<&str> = option_env!("FIG_BUILD_DATETIME");

    /// If `fish` tests should be skipped
    pub const SKIP_FISH_TESTS: bool = option_env!("FIG_BUILD_SKIP_FISH_TESTS").is_some();

    /// If `shellcheck` tests should be skipped
    pub const SKIP_SHELLCHECK_TESTS: bool = option_env!("FIG_BUILD_SKIP_SHELLCHECK_TESTS").is_some();
}

/// macOS specific constants
pub mod macos {
    pub const BUNDLE_CONTENTS_MACOS_PATH: &str = "Contents/MacOS";
    pub const BUNDLE_CONTENTS_RESOURCE_PATH: &str = "Contents/Resources";
    pub const BUNDLE_CONTENTS_HELPERS_PATH: &str = "Contents/Helpers";
    pub const BUNDLE_CONTENTS_INFO_PLIST_PATH: &str = "Contents/Info.plist";
}

pub mod linux {
    pub const DESKTOP_ENTRY_NAME: &str = "fig.desktop";

    /// Name of the deb package.
    pub const PACKAGE_NAME: &str = "fig";

    /// The wm_class used for the application windows.
    pub const DESKTOP_APP_WM_CLASS: &str = "Fig";
}

pub mod env_var {
    macro_rules! define_env_vars {
        ($($(#[$meta:meta])* $ident:ident = $name:expr),*) => {
            $(
                $(#[$meta])*
                pub const $ident: &str = $name;
            )*

            pub const ALL: &[&str] = &[$($ident),*];
        }
    }

    define_env_vars! {
        /// The UUID of the current parent figterm instance
        QTERM_SESSION_ID = "FIGTERM_SESSION_ID",

        /// The current parent socket to connect to
        Q_PARENT = "FIG_PARENT",

        /// Set the [`Q_PARENT`] parent socket to connect to
        Q_SET_PARENT = "FIG_SET_PARENT",

        /// Guard for the [`Q_SET_PARENT`] check
        Q_SET_PARENT_CHECK = "FIG_SET_PARENT_CHECK",

        /// Set if figterm is running, contains the version
        Q_TERM = "FIG_TERM",

        /// Sets the current log level
        Q_LOG_LEVEL = "FIG_LOG_LEVEL",

        /// Overrides the ZDOTDIR environment variable
        Q_ZDOTDIR = "FIG_ZDOTDIR",

        /// Indicates a process was launched by Fig
        PROCESS_LAUNCHED_BY_Q = "PROCESS_LAUNCHED_BY_FIG",

        /// The shell to use in figterm
        Q_SHELL = "FIG_SHELL",

        /// Indicates the user is debugging the shell
        Q_DEBUG_SHELL = "FIG_DEBUG_SHELL",

        /// Indicates the user is using zsh autosuggestions which disables Inline
        Q_USING_ZSH_AUTOSUGGESTIONS = "FIG_USING_ZSH_AUTOSUGGESTIONS",

        /// Overrides the path to the bundle metadata released with certain desktop builds.
        Q_BUNDLE_METADATA_PATH = "FIG_BUNDLE_METADATA_PATH"
    }
}

pub mod system_paths {
    /// System installation paths
    pub const APPLICATIONS_DIR: &str = "/Applications";
    pub const USR_LOCAL_BIN: &str = "/usr/local/bin";
    pub const USR_SHARE: &str = "/usr/share";
    pub const OPT_HOMEBREW_BIN: &str = "/opt/homebrew/bin";
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;
    use time::format_description::well_known::Rfc3339;

    use super::*;

    #[test]
    fn test_build_envs() {
        if let Some(build_variant) = build::VARIANT {
            println!("build_variant: {build_variant}");
            assert!(["full", "minimal"].contains(&&*build_variant.to_ascii_lowercase()));
        }

        if let Some(build_hash) = build::HASH {
            println!("build_hash: {build_hash}");
            assert!(!build_hash.is_empty());
        }

        if let Some(build_datetime) = build::DATETIME {
            println!("build_datetime: {build_datetime}");
            println!("{}", OffsetDateTime::parse(build_datetime, &Rfc3339).unwrap());
        }
    }
}
