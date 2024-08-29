use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct TokenAndKey {
    token: String,
    key: String,
}

impl TokenAndKey {
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }
}
