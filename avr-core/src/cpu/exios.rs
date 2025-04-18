use std::ops::{Index, IndexMut};
use crate::cpu::sreg::Sreg;
#[derive(Copy, Clone)]
pub struct ExIOs{
    sreg: Sreg,
}
impl ExIOs {
    pub fn new() -> ExIOs {
        ExIOs {
            sreg: Sreg::new(),
        }
    }
}
impl Index<u16> for ExIOs {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        panic!("exio {:x?} read accessed", index);
    }
}

impl IndexMut<u16> for ExIOs {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        panic!("exio {:x?} write accessed", index);
    }
}
