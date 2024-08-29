use serde::{Deserialize, Serialize};
use std::net::{AddrParseError, IpAddr};
use std::num::ParseIntError;

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct Device {
    id: String,
    #[serde(rename = "ipaddress")]
    ip_address: String,
}

impl Device {
    pub fn id(&self) -> Result<u32, ParseIntError> {
        self.id.parse()
    }

    pub fn ip_address(&self) -> Result<IpAddr, AddrParseError> {
        self.ip_address.parse()
    }
}
