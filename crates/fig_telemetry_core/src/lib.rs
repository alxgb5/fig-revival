//! Fig Telemetry Core - Local stub (AWS removed)
//!
//! This is a minimal stub that preserves the public API but removes all AWS dependencies.

use std::any::Any;
use std::sync::OnceLock;
use std::time::{Duration, SystemTime};
use strum::{Display, EnumString};

type GlobalTelemetryEmitter = dyn TelemetryEmitter + Send + Sync + 'static;

static EMITTER: OnceLock<Box<GlobalTelemetryEmitter>> = OnceLock::new();

pub fn init_global_telemetry_emitter<T>(telemetry_emitter: T)
where
    T: TelemetryEmitter + Send + Sync + 'static,
{
    match EMITTER.set(Box::new(telemetry_emitter)) {
        Ok(_) => (),
        Err(_) => panic!("The global telemetry emitter can only be initialized once"),
    }
}

pub async fn send_event(_event: Event) -> Option<()> {
    // No-op in local version
    None
}

#[async_trait::async_trait]
pub trait TelemetryEmitter {
    async fn send(&self, event: Event);
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub created_time: Option<SystemTime>,
    pub credential_start_url: Option<String>,
    #[serde(flatten)]
    pub ty: EventType,
}

impl Event {
    pub fn new(ty: EventType) -> Self {
        Self {
            ty,
            created_time: Some(SystemTime::now()),
            credential_start_url: None,
        }
    }

    pub fn with_credential_start_url(mut self, credential_start_url: String) -> Self {
        self.credential_start_url = Some(credential_start_url);
        self
    }

    // Stub - no AWS metric conversion
    pub fn into_metric_datum(self) -> Option<()> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "name", rename_all = "camelCase")]
pub enum EventType {
    #[serde(rename = "codewhisperer_userLoggedIn")]
    UserLoggedIn {},

    #[serde(rename = "codewhisperer_refreshCredentials")]
    RefreshCredentials {
        request_id: String,
        result: TelemetryResult,
        reason: Option<String>,
        oauth_flow: bool,
    },

    #[serde(rename = "codewhispererterminal_completionInserted")]
    CompletionInserted {
        command: String,
        terminal: Option<String>,
        shell: Option<String>,
    },

    #[serde(rename = "codewhispererterminal_inlineShellActioned")]
    InlineShellCompletionActioned {
        terminal: Option<String>,
        terminal_version: Option<String>,
        shell: Option<String>,
        shell_version: Option<String>,
        suggestion_state: SuggestionState,
        edit_buffer_len: Option<i32>,
        suggested_chars_len: i32,
        latency: Duration,
    },

    #[serde(rename = "codewhispererterminal_translationActioned")]
    TranslationActioned {
        latency: Option<Duration>,
        suggestion_state: SuggestionState,
        terminal: Option<String>,
        terminal_version: Option<String>,
        shell: Option<String>,
        shell_version: Option<String>,
    },

    #[serde(rename = "codewhispererterminal_cliSubcommandExecuted")]
    CliSubcommandExecuted { command: String, result: TelemetryResult },

    #[serde(rename = "codewhispererterminal_doctorCheckFailed")]
    DoctorCheckFailed { failed_check: String },

    #[serde(rename = "codewhispererterminal_dashboardPageViewed")]
    DashboardPageViewed { page: String },

    #[serde(rename = "codewhispererterminal_menuBarActioned")]
    MenuBarActioned { action: String },

    #[serde(rename = "codewhispererterminal_figUserMigrated")]
    FigUserMigrated {},

    #[serde(rename = "amazonq_startChat")]
    StartChat,

    #[serde(rename = "amazonq_endChat")]
    EndChat { result: TelemetryResult },

    #[serde(rename = "codewhispererterminal_addChatMessage")]
    AddChatMessage {
        has_code_snippet: bool,
        has_shell_output: bool,
        has_contextual_information: bool,
        utterance_id: Option<String>,
    },

    #[serde(rename = "codewhispererterminal_migrateOldClientId")]
    MigrateOldClientId,

    #[serde(rename = "codewhispererterminal_toolUseSuggested")]
    ToolUseSuggested,

    #[serde(rename = "codewhispererterminal_mcpServerInit")]
    McpServerInit,

    #[serde(rename = "amazonq_didSelectProfile")]
    DidSelectProfile,

    #[serde(rename = "amazonq_profileState")]
    ProfileState,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, serde::Serialize, serde::Deserialize)]
pub enum SuggestionState {
    Accept,
    Discard,
    Empty,
    Reject,
}

impl SuggestionState {
    pub fn is_accepted(&self) -> bool {
        matches!(self, SuggestionState::Accept)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SuggestionState::Accept => "ACCEPT",
            SuggestionState::Discard => "DISCARD",
            SuggestionState::Empty => "EMPTY",
            SuggestionState::Reject => "REJECT",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, serde::Serialize, serde::Deserialize)]
pub enum TelemetryResult {
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Display, serde::Serialize, serde::Deserialize)]
pub enum QProfileSwitchIntent {
    User,
    Auth,
    Update,
    Reload,
}
