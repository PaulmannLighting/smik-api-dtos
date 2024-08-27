use bitflags::bitflags;
use chrono::Weekday;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::BuildHasher;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Weekdays(u8);

bitflags! {
    impl Weekdays: u8 {
        const Mon = 0b0100_0000;
        const Tue = 0b0010_0000;
        const Wed = 0b0001_0000;
        const Thu = 0b0000_1000;
        const Fri = 0b0000_0100;
        const Sat = 0b0000_0010;
        const Sun = 0b0000_0001;
    }
}

impl Display for Weekdays {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        HashSet::<Weekday>::from(*self).fmt(f)
    }
}

impl From<Weekday> for Weekdays {
    fn from(weekday: Weekday) -> Self {
        match weekday {
            Weekday::Mon => Self::Mon,
            Weekday::Tue => Self::Tue,
            Weekday::Wed => Self::Wed,
            Weekday::Thu => Self::Thu,
            Weekday::Fri => Self::Fri,
            Weekday::Sat => Self::Sat,
            Weekday::Sun => Self::Sun,
        }
    }
}

impl From<HashSet<Weekday>> for Weekdays {
    fn from(weekdays: HashSet<Weekday>) -> Self {
        weekdays
            .into_iter()
            .fold(Self::empty(), |mask, weekday| mask | weekday.into())
    }
}

impl From<&[Weekday]> for Weekdays {
    fn from(weekdays: &[Weekday]) -> Self {
        weekdays
            .iter()
            .fold(Self::empty(), |mask, weekday| mask | (*weekday).into())
    }
}

impl TryFrom<Weekdays> for Weekday {
    type Error = &'static str;

    fn try_from(mask: Weekdays) -> Result<Self, Self::Error> {
        match mask {
            Weekdays::Mon => Ok(Self::Mon),
            Weekdays::Tue => Ok(Self::Tue),
            Weekdays::Wed => Ok(Self::Wed),
            Weekdays::Thu => Ok(Self::Thu),
            Weekdays::Fri => Ok(Self::Fri),
            Weekdays::Sat => Ok(Self::Sat),
            Weekdays::Sun => Ok(Self::Sun),
            _ => Err("Invalid mask"),
        }
    }
}

impl<H> From<Weekdays> for HashSet<Weekday, H>
where
    H: Default + BuildHasher,
{
    fn from(mask: Weekdays) -> Self {
        mask.iter()
            .filter_map(|mask| mask.try_into().ok())
            .collect()
    }
}

impl FromStr for Weekdays {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u8::from_str_radix(s, 2).map(Self)
    }
}

impl Serialize for Weekdays {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:07b}", self.0))
    }
}

impl<'de> Deserialize<'de> for Weekdays {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|string| Self::from_str(&string).map_err(serde::de::Error::custom))
    }
}
