use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Icon {
    #[serde(rename = "brightFocus")]
    BrightFocus,
    #[serde(rename = "brightWarm")]
    BrightWarm,
    #[serde(rename = "dimmedWarm")]
    DimmedWarm,
    #[serde(rename = "nightLight")]
    NightLight,
}
