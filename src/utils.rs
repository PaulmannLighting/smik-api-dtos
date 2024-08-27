use chrono::{DateTime, NaiveDateTime, Utc};
use log::error;

pub const DATE_TIME_FORMAT: &str = "%Y-%m-%d - %H:%M";

pub fn parse_date_time(date_time: impl AsRef<str>) -> Option<DateTime<Utc>> {
    NaiveDateTime::parse_from_str(date_time.as_ref(), DATE_TIME_FORMAT)
        .inspect_err(|error| error!("{error}"))
        .ok()
        .map(|naive| DateTime::from_naive_utc_and_offset(naive, Utc))
}
