use core::ops::{Index, IndexMut};

pub enum Flag{
    C,
    Z,
    N,
    V,
    S,
    H,
    T,
    I
}
pub struct Sreg{
    flags:[bool;8]
}
impl Sreg{
    pub fn new() -> Self{
        Sreg{
            flags:[false;8]
        }
    }
}
impl Index<Flag> for Sreg{
    type Output = bool;
    fn index(&self, idx: Flag) -> &bool{
        &self.flags[idx as usize]
    }
}
impl Index<u8> for Sreg{
    type Output = bool;
    fn index(&self, idx:u8) -> &bool{
        if idx < 8{
           return &self.flags[idx as usize];
        }
        panic!("Index out of bounds");

    }
}
impl IndexMut<Flag> for Sreg{
    fn index_mut(&mut self, idx: Flag) -> &bool{
        &mut self.flags[idx as usize]
    }
}