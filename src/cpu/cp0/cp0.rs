use super::reg_status;
use super::reg_config;

#[derive(Debug, Default)]
pub struct Cp0 {
    reg_status: reg_status::RegStatus,
    reg_config: reg_config::RegConfig
}

impl Cp0 {
    pub fn write_reg(&mut self, index: u32, data: u64) {
        match index {
            12 => { self.reg_status = (data as u32).into() },
            16 => { self.reg_config = (data as u32).into(); },
            _ => panic!("Unrecognized Cp0 reg: {}, {:#x}", index, data)
        }
    }
}
