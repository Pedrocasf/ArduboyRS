use crate::cpu::CPU;
use crate::cpu::kind::AVR_TYPE;
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstructionData{
    pub k:u16,
    pub r:u8,
    pub d:u8,
}