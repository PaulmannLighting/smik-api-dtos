use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    #[serde(rename = "allLights")]
    AllLights,
    #[serde(rename = "area")]
    Area,
}
