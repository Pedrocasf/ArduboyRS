use crate::cpu::instructions::Instruction;
pub const I:u8 = 0b10000000;
pub const T:u8 = 0b01000000;
pub const H:u8 = 0b00100000;
pub const S:u8 = 0b00010000;
pub const V:u8 = 0b00001000;
pub const N:u8 = 0b00000100;
pub const Z:u8 = 0b00000010;
pub const C:u8 = 0b00000001;
pub const HSVNZC:u8 = 0b00111111;
pub const ZC: u8 = 0b00000011;
#[repr(u8)]
#[derive(Clone,Copy)]
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
impl From<u8> for Flag{
    fn from(flag:u8) -> Self{
        match flag {
            0 => Flag::C,
            1 => Flag::Z,
            2 => Flag::N,
            3 => Flag::V,
            4 => Flag::S,
            5 => Flag::H,
            6 => Flag::T,
            7 => Flag::I,
            _=> unreachable!()
        }
    }
}
#[derive(Clone,Copy)]
pub struct LazyFlags{
    pub op1: u8,
    pub op2: u8,
    pub res: i16,
}

impl LazyFlags{
    pub fn new() -> LazyFlags{
        LazyFlags{
            op1:0,
            op2:0,
            res:0,
        }
    }
    pub fn calc_flag(&mut self, flag:Flag)->bool{
        match flag{
            Flag::C => {
                let rd7 = self.op1 & 0x80 == 0x80;
                let rr7 = self.op2 & 0x80 == 0x80;
                let r7 = self.res & 0x80 == 0x80;
                (rd7 & rr7) | (rr7 & !r7) | (!r7 & rd7)
            }
            Flag::Z => {
                self.res == 0
            }
            Flag::N => {
                self.res & 0x80 == 0x80
            }
            Flag::V => {
                let rd7 = self.op1 & 0x80 == 0x80;
                let rr7 = self.op2 & 0x80 == 0x80;
                let r7 = self.res & 0x80 == 0x80;
                (rd7 & rr7 & !r7) | ( !rd7 & !rr7 & r7)
            }
            Flag::S => {
                let rd7 = self.op1 & 0x80 == 0x80;
                let rr7 = self.op2 & 0x80 == 0x80;
                let r7 = self.res & 0x80 == 0x80;
                (rd7 & rr7 & !r7) | ( !rd7 & !rr7 & r7) ^ (self.res & 0x80 == 0x80)
            }
            Flag::H => {
                let rd3 = self.op1 & 0x08 == 0x08;
                let rr3 = self.op2 & 0x08 == 0x08;
                let r3 = self.res & 0x08 == 0x08;
                (rd3 & rr3 & !r3) | ( !rd3 & !rr3 & r3)
            }
            _=> unreachable!()
        }
    }
}