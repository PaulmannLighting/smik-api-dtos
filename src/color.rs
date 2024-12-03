use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Ord, PartialOrd, Deserialize, Serialize)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    #[must_use]
    pub const fn red(&self) -> u8 {
        self.red
    }

    #[must_use]
    pub const fn green(&self) -> u8 {
        self.green
    }

    #[must_use]
    pub const fn blue(&self) -> u8 {
        self.blue
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Deserialize, Serialize)]
pub enum Mode {
    #[serde(rename = "RGB")]
    Rgb,
    #[serde(rename = "CT")]
    Ct,
}
