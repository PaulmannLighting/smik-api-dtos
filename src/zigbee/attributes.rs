use serde::{Deserialize, Serialize};

use crate::color::{Mode, Rgb};

/// Zigbee attributes.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Attributes {
    #[serde(rename = "deviceType")]
    typ: Option<String>,
    min: u16,
    max: u16,
    on: bool,
    brightness: Option<u8>,
    color: Option<Rgb>,
    #[serde(rename = "colorTemperature")]
    color_temperature: Option<u16>,
    #[serde(rename = "colorMode")]
    color_mode: Mode,
    #[serde(rename = "manufacturerName")]
    manufacturer: String,
    #[serde(rename = "modelIdentifier")]
    model: String,
    #[serde(rename = "firmwareVersion")]
    firmware_version: String,
    #[serde(rename = "serialNo")]
    serial_number: String,
}

impl Attributes {
    #[must_use]
    pub fn typ(&self) -> Option<&str> {
        self.typ.as_deref()
    }

    #[must_use]
    pub const fn min(&self) -> u16 {
        self.min
    }

    #[must_use]
    pub const fn max(&self) -> u16 {
        self.max
    }

    #[must_use]
    pub const fn on(&self) -> bool {
        self.on
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
    pub const fn color_mode(&self) -> Mode {
        self.color_mode
    }

    #[must_use]
    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    #[must_use]
    pub fn firmware_version(&self) -> &str {
        &self.firmware_version
    }

    #[must_use]
    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }
}
