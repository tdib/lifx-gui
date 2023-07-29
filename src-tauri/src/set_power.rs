use bitfield::bitfield;

pub const SET_POWER_CONFIG_SIZE: usize = 2;

bitfield! {
    #[derive(Debug, Default)]
    pub struct SetPowerConfig(MSB0 [u8]);
    u16;

    pub u16, power, set_power: 15, 0;
}
