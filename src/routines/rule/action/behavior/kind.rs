use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    #[default]
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
