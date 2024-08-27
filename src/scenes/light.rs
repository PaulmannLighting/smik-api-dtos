use crate::lights::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Light {
    id: u32,
    state: State,
}

impl Light {
    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }

    #[must_use]
    pub const fn state(&self) -> &State {
        &self.state
    }
}
