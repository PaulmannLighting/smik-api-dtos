use crate::color::Rgb;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Action {
    on: bool,
    brightness: u8,
    color: Rgb,
    color_temperature: u16,
}

impl Action {
    #[must_use]
    pub const fn new(on: bool, brightness: u8, color: Rgb, color_temperature: u16) -> Self {
        Self {
            on,
            brightness,
            color,
            color_temperature,
        }
    }

    #[must_use]
    pub const fn on(&self) -> bool {
        self.on
    }

    #[must_use]
    pub const fn brightness(&self) -> u8 {
        self.brightness
    }

    #[must_use]
    pub const fn color(&self) -> &Rgb {
        &self.color
    }

    #[must_use]
    pub const fn color_temperature(&self) -> u16 {
        self.color_temperature
    }
}
