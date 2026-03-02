//! Telemetry event wrapper - AWS removed

use fig_telemetry_core::{Event, EventType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppTelemetryEvent(Event);

impl AppTelemetryEvent {
    pub async fn new_with_start_url(ty: EventType, start_url: Option<String>) -> Self {
        Self(Event {
            ty,
            created_time: Some(std::time::SystemTime::now()),
            credential_start_url: match start_url {
                Some(start_url) => Some(start_url),
                None => None, // No AWS token in local version
            },
        })
    }

    pub async fn new(ty: EventType) -> Self {
        Self::new_with_start_url(ty, None).await
    }

    pub fn into_metric_datum(self) -> Option<()> {
        // AWS telemetry stubbed
        None
    }

    pub fn inner(self) -> Event {
        self.0
    }
}

impl From<Event> for AppTelemetryEvent {
    fn from(event: Event) -> Self {
        Self(event)
    }
}
