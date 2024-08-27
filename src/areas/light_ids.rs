use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct LightIds {
    area_id: u32,
    light_ids: Vec<u32>,
}

impl LightIds {
    #[must_use]
    pub const fn new(area_id: u32, light_ids: Vec<u32>) -> Self {
        Self { area_id, light_ids }
    }

    #[must_use]
    pub const fn area_id(&self) -> u32 {
        self.area_id
    }

    #[must_use]
    pub fn light_ids(&self) -> &[u32] {
        &self.light_ids
    }
}
