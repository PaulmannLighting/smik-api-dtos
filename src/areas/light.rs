mod kind;

use crate::lights::Icon;
pub use kind::Kind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Light {
    id: u32,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(rename = "type")]
    kind: Option<Kind>,
    min: u32,
    max: u32,
    manufacturer: String,
    model: String,
}

impl Light {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        id: u32,
        name: String,
        icon: Option<Icon>,
        kind: Option<Kind>,
        min: u32,
        max: u32,
        manufacturer: String,
        model: String,
    ) -> Self {
        Self {
            id,
            name,
            icon,
            kind,
            min,
            max,
            manufacturer,
            model,
        }
    }

    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn icon(&self) -> Option<Icon> {
        self.icon
    }

    #[must_use]
    pub const fn kind(&self) -> Option<Kind> {
        self.kind
    }

    #[must_use]
    pub const fn min(&self) -> u32 {
        self.min
    }

    #[must_use]
    pub const fn max(&self) -> u32 {
        self.max
    }

    #[must_use]
    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }
}
