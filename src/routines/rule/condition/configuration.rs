mod kind;

use crate::Weekdays;
use chrono::Weekday;
pub use kind::Kind;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Configuration {
    days: String,
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
    fade_duration: Option<u32>,
    #[serde(rename = "startBrightness", skip_serializing_if = "Option::is_none")]
    start_brightness: Option<u32>,
    #[serde(rename = "endBrightness", skip_serializing_if = "Option::is_none")]
    end_brightness: Option<u32>,
}

impl Configuration {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        days: String,
        near_location: Option<bool>,
        kind: Option<Kind>,
        offset: Option<u32>,
        value: Option<u32>,
        unit: Option<String>,
        start_temperature: Option<u32>,
        end_temperature: Option<u32>,
        fade_duration: Option<u32>,
        start_brightness: Option<u32>,
        end_brightness: Option<u32>,
    ) -> Self {
        Self {
            days,
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
    pub fn weekdays(&self) -> HashSet<Weekday> {
        Weekdays::from_str(&self.days)
            .map(Weekdays::into)
            .unwrap_or_default()
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
    pub const fn fade_duration(&self) -> Option<u32> {
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
    pub fn with_weekdays(self, weekdays: &[Weekday]) -> Self {
        self.with_weekday_mask(weekdays.into())
    }

    fn with_weekday_mask(self, mask: Weekdays) -> Self {
        self.with_days(mask.to_string())
    }

    fn with_days(mut self, days: String) -> Self {
        self.days = days;
        self
    }
}

impl From<&[Weekday]> for Configuration {
    fn from(weekdays: &[Weekday]) -> Self {
        Self::default().with_weekdays(weekdays)
    }
}
