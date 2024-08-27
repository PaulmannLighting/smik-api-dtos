use super::{rule::new::Rule, Icon, Kind};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Routine {
    #[serde(rename = "sensorId", skip_serializing_if = "Option::is_none")]
    sensor_id: Option<u32>,
    name: String,
    #[serde(rename = "type")]
    kind: Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(rename = "isActive")]
    is_active: bool,
    rules: Vec<Rule>,
}

impl Routine {
    #[must_use]
    pub const fn new(
        sensor_id: Option<u32>,
        name: String,
        kind: Kind,
        icon: Option<Icon>,
        is_active: bool,
        rules: Vec<Rule>,
    ) -> Self {
        Self {
            sensor_id,
            name,
            kind,
            icon,
            is_active,
            rules,
        }
    }

    #[must_use]
    pub const fn sensor_id(&self) -> Option<u32> {
        self.sensor_id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn kind(&self) -> Kind {
        self.kind
    }

    #[must_use]
    pub const fn icon(&self) -> Option<Icon> {
        self.icon
    }

    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.is_active
    }

    #[must_use]
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }

    #[must_use]
    pub const fn with_sensor_id(mut self, sensor_id: u32) -> Self {
        self.sensor_id = Some(sensor_id);
        self
    }

    #[must_use]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    #[must_use]
    pub const fn with_kind(mut self, kind: Kind) -> Self {
        self.kind = kind;
        self
    }

    #[must_use]
    pub const fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub const fn with_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    #[must_use]
    pub fn with_rule(mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self
    }
}
