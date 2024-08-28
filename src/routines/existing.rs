use super::kind::Kind;
use super::rule::existing::Rule;
use super::Icon;
use crate::utils::parse_date_time;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Routine {
    id: u32,
    #[serde(rename = "sensorId", skip_serializing_if = "Option::is_none")]
    sensor_id: Option<u32>,
    name: String,
    #[serde(rename = "type")]
    kind: Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Icon>,
    #[serde(rename = "isActive")]
    is_active: bool,
    #[serde(rename = "isDeleted")]
    is_deleted: bool,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
    rules: Vec<Rule>,
}

impl Routine {
    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
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
    pub const fn is_deleted(&self) -> bool {
        self.is_deleted
    }

    #[must_use]
    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at.as_deref().and_then(parse_date_time)
    }

    #[must_use]
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at.as_deref().and_then(parse_date_time)
    }

    #[must_use]
    pub fn owner(&self) -> Option<&str> {
        self.owner.as_deref()
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
