pub mod kind;
mod sreg;
mod fuses;
mod data_memory;
mod io;
mod flash;
mod exios;

use alloc::{vec, vec::Vec};
use kind::AVRKind;
use crate::cpu;
use crate::cpu::fuses::Fuses;
use crate::cpu::sreg::Flag;
use sreg::Sreg;
use crate::cpu::data_memory::DataMemory;
use crate::cpu::flash::Flash;

pub struct CPU{
    fuses: Fuses,
    status: Sreg,
    flash: Flash,
    data_memory: DataMemory,
    pc:u16,
}
impl CPU{
    pub fn new(rom_file:&[u16],kind:AVRKind ) -> CPU{
        let len = kind.flash_size as usize;
        if rom_file.len() > len{
            panic!("ROM file is too large!");
        }
        CPU {
            fuses: Fuses::new(kind.fuses),
            status: Sreg::new(),
            flash: Flash::new(rom_file),
            data_memory: DataMemory::new(),
            pc: 0,
        }
    }
    fn short_instr(&mut self)->u16{
        self.flash[self.pc]
    }
    pub fn run(&mut self){
        let instr = self.short_instr();
        LUT_UPPER[instr as usize >> 12](self);
        #[cfg(std)]
        println!("Instr:{:x} \n PC:{:x}",instr, pc);
    }
    pub fn halt(&mut self)->u8{
        panic!("HALT at {:x?}, instr:{:x?}", self.pc, self.flash[self.pc])
    }
}
pub const LUT_UPPER: [fn(&mut CPU)->u8; 0x10] = [
    CPU::halt,CPU::halt,CPU::halt,CPU::halt,
    CPU::halt,CPU::halt,CPU::halt,CPU::halt,
    CPU::halt,CPU::halt,CPU::halt,CPU::halt,
    CPU::halt,CPU::halt,CPU::halt,CPU::halt,
];