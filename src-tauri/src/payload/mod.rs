mod set_colour;
mod set_power;

use set_colour::{SetColourConfig, SET_COLOUR_CONFIG_SIZE};
use set_power::{SetPowerConfig, SET_POWER_CONFIG_SIZE};

#[derive(Debug)]
pub enum Payload {
    SetColour(SetColourConfig<[u8; SET_COLOUR_CONFIG_SIZE]>),
    SetPower(SetPowerConfig<[u8; SET_POWER_CONFIG_SIZE]>),
}

impl Payload {
    pub fn get_bytes(&self) -> Vec<u8> {
        match self {
            Self::SetColour(config) => config.0.to_vec(),
            Self::SetPower(config) => config.0.to_vec(),
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            Self::SetColour(_) => SET_COLOUR_CONFIG_SIZE,
            Self::SetPower(_) => SET_POWER_CONFIG_SIZE,
        }
    }

    pub fn set_power(power: bool) -> Payload {
        Self::SetPower({
            let mut config = SetPowerConfig::default();
            let power = if power { 65535 } else { 0 };
            config.set_power(power);
            config
        })
    }

    pub fn set_colour(
        hue: u16,
        saturation: u16,
        brightness: u16,
        kelvin: u16,
        duration: u32,
    ) -> Payload {
        Self::SetColour({
            let mut config = SetColourConfig::default();

            config.set_hue(hue);
            config.set_saturation(saturation);
            config.set_brightness(brightness);
            config.set_kelvin(kelvin);
            config.set_duration(duration);

            config
        })
    }
}
