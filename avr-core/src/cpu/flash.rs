use core::ops::Index;
use crate::cpu::instructions::Instructions;
use crate::cpu::kind::AVR_TYPE;
use crate::cpu::instructions;
pub struct Flash {
    flash: [Instructions;AVR_TYPE.flash_size as usize],
}
impl Flash {
    pub fn new(rom_file:&[u16]) -> Flash {
        if rom_file.len() > AVR_TYPE.flash_size as usize {
            panic!("Flash file is too large");
        }
        Flash{
            flash:instructions::translate(rom_file),
        }
    }
}
impl Index<u16> for Flash {
    type Output = Instructions;
    fn index(&self, index: u16) -> &Instructions {
        &self.flash[index as usize]
    }
}