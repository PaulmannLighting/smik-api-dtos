use super::Category;
use super::Icon;
use super::Light;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Scene {
    id: u32,
    #[serde(rename = "areaId")]
    area_id: u32,
    #[serde(rename = "sceneCategory")]
    category: Category,
    #[serde(rename = "isDeleted", skip_serializing_if = "Option::is_none")]
    is_deleted: Option<bool>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    lights: Vec<Light>,
}

impl Scene {
    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }

    #[must_use]
    pub const fn area_id(&self) -> u32 {
        self.area_id
    }

    #[must_use]
    pub const fn category(&self) -> Category {
        self.category
    }

    #[must_use]
    pub const fn is_deleted(&self) -> Option<bool> {
        self.is_deleted
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
}
