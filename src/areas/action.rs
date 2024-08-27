use crate::color::Rgb;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Action {
    on: bool,
    brightness: u8,
    color: Rgb,
    color_temperature: u16,
}
