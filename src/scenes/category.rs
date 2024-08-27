use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Category {
    Default = 0,
    Custom = 1,
    Featured = 2,
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "Default"),
            Self::Custom => write!(f, "Custom"),
            Self::Featured => write!(f, "Featured"),
        }
    }
}
