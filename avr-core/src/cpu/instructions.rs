use crate::cpu::CPU;
use crate::cpu::kind::AVR_TYPE;
use crate::cpu::lazy_flags::*;
#[derive(Clone,Copy)]
pub enum InstructionData{
    NILL,
    K(u16),
    DR(u8,u8),
    SK(i16),
    BR(Flag, i8),
    BIT(u8),
    DS(u8, u16)
}
#[repr(u8)]
#[derive(Clone,Copy)]
pub enum Instruction{
    HALT,
    RJMP,
    RETI,
    CPSE,
    EOR,
    MULS,
    LDI,
    CPI,
    CPC,
    BRBC,
    BSETR,
    OUT,
    JMP,
    NOP,
    RCALL,
    LDS,
    OR,
    MOVW,
    ADD,
}