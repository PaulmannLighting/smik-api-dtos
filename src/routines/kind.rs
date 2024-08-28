use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    #[serde(rename = "timeBased")]
    TimeBased,
    #[serde(rename = "timerCountdown")]
    Countdown,
    #[serde(rename = "timePeriod")]
    TimePeriod,
}
