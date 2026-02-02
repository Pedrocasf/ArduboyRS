use core::ops::{Index, IndexMut};
use crate::cpu::data_memory::REG_ADDR;
use crate::cpu::sreg::*;

pub const IO_SIZE:usize = 0x40;

pub struct IOs{
    sreg:Sreg,
    sph:u8,
    spl:u8,
    ddrb:u8,
    portb:u8
}
impl IOs {
    pub fn new() -> IOs {
        IOs{
            sreg:Sreg::new(),
            sph:0,
            spl:0,
            ddrb:0,
            portb:0
        }
    }
}
impl Index<u16> for IOs {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        match index {
            0x3F => &self.sreg.0,
            0x3E => &self.sph,
            0x3D => &self.spl,
            0x17 => &self.ddrb,
            0x18 => &self.portb,
            _ => panic!("io 0x{:x?} read accessed", index),
        }
    }
}

impl IndexMut<u16> for IOs {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        match index{
            0x3F => &mut self.sreg.0,
            0x3E => &mut self.sph,
            0x3D => &mut self.spl,
            0x17 => &mut self.ddrb,
            0x18 => &mut self.portb,
            _ => panic!("io 0x{:x?} write accessed", index),
        }
    }
}
