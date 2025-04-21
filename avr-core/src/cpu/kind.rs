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
const ATMega32u4: AVRKind = AVRKind{
    flash_size: 0x4000,
    sram_size: 0x0A00,
    fuses:[0xFF,0xD2,0xCB, 0x3F],
    ios_size:0x40,
    exios_size:0xA0
};
pub const AVR_TYPE:AVRKind = ATMega32u4;
/*macro_rules! AVRType {
    ($val:literal) => {};
}*/