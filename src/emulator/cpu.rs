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
const JR_NZ : u8 = 0x20;
const LD_HL_NN : u8 = 0x21;
const LDRR_AH : u8 = 0x7c;
const LD_HL_DA : u8 = 0x32;

impl<'a> CPU<'a> {
  pub fn exec(&mut self) {
      
      let instr = self.mmu.rb(self.pc);
      self.view_position_count();

      self.pc += 1;

      let panic_and_die = || -> () { print!("unimplemented opcode 0x{:x?} {:x?} {:x?}, send help!\r\n", instr, self.mmu.rb(self.pc+1), self.mmu.rb(self.pc+2)); std::process::exit(0); };

      match instr {
        LD_SP => { self.sp = self.mmu.rw(self.pc); self.pc += 2; self.m = 3; self.debug("LD_SP"); },
        XOR_A => { self.a ^= self.a; self.m = 1; self.debug("XOR_A"); },
        LD_HL_DA => { self.mmu.wb((((self.h as u16) << 8) as u16)+(self.l as u16), self.a); self.m=2; self.pc += 2; self.debug("LD_HL_DA"); },
        LD_HL_NN => { self.l=self.mmu.rb(self.pc);self.h=self.mmu.rb(self.pc+1); self.pc+=2; self.m=3; self.debug("LD_HL_NN") }
        LDRR_AH => { self.a = self.h; self.m = 1; self.debug("LDRR_AH"); }
        JR_NZ => {
          self.debug("JR_NZ");
          if (self.flags.zero == true) {
            self.pc = self.mmu.rb(self.pc as u16) as u16;
          } else {
            self.pc += 1;
         }
         }
        code => { panic_and_die() }
      }

      self.view_registers();

      self.clock_m += self.m as u32;
  }

  fn view_position_count(&mut self) {
    print!("[{}] ", self.pc);
  }

  fn view_registers(&mut self) {
    print!("CPU: [a: {}] [b: {}] [c: {}] [d: {}], [e: {}], [h: {}], [l: {}], [f: {}], [pc: {}], [sp: {}]\r\n", self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.f, self.pc, self.sp)
  }

  fn debug(&self, command : &str) {
    print!("{}\t\t", command);
  }
}

pub struct CPU<'a> {
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
    pub flags : Flags,
    pub mmu : &'a mut super::mmu::MMU<'a>
}