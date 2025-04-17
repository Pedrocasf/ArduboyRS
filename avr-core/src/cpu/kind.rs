pub struct AVRKind{
    pub flash_size:usize,
    pub sram_size:usize,
    pub fuses:[u8;4],
    pub ios:usize,
    pub exios:Option<usize>
}
pub const ATtiny13A: AVRKind = AVRKind{
    flash_size:0x0400,
    sram_size:0x040,
    fuses:[0x7A, 0xFF, 0x00, 0xFF],
    ios:64,
    exios:None
};