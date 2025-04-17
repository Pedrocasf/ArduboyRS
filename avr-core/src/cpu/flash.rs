use alloc::vec::Vec;
use core::ops::{Index, IndexMut};
use crate::cpu::data_memory::DataMemory;

pub struct Flash {
    flash: Vec<u16>
}
impl Flash {
    pub fn new(rom_file:&[u16]) -> Flash {
        let file = rom_file.to_vec();
        Flash{
            flash:file
        }
    }
}
impl Index<u16> for Flash {
    type Output = u16;
    fn index(&self, index: u16) -> &u16 {
        &self.flash[index as usize]
    }
}
impl IndexMut<u16> for Flash {
    fn index_mut(&mut self, index: u16) -> &u16 {
        panic!("can not write to flash memory at address {:#x?}", index);
    }
}