use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    #[serde(rename = "timeEvent")]
    Time,
    #[serde(rename = "timerEvent")]
    Timer,
    #[serde(rename = "astroTimeEvent")]
    Astro,
}
