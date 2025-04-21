use crate::cpu::CPU;
use crate::cpu::kind::AVR_TYPE;
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Instructions{
    HALT,
}
impl Instructions{
    pub fn run(&mut self, cpu: CPU){
        match self{
            Self::HALT => panic!("Halting at addr {:x?}", cpu.pc),
        }
    }
}
