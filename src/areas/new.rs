use super::{Icon, Light, Scene};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Area {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    lights: Vec<Light>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    scenes: Vec<Scene>,
}

impl Area {
    #[must_use]
    pub const fn new(
        name: String,
        icon: Option<Icon>,
        lights: Vec<Light>,
        scenes: Vec<Scene>,
    ) -> Self {
        Self {
            name,
            icon,
            lights,
            scenes,
        }
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
    pub fn lights(&self) -> &[Light] {
        &self.lights
    }

    #[must_use]
    pub fn scenes(&self) -> &[Scene] {
        &self.scenes
    }
}
