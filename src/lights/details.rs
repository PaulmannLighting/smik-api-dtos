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
    pub const fn id(&self) -> u32 {
        self.id
    }

    #[must_use]
    pub const fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn icon(&self) -> Icon {
        self.icon
    }

    #[must_use]
    pub const fn state(&self) -> &State {
        &self.state
    }

    #[must_use]
    pub const fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    #[must_use]
    pub const fn is_new(&self) -> bool {
        self.is_new
    }

    #[must_use]
    pub fn company(&self) -> &str {
        &self.company
    }

    #[must_use]
    pub fn firmware_version(&self) -> &str {
        &self.firmware_version
    }

    #[must_use]
    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }
}
