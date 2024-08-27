use chrono::Duration;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on: Option<bool>,
    #[serde(rename = "fadeDuration", skip_serializing_if = "Option::is_none")]
    fade_duration: Option<i64>,
    #[serde(rename = "startColor", skip_serializing_if = "Option::is_none")]
    start_color: Option<String>,
    #[serde(rename = "endColor", skip_serializing_if = "Option::is_none")]
    end_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<u32>,
    #[serde(rename = "startBrightness", skip_serializing_if = "Option::is_none")]
    start_brightness: Option<u32>,
    #[serde(rename = "endBrightness", skip_serializing_if = "Option::is_none")]
    end_brightness: Option<u32>,
    #[serde(rename = "startTemperature", skip_serializing_if = "Option::is_none")]
    start_temperature: Option<u32>,
    #[serde(rename = "endTemperature", skip_serializing_if = "Option::is_none")]
    end_temperature: Option<u32>,
}

impl Configuration {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        id: Option<u32>,
        on: Option<bool>,
        fade_duration: Option<i64>,
        start_color: Option<String>,
        end_color: Option<String>,
        duration: Option<u32>,
        start: Option<u32>,
        end: Option<u32>,
        start_brightness: Option<u32>,
        end_brightness: Option<u32>,
        start_temperature: Option<u32>,
        end_temperature: Option<u32>,
    ) -> Self {
        Self {
            id,
            on,
            fade_duration,
            start_color,
            end_color,
            duration,
            start,
            end,
            start_brightness,
            end_brightness,
            start_temperature,
            end_temperature,
        }
    }

    #[must_use]
    pub const fn id(&self) -> Option<u32> {
        self.id
    }

    #[must_use]
    pub const fn on(&self) -> Option<bool> {
        self.on
    }

    #[must_use]
    pub const fn fade_duration(&self) -> Option<i64> {
        self.fade_duration
    }

    #[must_use]
    pub fn start_color(&self) -> Option<&str> {
        self.start_color.as_deref()
    }

    #[must_use]
    pub fn end_color(&self) -> Option<&str> {
        self.end_color.as_deref()
    }

    #[must_use]
    pub const fn duration(&self) -> Option<u32> {
        self.duration
    }

    #[must_use]
    pub const fn start(&self) -> Option<u32> {
        self.start
    }

    #[must_use]
    pub const fn end(&self) -> Option<u32> {
        self.end
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
    pub const fn start_temperature(&self) -> Option<u32> {
        self.start_temperature
    }

    #[must_use]
    pub const fn end_temperature(&self) -> Option<u32> {
        self.end_temperature
    }

    #[must_use]
    pub const fn with_id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }

    #[must_use]
    pub const fn with_on(mut self, on: bool) -> Self {
        self.on = Some(on);
        self
    }

    #[must_use]
    pub fn with_fade_duration(mut self, duration: Duration) -> Self {
        self.fade_duration = Some(duration.num_minutes());
        self
    }

    #[must_use]
    pub fn with_duration(mut self, duration: Duration) -> Self {
        match duration.num_minutes().try_into() {
            Ok(minutes) => self.duration = Some(minutes),
            Err(error) => error!("{error}"),
        };
        self
    }
}
