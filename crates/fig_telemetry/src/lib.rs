//! Fig Telemetry - Local stub (AWS removed)
//!
//! This is a minimal stub that preserves the public API but removes all AWS dependencies.

mod event;

pub use event::AppTelemetryEvent;
pub use fig_telemetry_core::{EventType, SuggestionState, TelemetryResult};

use std::any::Any;

// Re-export from core
pub use fig_telemetry_core::{Event, TelemetryEmitter, send_event};

// Re-export init with a direct call
pub use fig_telemetry_core::init_global_telemetry_emitter as init_core;

/// Install method stub
#[derive(Debug, Clone, Copy)]
pub enum InstallMethod {
    Manual,
    Script,
    Homebrew,
    Other,
}

impl std::fmt::Display for InstallMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallMethod::Manual => write!(f, "manual"),
            InstallMethod::Script => write!(f, "script"),
            InstallMethod::Homebrew => write!(f, "homebrew"),
            InstallMethod::Other => write!(f, "other"),
        }
    }
}

/// Get install method (stub)
pub fn get_install_method() -> InstallMethod {
    InstallMethod::Manual
}

/// No-op telemetry emitter for local version
pub struct NoopTelemetryEmitter;

#[async_trait::async_trait]
impl TelemetryEmitter for NoopTelemetryEmitter {
    async fn send(&self, _event: Event) {
        // No-op in local version
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Initialize with no-op emitter
pub fn init() {
    init_core(NoopTelemetryEmitter);
}

/// Dispatch mode stub
#[derive(Debug, Clone, Copy)]
pub enum DispatchMode {
    On,
    Off,
}

/// Set dispatch mode (stub - no-op)
pub fn set_dispatch_mode(_mode: DispatchMode) {
    // No-op in local version
}

/// Initialize telemetry (wrapper for backward compatibility)
pub fn init_global_telemetry_emitter() {
    init();
}

// Telemetry send functions (all no-op in local version)
pub async fn send_doctor_check_failed(_check: &str) {
    // No-op
}

pub async fn send_translation_actioned(_input: &str, _output: &str) {
    // No-op
}

pub async fn send_user_logged_in(_method: &str) {
    // No-op
}

pub async fn send_profile_state(_profile: &str, _state: &str) {
    // No-op
}

pub async fn send_did_select_profile(_profile: &str) {
    // No-op
}

pub async fn send_cli_subcommand_executed(_subcommand: &str, _result: TelemetryResult) {
    // No-op
}

pub async fn finish_telemetry() {
    // No-op
}
