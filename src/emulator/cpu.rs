use std::cell::RefCell;
use std::rc::Rc;

// registers A-F are 8-bit
// registers PC and SP are 16-bit

impl CPU {
  pub fn exec(&mut self, mmu : &super::mmu::MMU) {
      let instr = mmu.rb(self.pc);
      match instr {
        0x31 => { self.sp = mmu.rw(self.pc+1); self.pc += 3; }
        _ => { panic!("unimplemented opcode, send help!")}
      }
  }
}

pub struct CPU {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub pc: u16,
    pub sp: u16,
    pub clock_m: u32,
    pub clock_t: u32
}

impl Default for CPU {
  fn default() -> CPU { CPU { a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, l: 0, pc: 0, sp: 0, clock_m: 0, clock_t: 0, h: 0 } }
}