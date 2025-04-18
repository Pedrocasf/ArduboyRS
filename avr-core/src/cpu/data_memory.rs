use core::ops::{Index, IndexMut};
use crate::cpu::exios::ExIOs;
use crate::cpu::io::IOs;
use crate::cpu::kind::{AVRKind, AVR_TYPE};
pub enum IndexRegisters{
    X,Y,Z
}
pub const REG_ADDR:u16 = 0x00;
pub const REG_SIZE:u16 = 0x20;
pub const REG_RANGE:u16 = REG_ADDR + REG_SIZE;
pub const IOS_ADDR:u16 = REG_ADDR + REG_SIZE;
pub const IOS_SIZE:u16 = 0x3F;
pub const IOS_RANGE:u16 = IOS_ADDR + IOS_SIZE;
pub const EXIOS_ADDR:u16 = IOS_ADDR + IOS_SIZE;
pub const EXIOS_SIZE:u16 = AVR_TYPE.exios_size+1;
pub const EXIOS_RANGE:u16 = EXIOS_ADDR+EXIOS_SIZE;
pub const SRAM_ADDR:u16 = EXIOS_ADDR+EXIOS_ADDR;
pub const SRAM_SIZE:u16 = AVR_TYPE.sram_size;
pub const SRAM_RANGE:u16 = SRAM_ADDR+SRAM_SIZE;
pub struct DataMemory {
    regs: [u8;REG_SIZE as usize],
    ios: IOs,
    exios: ExIOs,
    sram: [u8;AVR_TYPE.sram_size as usize],
}
impl DataMemory {
    pub fn new() -> DataMemory {
        DataMemory {
            regs: [0; REG_SIZE as usize],
            ios: IOs::new(),
            exios: ExIOs::new(),
            sram:[0;AVR_TYPE.sram_size as usize],
        }
    }
}
impl Index<u16> for DataMemory {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        match index {
            REG_ADDR..REG_SIZE => &self.regs[index as usize],
            IOS_ADDR..IOS_RANGE => &self.ios[index - REG_RANGE],
            EXIOS_ADDR..EXIOS_RANGE => &self.exios[index - IOS_RANGE],
            SRAM_ADDR..SRAM_RANGE => &self.sram[index as usize - EXIOS_RANGE as usize],
            _=> unreachable!()
            }
        }
    }

impl IndexMut<u16> for DataMemory {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        match index {
            REG_ADDR..REG_SIZE => &mut self.regs[index as usize],
            IOS_ADDR..IOS_RANGE => &mut self.ios[index - REG_RANGE],
            EXIOS_ADDR..EXIOS_RANGE => &mut self.exios[index - IOS_RANGE],
            SRAM_ADDR..SRAM_RANGE => &mut self.sram[index as usize - EXIOS_RANGE as usize],
            _=> unreachable!()
        }
    }
}