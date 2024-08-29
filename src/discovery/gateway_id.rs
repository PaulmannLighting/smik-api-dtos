use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct GatewayId {
    id: String,
}

impl GatewayId {
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}
