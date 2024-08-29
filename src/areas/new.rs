use super::Icon;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Area {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    lights: Vec<u32>,
}

impl Area {
    #[must_use]
    pub const fn new(name: String, icon: Option<Icon>, lights: Vec<u32>) -> Self {
        Self { name, icon, lights }
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
    pub fn lights(&self) -> &[u32] {
        &self.lights
    }

    #[must_use]
    pub const fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn with_light(mut self, light: u32) -> Self {
        self.lights.push(light);
        self
    }

    #[must_use]
    pub fn with_lights(mut self, lights: &[u32]) -> Self {
        self.lights.extend_from_slice(lights);
        self
    }
}

impl From<String> for Area {
    fn from(name: String) -> Self {
        Self::new(name, None, Vec::new())
    }
}

impl From<&str> for Area {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}
