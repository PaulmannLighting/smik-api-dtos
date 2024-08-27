use serde::{Deserialize, Serialize};

pub use configuration::Configuration;
pub use kind::Kind;

pub mod configuration;
mod kind;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "type")]
    kind: Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Configuration>,
}

impl Destination {
    #[must_use]
    pub const fn new(kind: Kind, configuration: Option<Configuration>) -> Self {
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
    pub const fn configuration(&self) -> Option<&Configuration> {
        self.configuration.as_ref()
    }

    #[must_use]
    pub const fn with_kind(mut self, kind: Kind) -> Self {
        self.kind = kind;
        self
    }

    #[must_use]
    pub const fn with_configuration(mut self, configuration: Configuration) -> Self {
        self.configuration = Some(configuration);
        self
    }
}
