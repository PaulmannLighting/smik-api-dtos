use serde::{Deserialize, Serialize};

pub use crate::color::Rgb;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct State {
    #[serde(skip_serializing_if = "Option::is_none")]
    on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reachable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brightness: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Rgb>,
    #[serde(rename = "colorTemperature", skip_serializing_if = "Option::is_none")]
    color_temperature: Option<u16>,
}

impl State {
    #[must_use]
    pub const fn new(
        on: Option<bool>,
        reachable: Option<bool>,
        brightness: Option<u8>,
        color: Option<Rgb>,
        color_temperature: Option<u16>,
    ) -> Self {
        Self {
            on,
            reachable,
            brightness,
            color,
            color_temperature,
        }
    }

    #[must_use]
    pub const fn on(&self) -> Option<bool> {
        self.on
    }

    #[must_use]
    pub const fn reachable(&self) -> Option<bool> {
        self.reachable
    }

    #[must_use]
    pub const fn brightness(&self) -> Option<u8> {
        self.brightness
    }

    #[must_use]
    pub const fn color(&self) -> Option<&Rgb> {
        self.color.as_ref()
    }

    #[must_use]
    pub const fn color_temperature(&self) -> Option<u16> {
        self.color_temperature
    }

    #[must_use]
    pub const fn with_on(mut self, on: bool) -> Self {
        self.on = Some(on);
        self
    }

    #[must_use]
    pub const fn with_reachable(mut self, reachable: bool) -> Self {
        self.reachable = Some(reachable);
        self
    }

    #[must_use]
    pub const fn with_brightness(mut self, brightness: u8) -> Self {
        self.brightness = Some(brightness);
        self
    }

    #[must_use]
    pub const fn with_color(mut self, color: Rgb) -> Self {
        self.color = Some(color);
        self
    }

    #[must_use]
    pub const fn with_color_temperature(mut self, color_temperature: u16) -> Self {
        self.color_temperature = Some(color_temperature);
        self
    }
}
