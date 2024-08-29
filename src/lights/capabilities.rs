use serde::{Deserialize, Serialize};

pub use kind::Kind;

mod kind;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Capabilities {
    #[serde(rename = "type")]
    kind: Kind,
}
