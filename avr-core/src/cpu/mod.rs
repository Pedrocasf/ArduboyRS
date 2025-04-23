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
    instr_array:[(fn(&mut CPU, InstructionData), InstructionData);AVR_TYPE.flash_size as usize],
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
        let instr_data = self.instr_array[self.pc as usize].1;
        self.instr_array[self.pc as usize].0(self, instr_data);
        #[cfg(std)]
        println!("Instr:{:x} \n PC:{:x}",instr, pc);
    }
    pub fn halt(&mut self, data:InstructionData){
        panic!("HALT at {:x?}, instr:{:x?}", self.pc, self.instr_array[self.pc as usize])
    }
    pub fn jmp(&mut self, data:InstructionData){
        self.pc = data.k;
    }
    pub fn eor(&mut self, data:InstructionData){
        self.data_memory[data.d as u16] ^= self.data_memory[data.r as u16];
    }
    pub fn out(&mut self, data:InstructionData){
        self.data_memory[data.d as u16+32] = self.data_memory[data.r as u16];
    }
}
pub fn translate(data:&[u16])->[(fn(&mut CPU, InstructionData), InstructionData);AVR_TYPE.flash_size as usize]{
    let mut r:[(fn(&mut CPU, InstructionData), InstructionData);AVR_TYPE.flash_size as usize] = [(CPU::halt, InstructionData{
        k:0,
        r:0,
        d:0
    }); AVR_TYPE.flash_size as usize];
    let data_len = data.len();
    let mut i = 0;
    while i < data_len {
        let d = data[i];
        let op0 = ((d & 0xF000) >> 12) as u8;
        let op1 = ((d & 0x0F00) >> 08) as u8;
        let op2 = ((d & 0x00F0) >> 04) as u8;
        let op3 = ((d & 0x000F) >> 00) as u8;
        match (op0,op1,op2,op3) {
            (0x9,0x4,_,0xC) |(0x9,0x4,_,0xD) | (0x9,0x5,_,0xC)| (0x9,0x5,_,0xD)=>{
                let k = data[i+1];
                r[i] = (CPU::jmp, InstructionData{k, r:0,d:0});
                println!("jmp k:{:x}, i:{:x}",k,i);
                i = k as usize;
                i-=1;
            },
            (0x2, 0x4,_,_) | (0x2, 0x5,_,_) | (0x2, 0x6,_,_) | (0x2, 0x7,_,_)=>{
                let dest = (data[i] & 0x01F0) >> 4;
                let reg = (data[i] & 0x000F) |((data[i] & 0x0200) >> 5);
                r[i] = (CPU::eor, InstructionData{k:0, r:reg as u8,d:dest as u8});
                println!("eor d:{:x}, r:{:x}, i:{:x}",dest,reg,i);
            }
            (0xB,0x8,_,_)|(0xB,0x9,_,_)|(0xB,0xA,_,_)|(0xB,0xB,_,_)|
            (0xB,0xC,_,_)|(0xB,0xD,_,_)|(0xB,0xE,_,_)|(0xB,0xF,_,_)=>{
                let reg = (data[i] & 0x01F0) >> 4;
                let a = (data[i] & 0x000F) |((data[i] & 0x0600) >> 5);
                r[i] = (CPU::out, InstructionData{k:0, r:reg as u8,d:a as u8});
                println!("out io:{:x}, r:{:x}, i:{:x}",a,reg,i);
            }
            (_,_,_,_) => panic!("Unsupported instruction, {:#x?} at addr {:#x?}", d, i),
        }
        i+=1;
    }
    r
}