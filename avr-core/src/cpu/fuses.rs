use core::ops::Index;
pub enum Fuse{
    L,
    H,
    E,
    K
}

pub struct Fuses{
    fuses:[u8;4]
}
impl Fuses{
    pub const fn new(f:[u8;4])->Fuses{
        Fuses{
            fuses:f
        }
    }
}
impl Index<Fuse> for Fuses{
    type Output = u8;
    fn index(&self, idx: Fuse) -> &u8{
        &self.fuses[idx as usize]
    }
}