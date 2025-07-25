use core::ops::{Index, IndexMut};
use crate::cpu::data_memory::REG_ADDR;
use crate::cpu::sreg::*;

pub const IO_SIZE:usize = 0x40;

pub struct IOs{
    sreg:Sreg,
    sph:u8,
    spl:u8,
}
impl IOs {
    pub fn new() -> IOs {
        IOs{
            sreg:Sreg::new(),
            sph:0,
            spl:0,
        }
    }
}
impl Index<u16> for IOs {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        match index {
            0x5F => &self.sreg.0,
            0x5E => &self.sph,
            0x5D => &self.spl,
            _ => panic!("io {:x?} read accessed", index),
        }
    }
}

impl IndexMut<u16> for IOs {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        match index{
            0x5F => &mut self.sreg.0,
            0x5E => &mut self.sph,
            0x5D => &mut self.spl,
            _ => panic!("io {:x?} write accessed", index),
        }
    }
}
