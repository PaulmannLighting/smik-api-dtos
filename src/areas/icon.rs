use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum Icon {
    #[serde(rename = "default_area")]
    Default,
    #[serde(rename = "area_gen_home")]
    Home,
    #[serde(rename = "area_gen_upper")]
    Upper,
    #[serde(rename = "area_gen_ground")]
    Ground,
    #[serde(rename = "area_gen_lower")]
    Lower,
    #[serde(rename = "area_gen_party")]
    Party,
    #[serde(rename = "area_in_livingroom")]
    LivingRoom,
    #[serde(rename = "area_in_kitchen")]
    Kitchen,
    #[serde(rename = "area_in_eating")]
    DiningRoom,
    #[serde(rename = "area_in_bedroom")]
    Bedroom,
    #[serde(rename = "area_in_kidsroom")]
    KidsRoom,
    #[serde(rename = "area_in_office")]
    Office,
    #[serde(rename = "area_in_bathroom")]
    Bathroom,
    #[serde(rename = "area_in_restroom")]
    Restroom,
    #[serde(rename = "area_in_corridor")]
    Hallway,
    #[serde(rename = "area_in_stairs")]
    Staircase,
    #[serde(rename = "area_in_garage")]
    Garage,
    #[serde(rename = "area_out_garden")]
    Garden,
    #[serde(rename = "area_out_terrace")]
    Terrace,
    #[serde(rename = "area_out_balcony")]
    Balcony,
    #[serde(rename = "area_out_porch")]
    Porch,
    #[serde(rename = "area_out_entrance")]
    Entrance,
    #[serde(rename = "area_out_parking")]
    Parking,
    #[serde(rename = "area_act_fitness")]
    Fitness,
    #[serde(rename = "area_act_reading")]
    Reading,
    #[serde(rename = "area_act_music")]
    Music,
    #[serde(rename = "area_act_tv")]
    Tv,
    #[serde(rename = "area_act_computer")]
    Computer,
}
