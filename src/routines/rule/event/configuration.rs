mod kind;

use chrono::{DateTime, Utc};
pub use kind::Kind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    kind: Option<Kind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(rename = "timeStart", skip_serializing_if = "Option::is_none")]
    time_start: Option<String>,
    #[serde(rename = "timeEnd", skip_serializing_if = "Option::is_none")]
    time_end: Option<String>,
    #[serde(rename = "actionType", skip_serializing_if = "Option::is_none")]
    action: Option<String>,
    #[serde(rename = "endActionDelay", skip_serializing_if = "Option::is_none")]
    end_action_delay: Option<u32>,
}

impl Configuration {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        kind: Option<Kind>,
        time: Option<String>,
        start_time: Option<String>,
        duration: Option<String>,
        offset: Option<u32>,
        time_start: Option<String>,
        time_end: Option<String>,
        action: Option<String>,
        end_action_delay: Option<u32>,
    ) -> Self {
        Self {
            kind,
            time,
            start_time,
            duration,
            offset,
            time_start,
            time_end,
            action,
            end_action_delay,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> Option<Kind> {
        self.kind
    }

    #[must_use]
    pub fn time(&self) -> Option<&str> {
        self.time.as_deref()
    }

    #[must_use]
    pub fn start_time(&self) -> Option<&str> {
        self.start_time.as_deref()
    }

    #[must_use]
    pub fn duration(&self) -> Option<&str> {
        self.duration.as_deref()
    }

    #[must_use]
    pub const fn offset(&self) -> Option<u32> {
        self.offset
    }

    #[must_use]
    pub fn time_start(&self) -> Option<&str> {
        self.time_start.as_deref()
    }

    #[must_use]
    pub fn time_end(&self) -> Option<&str> {
        self.time_end.as_deref()
    }

    #[must_use]
    pub fn action(&self) -> Option<&str> {
        self.action.as_deref()
    }

    #[must_use]
    pub const fn end_action_delay(&self) -> Option<u32> {
        self.end_action_delay
    }

    #[must_use]
    pub const fn with_kind(mut self, kind: Kind) -> Self {
        self.kind = Some(kind);
        self
    }

    #[must_use]
    pub fn with_time(mut self, time: DateTime<Utc>) -> Self {
        self.time = Some(time.format("%H:%M").to_string());
        self
    }
}
