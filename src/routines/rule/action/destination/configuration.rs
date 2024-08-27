use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
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

impl From<u32> for Configuration {
    fn from(id: u32) -> Self {
        Self { id }
    }
}

impl From<Configuration> for u32 {
    fn from(configuration: Configuration) -> Self {
        configuration.id
    }
}
