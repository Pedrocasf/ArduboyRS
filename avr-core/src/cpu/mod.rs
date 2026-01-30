use std::println;
pub mod kind;
mod sreg;
mod fuses;
mod data_memory;
mod io;
mod exios;
mod instructions;
mod lazy_flags;

use core::ops::{IndexMut, Index};
use std::path::absolute;
use kind::AVRKind;
use crate::cpu::fuses::Fuses;
use sreg::Sreg;
use crate::cpu::data_memory::DataMemory;
use crate::cpu::instructions::Instruction::*;
use crate::cpu::instructions::InstructionData;
use crate::cpu::instructions::InstructionData::BR;
use crate::cpu::kind::AVR_TYPE;
use crate::cpu::lazy_flags::*;

pub const SPL: u16 = 0x5D;
pub const SPH: u16 = 0x5E;
pub const SREG:u16 = 0x5F;
pub struct CPU{
    fuses: Fuses,
    instr_array:[(fn(&mut CPU, InstructionData), InstructionData);AVR_TYPE.flash_size as usize],
    data_memory: DataMemory,
    lazy_flags: LazyFlags,
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
            lazy_flags: LazyFlags::new(),
            pc: 0,
        }
    }
    pub fn run(&mut self){
        let instr_data = self.instr_array[self.pc as usize].1;
        self.instr_array[self.pc as usize].0(self, instr_data);
        //#[cfg(feature = "std")]
        //println!("Instr:{:x} \n PC:{:x}", self.pc);
    }
    pub fn halt(&mut self,  _data:InstructionData){
        panic!("HALT at {:x?}", self.pc)
    }
    pub fn jmp(&mut self,  data:InstructionData){
        let InstructionData::K(k) = data else {
            unreachable!();
        };
        #[cfg(feature = "std")]
        println!("JMP at {:x?} to {:x?}", self.pc, k);
        self.pc = k;

    }
    pub fn eor(&mut self,  data:InstructionData){
        let InstructionData::DR(d,r) = data else {
            unreachable!();
        };
        let op1 = self.data_memory[r as u16];
        let op2 = self.data_memory[d as u16];
        self.lazy_flags.op1 = op1;
        self.lazy_flags.op2 = op2;
        let res = op1 ^ op2;
        self.data_memory[d as u16] = res;
        self.lazy_flags.res = res as i16;
        #[cfg(feature = "std")]
        println!("EOR at {:x?}", self.pc);
        self.pc += 1;
    }
    pub fn out(&mut self,  data:InstructionData){
        let InstructionData::DR(d, r) = data else {
            unreachable!();
        };
        self.data_memory[d as u16+32] = self.data_memory[r as u16];
        #[cfg(feature = "std")]
        println!("OUT at {:x?}", self.pc);
    }
    pub fn rjmp(&mut self, data:InstructionData){
        #[cfg(feature = "std")]
        println!("RJMP at {:x?}", self.pc);

        let InstructionData::SK(sk) = data else {
            unreachable!();
        };
        let pc = self.pc as i16;
        self.pc = (sk + pc + 1) as u16;
    }
    pub fn reti(&mut self, _data:InstructionData){
        let sph = self.data_memory[SPH] as u16;
        let spl = self.data_memory[SPL] as u16;
        let sp = sph << 8 | spl;
        let pch = self.data_memory[sp] as u16;
        let pcl = self.data_memory[sp+1] as u16;
        let sp = sp+2;
        self.data_memory[SPH] = (sp >> 8) as u8;
        self.data_memory[SPL] = spl as u8;
        let pc = pch << 8 | pcl;
        let mut sreg = Sreg(self.data_memory[SREG]);
        sreg.set_i(true);
        self.data_memory[SREG] = sreg.0;
        self.pc = pc;
        #[cfg(feature = "std")]
        println!("RETI at {:x?}", self.pc);
    }
    pub fn cpse(&mut self, data:InstructionData){
        let InstructionData::DR(d, r) = data else {
            unreachable!();
        };
        if self.data_memory[d as u16] == self.data_memory[r as u16]{
            self.pc += 1;
        }
        self.pc += 1;
        #[cfg(feature = "std")]
        println!("CPSE at {:x?}", self.pc);
    }
    pub fn muls(&mut self, data:InstructionData){
        let InstructionData::DR(d, r) = data else {
            unreachable!();
        };
        let rd = self.data_memory[d as u16] as i16;
        let rr = self.data_memory[r as u16+16] as i16;
        self.lazy_flags.op1 = rd as u8;
        self.lazy_flags.op2 = rr as u8;
        let mult = rd * rr;
        self.lazy_flags.res = mult;
        self.data_memory[0] = mult as u8;
        self.data_memory[1] = (mult >> 8) as u8;
        #[cfg(feature = "std")]
        println!("MULS at {:x?}", self.pc);
    }
    pub fn cpi(&mut self, data:InstructionData){
        let InstructionData::DR(imm, reg) = data else {
            unreachable!();
        };
        let rd = self.data_memory[reg as u16] as i8;
        self.lazy_flags.op1 = rd as u8;
        self.lazy_flags.op2 = imm as u8;
        let (val, o) = rd.overflowing_sub(imm as i8);
        self.lazy_flags.res = val as i16;
        #[cfg(feature = "std")]
        println!("CPI at {:x?}", self.pc);
        self.pc += 1;
    }
    pub fn cpc(&mut self, data:InstructionData){
        let InstructionData::DR(rd, rr) = data else {
            unreachable!();
        };
        let rd = self.data_memory[rd as u16] as i8;
        let rr = self.data_memory[rr as u16] as i8;
        self.lazy_flags.op1 = rd as u8;
        self.lazy_flags.op2 = rr as u8;
        let (val, o) = rd.overflowing_sub(rd as i8);
        self.lazy_flags.res = val as i16;
        #[cfg(feature = "std")]
        println!("CPC at {:x?}", self.pc);
        self.pc += 1;
    }
    pub fn nop(&mut self, data:InstructionData){
        println!("NOP at {:x?}", self.pc);
    }
    pub fn ldi(&mut self, data:InstructionData){
        let InstructionData::DR(imm, reg) = data else {
            unreachable!();
        };
        self.data_memory[reg as u16] = imm;
        #[cfg(feature = "std")]
        println!("LDI at {:x?}", self.pc);
        self.pc += 1;
    }
    pub fn brbc(&mut self, data:InstructionData){
        let InstructionData::BR(flag, offset) = data else {
            unreachable!();
        };
        #[cfg(feature = "std")]
        println!("BRBC at {:x?}", self.pc);
        if self.lazy_flags.calc_flag(flag) == false {
            self.pc += offset as i16 as u16;
        }
        self.pc +=1;

    }
    pub fn bsetr(&mut self, data:InstructionData){
        let InstructionData::BIT(set) = data else {
            unreachable!();
        };

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
                (0x0, 0x2, _, _) =>{
                    let dest = ((data[i]>>4)& 0x000F)as u8 +16;
                    let reg = (data[i] & 0x000F)as u8 +16;
                    r[i] = (CPU::muls, InstructionData::DR(dest as u8, reg as u8));
                    #[cfg(feature = "std")]
                    println!("muls d:{:x}, r:{:x}, i:{:x}", dest, reg, i);
                }
                (0x1, 0x0, _, _) |(0x1, 0x1, _, _) | (0x1, 0x2, _, _) | (0x1, 0x3, _, _) =>{
                    let dest = (data[i] & 0x01F0) >> 4;
                    let reg = (data[i] & 0x000F) |((data[i] & 0x0200) >> 5);
                    r[i] = (CPU::cpse, InstructionData::DR(dest as u8, reg as u8));
                    #[cfg(feature = "std")]
                    println!("cpse d:{:x}, r:{:x}, i:{:x}", dest, reg, i);
                },
                (0x9,0x5,0x1,0x8) =>{
                    r[i] = (CPU::reti, InstructionData::NILL);
                    #[cfg(feature = "std")]
                    println!("reti i:{:x}", i);
                },
                (0xC,_,_,_) => {
                    r[i] = (CPU::rjmp, InstructionData::SK(((data[i]<<4)as i16)>>4));
                    #[cfg(feature = "std")]
                    println!("rjmp i:{:x}", i);
                },
                (0x9,0x4,_,0xC) |(0x9,0x4,_,0xD) | (0x9,0x5,_,0xC)| (0x9,0x5,_,0xD)=>{
                    r[i] = (CPU::jmp,InstructionData::K(data[i+1]));
                    #[cfg(feature = "std")]
                    println!("jmp i:{:x}",i);
                    i+=1;
                },
                (0x2, 0x4,_,_) | (0x2, 0x5,_,_) | (0x2, 0x6,_,_) | (0x2, 0x7,_,_)=>{
                    let dest = (data[i] & 0x01F0) >> 4;
                    let reg = (data[i] & 0x000F) |((data[i] & 0x0200) >> 5);
                    r[i] = (CPU::eor, InstructionData::DR(dest as u8,reg as u8));
                    #[cfg(feature = "std")]
                    println!("eor d:{:x}, r:{:x}, i:{:x}",dest,reg,i);
                }
                (0xB,0x8,_,_)|(0xB,0x9,_,_)|(0xB,0xA,_,_)|(0xB,0xB,_,_)|
                (0xB,0xC,_,_)|(0xB,0xD,_,_)|(0xB,0xE,_,_)|(0xB,0xF,_,_)=>{
                    let reg = (data[i] & 0x01F0) >> 4;
                    let a = (data[i] & 0x000F) |((data[i] & 0x0600) >> 5);
                    r[i] = (CPU::out, InstructionData::DR(a as u8, reg as u8));
                    #[cfg(feature = "std")]
                    println!("out io:{:x}, r:{:x}, i:{:x}",a,reg,i);
                }
                (0x3,_,_,_)=>{
                    let reg = (((data[i] & 0x0F0) >> 4) + 0x10) as u8;
                    let imm = (data[i] & 0x000F | ((data[i] & 0x0F00) >> 4)) as u8;
                    r[i] = (CPU::cpi, InstructionData::DR(imm, reg));
                    #[cfg(feature = "std")]
                    println!("cpi imm:{:x},reg:{:x} i:{:x}",imm, reg, i);
                }
                (0x0,0x0,0x0,0x0)=>{
                    r[i] = (CPU::nop, InstructionData::NILL);
                    #[cfg(feature = "std")]
                    println!("nop i:{:x}", i);
                }
                (0xE,_,_,_) => {
                    let reg = (((data[i] & 0x0F0) >> 4) + 0x10) as u8;
                    let imm = (data[i] & 0x000F | ((data[i] & 0x0F00) >> 4)) as u8;
                    r[i] = (CPU::ldi, InstructionData::DR(imm, reg));
                    #[cfg(feature = "std")]
                    println!("ldi imm:{:x}, reg:{:x}, i:{:x}",imm,reg,i);
                }
                (0x0,0x4,_,_)|(0x0,0x5,_,_)|(0x0,0x6,_,_)|(0x0,0x7,_,_) =>{
                    let reg_dest = (data[i] & 0x01F0) >> 4;
                    let reg_r = (data[i] & 0x000F) |((data[i] & 0x0200) >> 5);
                    r[i] = (CPU::cpc, InstructionData::DR(reg_dest as u8, reg_r as u8));
                    #[cfg(feature = "std")]
                    println!("cpc rd:{:x}, rr:{:x}, i:{:x}",reg_dest,reg_r,i);
                }
                (0xF,0x4,_,_)|(0xF,0x5,_,_)|(0xF,0x6,_,_)|(0xF,0x7,_,_)=>{
                    let bit_set = (data[i] & 0x0007) as u8;
                    let offset = ((data[i] & 0x03F8)>>2) as i8 >> 1;
                    r[i] = (CPU::brbc,BR(bit_set.into(), offset));
                    #[cfg(feature = "std")]
                    println!("cpc bit:{:x}, offset:{:x}, i:{:x}",bit_set, offset ,i);
                }
                (0x9,0x4,0x0,0x8)|(0x9,0x4,0x1,0x8)|(0x9,0x4,0x2,0x8)|(0x9,0x4,0x3,0x8)|(0x9,0x4,0x4,0x8)|(0x9,0x4,0x5,0x8)|(0x9,0x4,0x6,0x8)|(0x9,0x4,0x7,0x8) => {
                    let bit_set = ((data[i] & 0x0070) >> 4) as u8;
                    r[i] = ();

                }

                (_,_,_,_) => {
                    r[i] = (CPU::halt,InstructionData::NILL);
                    #[cfg(feature = "std")]
                    println!("halt i:{:x}", i);
                }
            }
            i+=1;
        }
        r
    }
}
