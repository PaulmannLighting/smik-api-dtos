use serde::{Deserialize, Serialize};

pub use crate::color::Rgb;
use crate::Percent;

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
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
    color_temperature: Option<u8>,
}

impl State {
    #[must_use]
    pub fn new(
        on: Option<bool>,
        reachable: Option<bool>,
        brightness: Option<Percent>,
        color: Option<Rgb>,
        color_temperature: Option<Percent>,
    ) -> Self {
        Self {
            on,
            reachable,
            brightness: brightness.map(Percent::into_inner),
            color,
            color_temperature: color_temperature.map(Percent::into_inner),
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

    /// Returns the brightness as a percentage.
    ///
    /// # Errors
    ///
    /// Returns `Err(None)` if the brightness is not set
    /// or `Err(Some(u8))` if the brightness is not a valid percentage.
    pub fn brightness(&self) -> Result<Percent, Option<u8>> {
        self.brightness
            .map_or_else(|| Err(None), |v| Percent::try_from(v).map_err(Some))
    }

    #[must_use]
    pub const fn color(&self) -> Option<&Rgb> {
        self.color.as_ref()
    }

    /// Returns the color temperature as a percentage.
    ///
    /// # Errors
    ///
    /// Returns `Err(None)` if the color temperature is not set
    /// or `Err(Some(u8))` if the color temperature is not a valid percentage.
    pub fn color_temperature(&self) -> Result<Percent, Option<u8>> {
        self.color_temperature
            .map_or_else(|| Err(None), |v| Percent::try_from(v).map_err(Some))
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
    pub const fn with_brightness(mut self, brightness: Percent) -> Self {
        self.brightness = Some(brightness.into_inner());
        self
    }

    #[must_use]
    pub const fn with_color(mut self, color: Rgb) -> Self {
        self.color = Some(color);
        self
    }

    #[must_use]
    pub const fn with_color_temperature(mut self, color_temperature: Percent) -> Self {
        self.color_temperature = Some(color_temperature.into_inner());
        self
    }
}
