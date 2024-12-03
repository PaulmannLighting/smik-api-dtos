/// Integer percentage value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Percent(u8);

impl Percent {
    #[must_use]
    pub const fn new(value: u8) -> Option<Self> {
        if value <= 100 {
            Some(Self(value))
        } else {
            None
        }
    }

    #[must_use]
    pub const fn into_inner(self) -> u8 {
        self.0
    }
}

impl From<Percent> for u8 {
    fn from(value: Percent) -> Self {
        value.0
    }
}

impl TryFrom<u8> for Percent {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}
