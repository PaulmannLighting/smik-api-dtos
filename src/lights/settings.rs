use super::Icon;
use crate::lights::state::State;
use crate::scenes::Light;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<State>,
    #[serde(rename = "isNew", skip_serializing_if = "Option::is_none")]
    is_new: Option<bool>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
}

impl Settings {
    #[must_use]
    pub const fn new(
        id: Option<u32>,
        name: Option<String>,
        icon: Option<Icon>,
        state: Option<State>,
        is_new: Option<bool>,
        client_id: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            icon,
            state,
            is_new,
            client_id,
        }
    }

    #[must_use]
    pub const fn with_id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }

    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    #[must_use]
    pub const fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub const fn with_state(mut self, state: State) -> Self {
        self.state = Some(state);
        self
    }

    #[must_use]
    pub const fn with_is_new(mut self, is_new: bool) -> Self {
        self.is_new = Some(is_new);
        self
    }

    #[must_use]
    pub fn with_client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }
}

impl From<Light> for Settings {
    fn from(light: Light) -> Self {
        (&light).into()
    }
}

impl From<&Light> for Settings {
    fn from(light: &Light) -> Self {
        Self::new(
            Some(light.id()),
            None,
            None,
            Some(light.state().clone()),
            None,
            None,
        )
    }
}
