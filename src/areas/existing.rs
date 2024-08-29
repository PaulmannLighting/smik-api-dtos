use super::{Icon, Scene};
use crate::lights::details::Light;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Area {
    id: u32,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(default)]
    lights: Vec<Light>,
    #[serde(default)]
    scenes: Vec<Scene>,
}

impl Area {
    #[must_use]
    pub const fn new(
        id: u32,
        name: String,
        icon: Option<Icon>,
        lights: Vec<Light>,
        scenes: Vec<Scene>,
    ) -> Self {
        Self {
            id,
            name,
            icon,
            lights,
            scenes,
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
    pub fn lights(&self) -> &[Light] {
        &self.lights
    }

    #[must_use]
    pub fn scenes(&self) -> &[Scene] {
        &self.scenes
    }
}
