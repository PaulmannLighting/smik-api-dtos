use bitflags::bitflags;
use chrono::Weekday;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
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
        write!(f, "{:07b}", self.0)
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

impl<H> From<Weekdays> for HashSet<Weekday, H>
where
    H: Default + BuildHasher,
{
    fn from(mask: Weekdays) -> Self {
        Weekday::Mon
            .iter_week()
            .filter(|&weekday| mask & weekday.into() != Weekdays::empty())
            .collect()
    }
}

impl FromStr for Weekdays {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u8::from_str_radix(s, 2).map(Self)
    }
}

pub trait IterWeek {
    fn iter_week(self) -> WeekIterator;
}

impl IterWeek for Weekday {
    fn iter_week(self) -> WeekIterator {
        WeekIterator::new(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WeekIterator {
    start: Weekday,
    next: Option<Weekday>,
}

impl WeekIterator {
    #[must_use]
    pub const fn new(start: Weekday) -> Self {
        Self { start, next: None }
    }
}

impl Iterator for WeekIterator {
    type Item = Weekday;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            if next == self.start {
                None
            } else {
                self.next.replace(next.succ());
                Some(next)
            }
        } else {
            self.next.replace(self.start.succ());
            Some(self.start)
        }
    }
}
