use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Icon {
    #[serde(rename = "time_period")]
    TimePeriod,
    #[serde(rename = "timer_countdown")]
    CountdownTimer,
    #[serde(rename = "time_based")]
    Time,
}
