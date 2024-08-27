use serde::{Deserialize, Serialize};

pub use kind::Kind;

mod kind;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Capabilities {
    #[serde(rename = "type")]
    kind: Kind,
}
