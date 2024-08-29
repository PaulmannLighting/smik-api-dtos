use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
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
