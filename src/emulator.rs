#![feature(get_mut_unchecked)]

extern crate sdl2;
extern crate gl;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
pub mod cpu;
pub mod mmu;
pub mod gpu;

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


      let sdl = sdl2::init().unwrap();
      let video_subsystem = sdl.video().unwrap();
      let window = video_subsystem
          .window("Game", 900, 700)
          .opengl()
          .resizable()
          .build()
          .unwrap();

      let gl_attr = video_subsystem.gl_attr();

      gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
      gl_attr.set_context_version(4, 5);
                    
      let gl_context = window.gl_create_context().unwrap();
      let mut event_pump = sdl.event_pump().unwrap();
      let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

      unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
      }

      while (self.running == true) {
        for _event in event_pump.poll_iter() {
            // handle user input here
        }
        self.cpu.exec()
      }
    }
  }
  
pub struct Emulator<'a> {
      pub cpu: &'a mut cpu::CPU<'a>,
      pub running : bool
}