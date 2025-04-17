use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};
use crate::cpu::io::IOs;
use crate::cpu::kind::AVRKind;
pub const REG_ADDR:usize = 0x00;
pub const REG_SIZE:usize = 0x20;
pub const IOS_ADDR:usize = REG_ADDR + REG_SIZE;
pub const IOS_SIZE:usize = 0x40;
pub struct DataMemory {
    regs: [u8;REG_SIZE],
    ios_size:usize,
    ios: IOs,
    exios_size:Option<usize>,
    exios:Option<Vec<u8>>,
    sram_size:usize,
    sram: Vec<u8>,
}
impl DataMemory {
    pub fn new(kind:AVRKind) -> DataMemory {
        if let Some(exios) = kind.exios {
            return DataMemory {
                regs: [0; REG_SIZE],
                ios_size:kind.ios,
                ios: IOs::new(),
                exios_size:Some(exios),
                exios: Some(vec![0; exios]),
                sram_size:kind.sram_size,
                sram: vec![0; kind.sram_size],
            }
        }
        DataMemory {
            regs: [0; REG_SIZE],
            ios_size:kind.ios,
            ios: IOs::new(),
            exios_size:None,
            exios: None,
            sram_size:kind.sram_size,
            sram: vec![0; kind.sram_size],
        }
    }
}
impl Index<u16> for DataMemory {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        let reg_addr = REG_ADDR as u16;
        let reg_size = REG_SIZE as u16;
        let reg_range = reg_addr + reg_size;
        let ios_addr = IOS_ADDR as u16;
        let ios_size = IOS_SIZE as u16;
        let ios_range = ios_addr+ios_size;
        let exios_addr = ios_range;
        let exios_size = if self.exios.is_some() { self.exios_size.unwrap() as u16} else {0};
        let exios_range = exios_addr + exios_size;
        let sram_addr = exios_addr + exios_size;
        let sram_size = self.sram_size as u16;
        let sram_range = sram_addr+sram_size;
        match index {
            reg_addr..reg_size => &self.regs[index as usize],
            ios_addr..ios_range=> &self.ios[index as usize - reg_range as usize],
            exios_addr..exios_range => &self.exios[index as usize - ios_range as usize],
            sram_addr..sram_range => &self.sram[index as usize - exios_range as usize],
            _=> unreachable!()
            }
        }
    }

impl IndexMut<u16> for DataMemory {
    fn index_mut(&mut self, index: u16) -> &u8 {
        let reg_addr = REG_ADDR as u16;
        let reg_size = REG_SIZE as u16;
        let reg_range = reg_addr + reg_size;
        let ios_addr = IOS_ADDR as u16;
        let ios_size = IOS_SIZE as u16;
        let ios_range = ios_addr+ios_size;
        let exios_addr = ios_range;
        let exios_size = if self.exios.is_some() { self.exios_size.unwrap() as u16} else {0};
        let exios_range = exios_addr + exios_size;
        let sram_addr = exios_addr + exios_size;
        let sram_size = self.sram_size as u16;
        let sram_range = sram_addr+sram_size;
        match index {
            reg_addr..reg_size => &mut self.regs[index as usize],
            ios_addr..ios_range=> &mut self.ios[index as usize - reg_range as usize],
            exios_addr..exios_range => &mut self.exios[index as usize - ios_range as usize],
            sram_addr..sram_range => &mut self.sram[index as usize - exios_range as usize],
            _=> unreachable!()
        }
    }
}