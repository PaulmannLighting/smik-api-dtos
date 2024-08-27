use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Kind {
    #[serde(rename = "rgbw")]
    Rgbw,
    #[serde(rename = "rgb")]
    Rgb,
    #[serde(rename = "onOff")]
    OnOff,
    #[serde(rename = "wTunable")]
    TunableWhite,
    #[serde(rename = "wFixed")]
    FixedWhite,
}
