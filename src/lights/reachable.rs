use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Reachable {
    #[serde(skip_serializing_if = "Option::is_none")]
    reachable: Option<bool>,
}
