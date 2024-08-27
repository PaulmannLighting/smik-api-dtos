use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Icon {
    #[serde(rename = "lamp_standard")]
    Standard,
    #[serde(rename = "lamp_candle")]
    Candle,
    #[serde(rename = "lamp_spot")]
    Spotlight,
    #[serde(rename = "lamp_globe")]
    Globe,
    #[serde(rename = "lamp_controller")]
    Controller,
    #[serde(rename = "light_strip")]
    Strip,
    #[serde(rename = "light_coin")]
    Coin,
    #[serde(rename = "light_wall_light")]
    WallLight,
    #[serde(rename = "light_wall_spot")]
    WallSpotlight,
    #[serde(rename = "light_ceiling_spots")]
    CeilingSpotlight,
    #[serde(rename = "light_urail_spot")]
    UrailSpotlight,
    #[serde(rename = "light_floor_lamp")]
    Floor,
    #[serde(rename = "light_recfloor")]
    RecFloor,
    #[serde(rename = "light_pollard")]
    Pollard,
    #[serde(rename = "light_table")]
    Table,
    #[serde(rename = "light_cable_disc")]
    CableDisc,
    #[serde(rename = "light_pendant_long")]
    PendantLong,
    #[serde(rename = "light_pendant_round")]
    PendantRound,
    #[serde(rename = "light_panel_round")]
    PanelRound,
    #[serde(rename = "light_panel_square")]
    PanelSquare,
}
