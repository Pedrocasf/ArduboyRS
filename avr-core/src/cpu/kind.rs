pub struct AVRKind{
    pub flash_size:u16,
    pub sram_size:u16,
    pub fuses:[u8;4],
    pub ios_size:u16,
    pub exios_size:u16
}
const ATtiny13A: AVRKind = AVRKind{
    flash_size:0x0200,
    sram_size:0x040,
    fuses:[0x7A, 0xFF, 0x00, 0xFF],
    ios_size:64,
    exios_size:0
};
pub const AVR_TYPE:AVRKind = ATtiny13A;
/*macro_rules! AVRType {
    ($val:literal) => {};
}*/