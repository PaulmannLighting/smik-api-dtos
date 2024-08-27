use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Configuration {
    id: u32,
}

impl Configuration {
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }
}
