// registers A-F are 8-bit
// registers PC and SP are 16-bit
impl CPU {
}

#[derive(Default)]
pub struct CPU {
    ra: u8,
    rb: u8,
    rc: u8,
    rd: u8,
    re: u8,
    rh: u8,
    rl: u8,
    rf: u8,
    pc: u16,
    sp: u16,
    clock_m: u32,
    clock_t: u32
}