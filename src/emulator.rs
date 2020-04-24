#![feature(get_mut_unchecked)]

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use std::time::{Instant};
use wasm_bindgen::prelude::*;

pub mod cpu;
pub mod mmu;
pub mod gpu;

#[wasm_bindgen(module = "/browser.js")]
extern "C" {
    fn postState(line: &str);
}

pub struct EmulatorState {
  a: u8
}

impl<'a> Emulator<'a> {

    pub fn load(&mut self, filename: &str) {

//      let f = File::open(&filename).expect("bios file not found");
 //     let mut rom = Vec::new();
      
//      for byte in f.bytes() {
//        rom.push(byte.unwrap())
//      }

      self.cpu.mmu.load()
    }
  
    pub fn loadBIOS(&mut self) {
/*      let f = File::open("bios.bin").expect("bios file not found");
      let mut bios = Vec::new();
      
      for byte in f.bytes() {
        bios.push(byte.unwrap())
      }
*/
      self.cpu.mmu.load_bios();
  }
  
    pub fn boot(&mut self) {
      print!("* Starting Euclid (A Gameboy emulator in Rust)\r\n");
      print!("* Booting BIOS...\r\n");

    /*  let mut window: PistonWindow =
      WindowSettings::new("Hello Piston!", [256, 164])
      .exit_on_esc(true).build().unwrap();*/

      let mut pixels = Vec::new();
      for i in (0..(256*164)) {
        if (i % 7) == 0 {
          pixels.push(1);
        } else {
          pixels.push(0);
        }
      }

  //    let mut screen = Screen { window: &mut window, width: 246, height: 164, pixels: pixels.clone() };
//      let time = Instant::now();

        while (self.running == true) {
          self.cpu.exec();
        }
    }

    pub fn step(&mut self) -> EmulatorState {
      return EmulatorState { a: self.cpu.a };
    }
  }
  
pub struct Emulator<'a> {
      pub cpu: &'a mut cpu::CPU<'a>,
      pub running : bool
}