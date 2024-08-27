use super::{Action, Condition, Event};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Rule {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    events: Vec<Event>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    conditions: Vec<Condition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    actions: Vec<Action>,
}

impl Rule {
    #[must_use]
    pub const fn new(events: Vec<Event>, conditions: Vec<Condition>, actions: Vec<Action>) -> Self {
        Self {
            events,
            conditions,
            actions,
        }
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
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
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
