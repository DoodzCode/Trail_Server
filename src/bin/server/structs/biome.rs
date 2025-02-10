use core::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Biome {
    pub name: String,
    pub hightemp_heat: u16,             // heat expressed in degrees C' / F'
    pub hightemp_time: u16,             // time expressed as the day of the year
    pub lowtemp_heat: u16,
    pub lowtemp_time: u16,
    pub base_humidity: u16,             // base humidity same all year - modified by temp
    // pub b_type: BiomeType,
}

impl fmt::Display for Biome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Biome.name: {} \n", self.name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum BiomeType {
    Grasslands,
    Forest,
    Desert,
    Mountain,
    Canyon,
}

impl BiomeType {
    pub fn base_chance_of_rain(&self) -> u8 {
        match self {
            BiomeType::Grasslands => 50,
            BiomeType::Forest => 60,
            BiomeType::Desert => 10,
            BiomeType::Mountain => 40,
            BiomeType::Canyon => 30,
        }
    }
}

