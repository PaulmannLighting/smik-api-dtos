use serde::{Deserialize, Serialize};

pub use configuration::Configuration;
pub use kind::Kind;

pub mod configuration;
pub mod kind;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "type")]
    kind: Kind,
    configuration: Configuration,
}

impl Event {
    #[must_use]
    pub const fn new(kind: Kind, configuration: Configuration) -> Self {
        Self {
            kind,
            configuration,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> Kind {
        self.kind
    }

    #[must_use]
    pub const fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}
