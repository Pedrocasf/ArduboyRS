use core::ops::{Index, IndexMut};
pub struct IOs{

}
impl IOs {
    pub fn new() -> IOs {
        IOs{}
    }
}
impl Index<u16> for IOs {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        panic!("io {:x?} read accessed", index);
    }
}

impl IndexMut<u16> for IOs {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        panic!("io {:x?} write accessed", index);
    }
}
