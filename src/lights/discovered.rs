use super::{Area, Icon, Reachable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Light {
    id: u32,
    name: String,
    icon: Option<Icon>,
    #[serde(rename = "serialNo")]
    serial_number: String,
    state: Reachable,
    areas: Vec<Area>,
}

impl Light {
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
    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    #[must_use]
    pub const fn state(&self) -> &Reachable {
        &self.state
    }

    #[must_use]
    pub fn areas(&self) -> &[Area] {
        &self.areas
    }
}
