use std::io;
use std::io::prelude::*;
use std::fs::File;

mod cpu;
mod mmu;

impl Emulator {
  fn load(&mut self, filename: &str) {
    let mut f = File::open(&filename).expect("no file found");
    let mut rom : Vec<u8> = Vec::new();
    f.read(&mut rom).expect("buffer overflow");
    &self.mmu.load(rom);
  }

  fn boot() {

  }
}

#[derive(Default)]
struct Emulator {
    cpu: cpu::CPU,
    mmu: mmu::MMU
}

fn main() {
    let mut emulator : Emulator = Default::default();
    emulator.load(&"cpu_instrs.gb");
}