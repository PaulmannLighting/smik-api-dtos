use crate::color::Rgb;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Action {
    on: bool,
    brightness: u8,
    color: Rgb,
    color_temperature: u16,
}
