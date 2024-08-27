use super::{Action, Condition, Event};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Rule {
    id: u32,
    events: Vec<Event>,
    conditions: Vec<Condition>,
    actions: Vec<Action>,
}

impl Rule {
    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }

    #[must_use]
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    #[must_use]
    pub fn conditions(&self) -> &[Condition] {
        &self.conditions
    }

    #[must_use]
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }
}
