use std::io;
use std::io::prelude::*;
use std::fs::File;

impl Emulator {
  fn load(&mut self, filename: &str) {
    let mut f = File::open(&filename).expect("no file found");
    f.read(&mut self.rom).expect("buffer overflow");  }
}

struct Emulator {
    rom: Vec<u8>
}

fn main() {
    let mut emulator = Emulator { rom: vec![] };
    emulator.load(&"cpu_instrs.gb");
}
