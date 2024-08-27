use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    #[default]
    #[serde(rename = "allLights")]
    AllLights,
    #[serde(rename = "area")]
    Area,
}
