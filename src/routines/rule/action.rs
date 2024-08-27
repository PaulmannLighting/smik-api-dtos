use serde::{Deserialize, Serialize};

pub use behavior::Behavior;
pub use destination::Destination;

pub mod behavior;
pub mod destination;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Action {
    destination: Destination,
    #[serde(rename = "behaviour")]
    behavior: Behavior,
}

impl Action {
    #[must_use]
    pub const fn new(destination: Destination, behavior: Behavior) -> Self {
        Self {
            destination,
            behavior,
        }
    }

    #[must_use]
    pub const fn destination(&self) -> &Destination {
        &self.destination
    }

    #[must_use]
    pub const fn behavior(&self) -> &Behavior {
        &self.behavior
    }
}
