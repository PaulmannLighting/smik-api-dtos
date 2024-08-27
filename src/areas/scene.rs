use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Scene {
    id: u32,
    name: String,
}

impl Scene {
    #[must_use]
    pub const fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }

    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
