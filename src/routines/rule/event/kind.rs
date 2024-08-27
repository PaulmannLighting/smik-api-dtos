use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    #[default]
    #[serde(rename = "timeEvent")]
    Time,
    #[serde(rename = "timerEvent")]
    Timer,
    #[serde(rename = "astroTimeEvent")]
    Astro,
}
