use super::{Action, Icon};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct Area {
    id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Action>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    client_id: Option<u32>,
}

impl Area {
    #[must_use]
    pub const fn new(
        id: u32,
        name: Option<String>,
        icon: Option<Icon>,
        action: Option<Action>,
        client_id: Option<u32>,
    ) -> Self {
        Self {
            id,
            name,
            icon,
            action,
            client_id,
        }
    }

    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    #[must_use]
    pub const fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub const fn with_action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    #[must_use]
    pub const fn with_client_id(mut self, client_id: u32) -> Self {
        self.client_id = Some(client_id);
        self
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct Areas {
    area_ids: Vec<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Action>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    client_id: Option<u32>,
}

impl Areas {
    #[must_use]
    pub const fn new(
        area_ids: Vec<u32>,
        name: Option<String>,
        icon: Option<Icon>,
        action: Option<Action>,
        client_id: Option<u32>,
    ) -> Self {
        Self {
            area_ids,
            name,
            icon,
            action,
            client_id,
        }
    }

    #[must_use]
    pub fn with_area_ids(mut self, area_ids: impl AsRef<[u32]>) -> Self {
        self.area_ids.extend_from_slice(area_ids.as_ref());
        self
    }

    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    #[must_use]
    pub const fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub const fn with_action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    #[must_use]
    pub const fn with_client_id(mut self, client_id: u32) -> Self {
        self.client_id = Some(client_id);
        self
    }
}
