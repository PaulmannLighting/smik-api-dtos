use super::Category;
use crate::lights::Settings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Scene {
    #[serde(rename = "areaId")]
    area_id: u32,
    #[serde(rename = "sceneId")]
    scene_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "sceneCateg", skip_serializing_if = "Option::is_none")]
    category: Option<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lights: Option<Vec<Settings>>,
}

impl Scene {
    #[must_use]
    pub const fn new(
        area_id: u32,
        scene_id: u32,
        name: Option<String>,
        category: Option<Category>,
        lights: Option<Vec<Settings>>,
    ) -> Self {
        Self {
            area_id,
            scene_id,
            name,
            category,
            lights,
        }
    }

    #[must_use]
    pub const fn area_id(&self) -> u32 {
        self.area_id
    }

    #[must_use]
    pub const fn scene_id(&self) -> u32 {
        self.scene_id
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[must_use]
    pub const fn scene_category(&self) -> Option<Category> {
        self.category
    }

    #[must_use]
    pub fn lights(&self) -> Option<&[Settings]> {
        self.lights.as_deref()
    }
}
