use std::io;
use std::io::prelude::*;
use std::fs::File;


// registers A-F are 8-bit
// registers PC and SP are 16-bit
impl CPU {
}

#[derive(Default)]
struct CPU {
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

impl MMU {
  fn rb() { }
  fn rw() {}
}

#[derive(Default)]
struct MMU {
    bios: Vec<u8>,
    rom: Vec<u8>,
    eram: Vec<u8>,
    wram: Vec<u8>,
    zram: Vec<u8>
}

impl Emulator {
  fn load(&mut self, filename: &str) {
    let mut f = File::open(&filename).expect("no file found");
    f.read(&mut self.rom).expect("buffer overflow");  
  }

  fn boot() {

  }
}

#[derive(Default)]
struct Emulator {
    rom: Vec<u8>,
    cpu: CPU
}

fn main() {
    let mut emulator : Emulator = Default::default();
    emulator.load(&"cpu_instrs.gb");
}
