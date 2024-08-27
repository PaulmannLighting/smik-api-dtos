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

    #[must_use]
    pub const fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn with_light(mut self, light: Light) -> Self {
        self.lights.push(light);
        self
    }

    #[must_use]
    pub fn with_lights(mut self, lights: impl AsRef<[Light]>) -> Self {
        self.lights.extend_from_slice(lights.as_ref());
        self
    }

    #[must_use]
    pub fn with_scene(mut self, scene: Scene) -> Self {
        self.scenes.push(scene);
        self
    }

    #[must_use]
    pub fn with_scenes(mut self, scenes: impl AsRef<[Scene]>) -> Self {
        self.scenes.extend_from_slice(scenes.as_ref());
        self
    }
}

impl From<String> for Area {
    fn from(name: String) -> Self {
        Self::new(name, None, Vec::new(), Vec::new())
    }
}

impl From<&str> for Area {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}
