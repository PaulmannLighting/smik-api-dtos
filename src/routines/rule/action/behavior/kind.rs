use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    #[serde(rename = "flash")]
    Flash,
    #[serde(rename = "doNothing")]
    DoNothing,
    #[serde(rename = "previousState")]
    PreviousState,
    #[serde(rename = "state")]
    State,
    #[serde(rename = "scene")]
    Scene,
}
