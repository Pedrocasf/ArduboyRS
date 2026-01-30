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
}
#[derive(Clone,Copy)]
pub enum Instruction{
    RJMP,
    EOR,
    MULS,
    LDI,
    CPI,
    CPC,
    NOP,
}