mod kind;

use crate::Weekdays;
use chrono::Weekday;
pub use kind::Kind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(rename = "days")]
    weekdays: Weekdays,
    #[serde(rename = "nearLocation", skip_serializing_if = "Option::is_none")]
    near_location: Option<bool>,
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    kind: Option<Kind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<u32>,
    #[serde(rename = "valueType", skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
    #[serde(rename = "startTemperature", skip_serializing_if = "Option::is_none")]
    start_temperature: Option<u32>,
    #[serde(rename = "endTemperature", skip_serializing_if = "Option::is_none")]
    end_temperature: Option<u32>,
    #[serde(rename = "fadeDuration", skip_serializing_if = "Option::is_none")]
    fade_duration: Option<i64>,
    #[serde(rename = "startBrightness", skip_serializing_if = "Option::is_none")]
    start_brightness: Option<u32>,
    #[serde(rename = "endBrightness", skip_serializing_if = "Option::is_none")]
    end_brightness: Option<u32>,
}

impl Configuration {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        weekdays: Weekdays,
        near_location: Option<bool>,
        kind: Option<Kind>,
        offset: Option<u32>,
        value: Option<u32>,
        unit: Option<String>,
        start_temperature: Option<u32>,
        end_temperature: Option<u32>,
        fade_duration: Option<i64>,
        start_brightness: Option<u32>,
        end_brightness: Option<u32>,
    ) -> Self {
        Self {
            weekdays,
            near_location,
            kind,
            offset,
            value,
            unit,
            start_temperature,
            end_temperature,
            fade_duration,
            start_brightness,
            end_brightness,
        }
    }

    #[must_use]
    pub const fn weekdays(&self) -> Weekdays {
        self.weekdays
    }

    #[must_use]
    pub const fn near_location(&self) -> Option<bool> {
        self.near_location
    }

    #[must_use]
    pub const fn kind(&self) -> Option<Kind> {
        self.kind
    }

    #[must_use]
    pub const fn offset(&self) -> Option<u32> {
        self.offset
    }

    #[must_use]
    pub const fn value(&self) -> Option<u32> {
        self.value
    }

    #[must_use]
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }

    #[must_use]
    pub const fn start_temperature(&self) -> Option<u32> {
        self.start_temperature
    }

    #[must_use]
    pub const fn end_temperature(&self) -> Option<u32> {
        self.end_temperature
    }

    #[must_use]
    pub const fn fade_duration(&self) -> Option<i64> {
        self.fade_duration
    }

    #[must_use]
    pub const fn start_brightness(&self) -> Option<u32> {
        self.start_brightness
    }

    #[must_use]
    pub const fn end_brightness(&self) -> Option<u32> {
        self.end_brightness
    }

    #[must_use]
    pub fn with_weekdays(mut self, weekdays: impl Into<Weekdays>) -> Self {
        self.weekdays = weekdays.into();
        self
    }

    #[must_use]
    pub const fn with_fade_duration(mut self, fade_duration: i64) -> Self {
        self.fade_duration = Some(fade_duration);
        self
    }
}

impl From<&[Weekday]> for Configuration {
    fn from(weekdays: &[Weekday]) -> Self {
        Self::default().with_weekdays(weekdays)
    }
}
