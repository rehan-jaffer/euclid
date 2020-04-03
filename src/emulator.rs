#![feature(get_mut_unchecked)]

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
pub mod cpu;
pub mod mmu;

impl Emulator {

    pub fn load(&mut self, filename: &str) {

      let f = File::open(&filename).expect("bios file not found");
      let mut rom = Vec::new();
      
      for byte in f.bytes() {
        rom.push(byte.unwrap())
      }

      self.mmu.load(rom)
    }
  
    pub fn loadBIOS(&mut self) {
      let f = File::open("bios.bin").expect("bios file not found");
      let mut bios = Vec::new();
      
      for byte in f.bytes() {
        bios.push(byte.unwrap())
      }

      self.mmu.load_bios(bios);
  }
  
    pub fn boot(&mut self) {
      while (self.running == true) {
        self.cpu.exec(&mut self.mmu)
      }
    }
  }
  
pub struct Emulator {
      pub cpu: cpu::CPU,
      pub mmu: mmu::MMU,
      pub running : bool
}