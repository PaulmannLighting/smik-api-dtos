use super::{Capabilities, Icon, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Light {
    id: u32,
    #[serde(rename = "isDeleted")]
    is_deleted: bool,
    name: String,
    icon: Icon,
    state: State,
    capabilities: Capabilities,
    #[serde(rename = "isNew")]
    is_new: bool,
    company: String,
    #[serde(rename = "firmwareVersion")]
    firmware_version: String,
    #[serde(rename = "serialNo")]
    serial_number: String,
}

impl Light {
    #[must_use]
    pub const fn state(&self) -> &State {
        &self.state
    }
}
