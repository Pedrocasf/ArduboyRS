use std::println;
pub mod kind;
mod sreg;
mod fuses;
mod data_memory;
mod io;
mod exios;
mod instructions;

use alloc::{vec, vec::Vec};
use core::ops::{IndexMut, Index};
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
            instr_array:CPU::translate(rom_file),
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
    pub fn halt(&mut self,  data:InstructionData){
        panic!("HALT at {:x?}", self.pc)
    }
    pub fn jmp(&mut self,  data:InstructionData){
        let InstructionData::K(k) = data else {
            unreachable!();
        };
        self.pc = k;
    }
    pub fn eor(&mut self,  data:InstructionData){
        let InstructionData::DR(d,r) = data else {
            unreachable!();
        };
        self.data_memory[d as u16] ^= self.data_memory[r as u16];
    }
    pub fn out(&mut self,  data:InstructionData){
        let InstructionData::DR(d, r) = data else {
            unreachable!();
        };
        self.data_memory[d as u16+32] = self.data_memory[r as u16];
    }
    pub fn rjmp(&mut self, data:InstructionData){
        let InstructionData::SK(sk) = data else {
            unreachable!();
        };
        let pc = self.pc as i16;
        self.pc = (sk + pc) as u16;
    }
    pub fn reti(&mut self, data:InstructionData){
        let sph = self.data_memory[0x5E] as u16;
        let spl = self.data_memory[0x5D] as u16;
        let sp = sph << 8 | spl;
        let pch = self.data_memory[sp] as u16;
        let pcl = self.data_memory[sp+1] as u16;
        let sp = sp+2;
        self.data_memory[0x5E] = (sp >> 8) as u8;
        self.data_memory[0x5D] = spl as u8;
        let pc = pch << 8 | pcl;
        let sreg = Sreg(self.data_memory[0x5F]);
        self.data_memory[0x5F] = Sreg::new
        self.pc = pc;

    }
    pub fn translate(data:&[u16])->[(fn(&mut CPU, InstructionData), InstructionData);AVR_TYPE.flash_size as usize]{
        let mut r:[(fn(&mut CPU, InstructionData), InstructionData );AVR_TYPE.flash_size as usize] = [(CPU::halt,InstructionData::NILL); AVR_TYPE.flash_size as usize];
        let data_len = data.len();
        let mut i = 0;
        while i < data_len {
            let d = data[i];
            let op0 = ((d & 0xF000) >> 12) as u8;
            let op1 = ((d & 0x0F00) >> 08) as u8;
            let op2 = ((d & 0x00F0) >> 04) as u8;
            let op3 = ((d & 0x000F) >> 00) as u8;
            match (op0,op1,op2,op3) {
                (0x9,0x5,0x1,0x8) =>{
                    r[i] = (CPU::reti, InstructionData::NILL);
                    println!("reti i:{:x}", i);
                },
                (0xC,_,_,_) => {
                    r[i] = (CPU::rjmp, InstructionData::SK(((data[i]<<4)as i16)>>4));
                    println!("rjmp i:{:x}", i);
                },
                (0x9,0x4,_,0xC) |(0x9,0x4,_,0xD) | (0x9,0x5,_,0xC)| (0x9,0x5,_,0xD)=>{
                    r[i] = (CPU::jmp,InstructionData::K(data[i+1]));

                    println!("jmp i:{:x}",i);
                    i+=1;
                },
                (0x2, 0x4,_,_) | (0x2, 0x5,_,_) | (0x2, 0x6,_,_) | (0x2, 0x7,_,_)=>{
                    let dest = (data[i] & 0x01F0) >> 4;
                    let reg = (data[i] & 0x000F) |((data[i] & 0x0200) >> 5);
                    r[i] = (CPU::eor, InstructionData::DR(dest as u8,reg as u8));
                    println!("eor d:{:x}, r:{:x}, i:{:x}",dest,reg,i);
                }
                (0xB,0x8,_,_)|(0xB,0x9,_,_)|(0xB,0xA,_,_)|(0xB,0xB,_,_)|
                (0xB,0xC,_,_)|(0xB,0xD,_,_)|(0xB,0xE,_,_)|(0xB,0xF,_,_)=>{
                    let reg = (data[i] & 0x01F0) >> 4;
                    let a = (data[i] & 0x000F) |((data[i] & 0x0600) >> 5);
                    r[i] = (CPU::out, InstructionData::DR(a as u8, reg as u8));
                    println!("out io:{:x}, r:{:x}, i:{:x}",a,reg,i);
                }
                (_,_,_,_) => panic!("Unsupported instruction, {:#x?} at addr {:#x?}", d, i),
            }
            i+=1;
        }
        r.clone()
    }
}
