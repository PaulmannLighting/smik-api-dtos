use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Reachable {
    #[serde(rename = "reachable", skip_serializing_if = "Option::is_none")]
    is_reachable: Option<bool>,
}

impl Reachable {
    #[must_use]
    pub const fn reachable(&self) -> Option<bool> {
        self.is_reachable
    }
}

impl From<Option<bool>> for Reachable {
    fn from(reachable: Option<bool>) -> Self {
        Self {
            is_reachable: reachable,
        }
    }
}

impl From<bool> for Reachable {
    fn from(reachable: bool) -> Self {
        Self {
            is_reachable: Some(reachable),
        }
    }
}

impl From<Reachable> for Option<bool> {
    fn from(reachable: Reachable) -> Self {
        reachable.is_reachable
    }
}
