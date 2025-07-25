use core::ops::{Index, IndexMut};
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct Sreg(pub u8);
impl Sreg{
    pub fn new() -> Self{
        Sreg(0)
    }
    fn sets(&mut self,val:bool,shift:u8){
        let val = (val as u8) << shift;
        let mask = 1 << shift;
        self.0&=!mask;
        self.0|=val;
    }
    pub fn set_i(&mut self,val:bool){
        self.sets(val,7);
    }
    pub fn set_t(&mut self,val:bool){
        self.sets(val,6);
    }
    pub fn set_h(&mut self,val:bool){
        self.sets(val,5);
    }
    pub fn set_s(&mut self,val:bool){
        self.sets(val,4);
    }
    pub fn set_v(&mut self,val:bool){
        self.sets(val,3);
    }
    pub fn set_n(&mut self,val:bool){
        self.sets(val,2);
    }
    pub fn set_z(&mut self,val:bool){
        self.sets(val,1);
    }
    pub fn set_c(&mut self,val:bool){
        self.sets(val,0);
    }

    fn gets(self,shift:u8) ->bool{
        let val = self.0 >> shift;
        val&1 == 1
    }
    pub fn get_i(&self) -> bool{
        self.gets(7)
    }
    pub fn get_t(&self) -> bool{
        self.gets(6)
    }
    pub fn get_h(&self) -> bool{
        self.gets(5)
    }
    pub fn get_s(&self) -> bool{
        self.gets(4)
    }
    pub fn get_v(&self) -> bool{
        self.gets(3)
    }
    pub fn get_n(&self) -> bool{
        self.gets(2)
    }
    pub fn get_z(&self) -> bool{
        self.gets(1)
    }
    pub fn get_c(&self) -> bool{
        self.gets(0)
    }
}