use crate::cpu::CPU;
use crate::cpu::kind::AVR_TYPE;
#[derive(Clone,Copy)]
pub enum InstructionData{
    NILL,
    K(u16),
    DR(u8,u8),
    SK(i16)
}