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
    /// Returns the ID as a `u32`.
    ///
    /// # Errors
    /// Returns a `ParseIntError` if the ID is not a valid `u32`.
    pub fn id(&self) -> Result<u32, ParseIntError> {
        self.id.parse()
    }

    /// Returns the IP address as an `IpAddr`.
    ///
    /// # Errors
    /// Returns a `AddrParseError` if the IP address is not a valid `IpAddr`.
    pub fn ip_address(&self) -> Result<IpAddr, AddrParseError> {
        self.ip_address.parse()
    }
}
