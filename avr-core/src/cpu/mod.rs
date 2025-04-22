pub mod kind;
mod sreg;
mod fuses;
mod data_memory;
mod io;
mod exios;
mod instructions;

use alloc::{vec, vec::Vec};
use kind::AVRKind;
use crate::cpu;
use crate::cpu::fuses::Fuses;
use crate::cpu::sreg::Flag;
use sreg::Sreg;
use crate::cpu::data_memory::DataMemory;
use crate::cpu::instructions::InstructionData;
use crate::cpu::kind::AVR_TYPE;

pub struct CPU{
    fuses: Fuses,
    status: Sreg,
    instr_array:[Option<(fn(&mut CPU), InstructionData)>;AVR_TYPE.flash_size as usize],
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
            instr_array:translate(rom_file),
            data_memory: DataMemory::new(),
            pc: 0,
        }
    }
    pub fn run(&mut self){
        self.instr_array[self.pc as usize](self);
        #[cfg(std)]
        println!("Instr:{:x} \n PC:{:x}",instr, pc);
    }
    pub fn halt(&mut self){
        panic!("HALT at {:x?}, instr:{:x?}", self.pc, self.instr_array[self.pc as usize])
    }
    pub fn jmp(&mut self){
        let k = self.instr_array[self.pc as usize].1;
        self.pc = k;
    }
}
pub fn translate(data:&[u16])->[fn(&mut CPU);AVR_TYPE.flash_size as usize]{
    let mut r = [CPU::halt; AVR_TYPE.flash_size as usize];
    let mut data_peek_iter = data.iter().peekable();
    let mut i = 0;
    while data_peek_iter.peek().is_some() {
        let d = data_peek_iter.next().unwrap();
        let op0 = ((d & 0xF000) >> 12) as u8;
        let op1 = ((d & 0x0F00) >> 08) as u8;
        let op2 = ((d & 0x00F0) >> 04) as u8;
        let op3 = ((d & 0x000F) >> 00) as u8;
        match (op0,op1,op2,op3) {
            (0x9,0x4,_,0xC) | (0x9,0x5,_,0xd)=>{
                let k = data_peek_iter.next().unwrap();
                r[i] = CPU::jmp;
                i+=1
            },
            (_,_,_,_) => panic!("Unsupported instruction, {:#x?}", d),
        }
        i+=1;
    }
    r
}