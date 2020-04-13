#![feature(get_mut_unchecked)]

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, SystemTime};
use std::time::{Instant};
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
pub mod cpu;
pub mod mmu;
pub mod gpu;

extern crate piston_window;

use piston_window::*;

impl<'a> Emulator<'a> {

    pub fn load(&mut self, filename: &str) {

      let f = File::open(&filename).expect("bios file not found");
      let mut rom = Vec::new();
      
      for byte in f.bytes() {
        rom.push(byte.unwrap())
      }

      self.cpu.mmu.load(rom)
    }
  
    pub fn loadBIOS(&mut self) {
      let f = File::open("bios.bin").expect("bios file not found");
      let mut bios = Vec::new();
      
      for byte in f.bytes() {
        bios.push(byte.unwrap())
      }

      self.cpu.mmu.load_bios(bios);
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
      let time = Instant::now();

      while (self.running == true) {
        self.cpu.exec();
      }
    }
  }
  
pub struct Emulator<'a> {
      pub cpu: &'a mut cpu::CPU<'a>,
      pub running : bool
}

pub struct Screen<'a> {
  width: u32,
  height: u32,
  pixels: Vec<u8>,
  window: &'a mut PistonWindow
}

impl<'a> Screen<'a> {
 

  fn draw(&mut self) {
    let width = self.width;
    let height = self.height;
    let pixels = &mut self.pixels;
    let e = self.window.next().unwrap();
    self.window.draw_2d(&e, |context, graphics, _device| {
      clear([1.0; 4], graphics);
      for x in (0..width-1) {
        for y in (0..height-1) {
          let val = pixels[(((x*height)+width)-1) as usize] as f32;
          rectangle([val, val, val, 1.0], // red
            [x as f64, y as f64, (x+1) as f64, (y+1) as f64],
            context.transform,
            graphics);
        }
      }
    });
  }

}