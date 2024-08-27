use super::category::Category;
use crate::lights::Settings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Scene {
    #[serde(rename = "areaId")]
    area_id: u32,
    name: String,
    #[serde(rename = "sceneCateg")]
    category: Category,
    lights: Vec<Settings>,
}

impl Scene {
    #[must_use]
    pub const fn new(
        area_id: u32,
        name: String,
        category: Category,
        lights: Vec<Settings>,
    ) -> Self {
        Self {
            area_id,
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
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn category(&self) -> Category {
        self.category
    }

    #[must_use]
    pub fn lights(&self) -> &[Settings] {
        &self.lights
    }
}
