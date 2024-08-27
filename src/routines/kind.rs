use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    #[default]
    #[serde(rename = "timeBased")]
    TimeBased,
    #[serde(rename = "timerCountdown")]
    Countdown,
    #[serde(rename = "timePeriod")]
    TimePeriod,
}
