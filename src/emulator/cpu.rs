use std::cell::RefCell;
use std::rc::Rc;

// registers A-F are 8-bit
// registers PC and SP are 16-bit

pub struct Flags {
  pub zero : bool,
  pub negative : bool,
  pub h : bool,
  pub c : bool
}

const NOP : u8 = 0x0;
const LD_SP : u8 = 0x31;
const XOR_A : u8 = 0xAF;
const JR_NZ : u8 = 0x21;
const LD_HL_DA : u8 = 0x32;
const LDRR_AH : u8 = 0x7c;

impl CPU {
  pub fn exec(&mut self, mmu : &mut super::mmu::MMU) {
      
      let instr = mmu.rb(self.pc);
      self.pc += 1;

      let panic_and_die = || -> () { print!("unimplemented opcode 0x{:x?} {:x?} {:x?}, send help!\r\n", instr, mmu.rb(self.pc+1), mmu.rb(self.pc+2)); std::process::exit(0); };

      match instr {
        LD_SP => { self.sp = mmu.rw(self.pc); self.pc += 2; self.m = 3; },
        XOR_A => { self.a ^= self.a; self.m = 1; },
        LD_HL_DA => { mmu.wb((((self.h as u16) << 8) as u16)+(self.l as u16), self.a); self.m=2; },
        LDRR_AH => { self.a = self.h; self.m = 1; }
        JR_NZ => {
          if (self.flags.zero == true) {
            self.pc = mmu.rb(self.pc as u16) as u16;
          } else {
            self.pc += 1;
          }
         }
         CB_PREFIX => {
          print!("");
          match mmu.rb(self.pc+1) {
            _ => panic_and_die()
          }
         }
        code => { panic_and_die() }
      }

      self.clock_m += self.m as u32;
      self.view_registers();
  }

  fn view_registers(&mut self) {
    print!("CPU: a: {} b: {} c: {} d: {}, e: {}, h: {}, l: {}, f: {}, pc: {}, sp: {}\n", self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.f, self.pc, self.sp)
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
    pub m : u8,
    pub pc: u16,
    pub sp: u16,
    pub clock_m: u32,
    pub clock_t: u32,
    pub flags : Flags
}

impl Default for CPU {
  fn default() -> CPU { CPU { a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, l: 0, pc: 0, sp: 0, clock_m: 0, clock_t: 0, h: 0, m: 0, flags: Flags { zero: false, negative: false, h: false, c: false } } }
}