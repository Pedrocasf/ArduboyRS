use core::ops::Index;
use crate::cpu::kind::AVR_TYPE;
pub struct Flash {
    flash: [u16;AVR_TYPE.flash_size as usize],
}
impl Flash {
    pub fn new(rom_file:&[u16]) -> Flash {
        if rom_file.len() > AVR_TYPE.flash_size as usize {
            panic!("Flash file is too large");
        }
        Flash{
            flash:rom_file.try_into().unwrap(),
        }
    }
}
impl Index<u16> for Flash {
    type Output = u16;
    fn index(&self, index: u16) -> &u16 {
        &self.flash[index as usize]
    }
}