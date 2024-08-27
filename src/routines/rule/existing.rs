use super::{Action, Condition, Event};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Rule {
    id: u32,
    events: Vec<Event>,
    conditions: Vec<Condition>,
    actions: Vec<Action>,
}

impl Rule {
    #[must_use]
    pub const fn new(
        id: u32,
        events: Vec<Event>,
        conditions: Vec<Condition>,
        actions: Vec<Action>,
    ) -> Self {
        Self {
            id,
            events,
            conditions,
            actions,
        }
    }

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

    #[must_use]
    pub fn with_event(mut self, event: Event) -> Self {
        self.events.push(event);
        self
    }

    #[must_use]
    pub fn with_events(mut self, events: impl AsRef<[Event]>) -> Self {
        self.events.extend_from_slice(events.as_ref());
        self
    }

    #[must_use]
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    #[must_use]
    pub fn with_conditions(mut self, conditions: impl AsRef<[Condition]>) -> Self {
        self.conditions.extend_from_slice(conditions.as_ref());
        self
    }

    #[must_use]
    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    #[must_use]
    pub fn with_actions(mut self, actions: impl AsRef<[Action]>) -> Self {
        self.actions.extend_from_slice(actions.as_ref());
        self
    }
}

impl From<u32> for Rule {
    fn from(id: u32) -> Self {
        Self::new(id, Vec::new(), Vec::new(), Vec::new())
    }
}
