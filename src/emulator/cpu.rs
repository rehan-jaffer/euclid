use std::{thread, time};

/* 
* CPU Flags 
* Made the decision to use a struct with separate flags rather
* than fiddling with an F register using bitwise ops. Time will tell whether this was a good idea.
* Zero Flag (Z) - Set to 1 when the result of a math operation is zero or two values match when using the CP instruction, else zero
* Subtract Flag (N) - Set to 1 if a subtraction was performed in the last math instruction
* Half Carry Flag (H) - Set to 1 if a carry occured from the lower nibble in the last math operation
* Carry Flag (C) - Set to 1 if a carry occured from the last math operation or if A is the smaller value when using the CP instruction
*/

#[derive(Default)]
pub struct Flags {
  pub zero : bool,
  pub negative : bool,
  pub h : bool,
  pub carry : bool
}

/* Flags for debugging */
const SLOW_MODE : bool = false;

const NOP : u8 = 0x0;

const LD_BC_NN : u8 = 0x1;
const INC_C : u8 = 0xc;
const LD_C_BYTE : u8 = 0xe;
const LD_B_BYTE : u8 = 0x6;

const RLr_c : u8 = 0x11;
const INC_DE : u8 = 0x13;
const RLA : u8 = 0x17;

const JR_NZ : u8 = 0x20;
const LD_HL_NN : u8 = 0x21;
const LD_HL_IA : u8 = 0x22;
const INC_HL : u8 = 0x23;

const LDRR_AH : u8 = 0x7c;
const LD_SP : u8 = 0x31;
const LD_HL_DA : u8 = 0x32;

const LD_A_BYTE : u8 = 0x3e;
const LD_IO_nA : u8 = 0xe0;
const LD_IO_CA : u8 = 0xe2;

const LD_HL_A : u8 = 0x77;
const LDrr_ba : u8 = 0x47;
const INC_B : u8 = 0x4;
const LD_ADE_m : u8 = 0x1a;

const POP_BC : u8 = 0xc1;
const PUSH_BC : u8 = 0xc5;
const RET : u8 = 0xc9;
const CALL_NN : u8 = 0xCD;

const DEC_B : u8 = 0x5;
const LD_A_E : u8 = 0x7b;
const CP_N : u8 = 0xfe;
const XOR_A : u8 = 0xAF;

/* 
*  CB Instructions
*  The following instructions have a prefix of 0xCB 
*/
const CB_PREFIX : u8 = 0xCB;
const LD_DE_NN : u8 = 0x11;
const BIT_7_H : u8 = 0x7c;

impl<'a> CPU<'a> {
  pub fn exec(&mut self) {

      
      let ten_millis = time::Duration::from_millis(5);
      let now = time::Instant::now();
      
      if (SLOW_MODE) {
        thread::sleep(ten_millis);
      }

      let instr = self.mmu.rb(self.pc);
      self.view_position_count();

      let panic_and_die = || -> () {  };

      match instr {
        LD_SP => { self.sp = self.mmu.rw(self.pc); self.pc += 2; self.m = 3; self.debug("LD_SP"); },
        LD_C_BYTE => { self.c = self.mmu.rb(self.pc+1); self.pc += 1; self.m = 2; self.debug("LD_C_NN"); },
        LD_A_BYTE => { self.a = self.mmu.rb(self.pc+1); self.pc += 1; self.m = 2; self.debug("LD_A_NN"); },
        LD_B_BYTE => { self.b = self.mmu.rb(self.pc+1); self.pc += 1; self.m = 2; self.debug("LD_B_NN"); },
        LD_IO_CA => { self.mmu.wb((0xFF00+self.c as u16) as u16, self.a); self.m = 2; self.debug("LD_IO_CA") },
        LD_IO_nA => { self.mmu.wb((0xFF00+self.mmu.rb(self.pc+1) as u16) as u16, self.a); self.m = 2; self.pc += 1; self.debug("LD_IO_nA") },
        LDrr_ba => { self.b = self.a; self.m += 1; self.debug("LDrr_ba"); },
        LD_BC_NN => {  self.c = self.mmu.rb(self.pc+1); self.b = self.mmu.rb(self.pc + 2); self.pc += 2; self.debug("LD_DE_NN") },
        LD_DE_NN => {  self.e = self.mmu.rb(self.pc+1); self.d = self.mmu.rb(self.pc + 2); self.pc += 2; self.debug("LD_DE_NN") },
        INC_C => { self.c += 1; self.debug("INC_C"); },
        XOR_A => { self.a ^= self.a; self.m = 1; self.debug("XOR_A"); },
        LD_ADE_m => { self.a=self.mmu.rb(((self.d as u16)<<8)+self.e as u16); self.m=2; self.debug("LD_ADE_m") },
        LD_HL_A => { self.mmu.wb((((self.h as u16) << 8) as u16)+(self.l as u16), self.a); self.m=2; self.pc += 2; self.debug("LD_HL_A"); },
        INC_B => { self.b += 1; self.b &= 255; if (self.b == 0) { self.flags.zero = true; } self.debug("INC B") },
        CALL_NN => {
          self.sp-=2; 
          self.mmu.ww(self.sp,self.pc+2); 
          self.pc = self.mmu.rw(self.pc+1); 
          self.m=5;
          self.debug("CALL_NN");
        },
        RET => {
          self.pc = self.mmu.rw(self.sp);
          self.sp += 2;
          self.m = 3;
          self.debug("RET");
        },
        LD_HL_DA => { 
          self.mmu.wb((((self.h as u16) << 8) as u16)+(self.l as u16), self.a); self.m=2;  self.debug("LD_HL_DA"); 
          self.l = if self.l == 0 { 255 } else { (self.l-1) };
          if self.l == 255  { 
            self.h=(self.h-1) & 255;
          }
          self.debug("LD_HL_DA");
        },
        LD_HL_NN => { self.l=self.mmu.rb(self.pc+1);self.h=self.mmu.rb(self.pc+2); self.pc+=2; self.m=3; self.debug("LD_HL_NN") }
        LDRR_AH => { self.a = self.h; self.m = 1; self.debug("LDRR_AH"); }
        JR_NZ => {
          self.debug("JR_NZ");
          if (self.flags.zero == false) {
            let target = self.mmu.rb(self.pc+1) as i8;
            let pc = self.pc as i16;
            let pc_with_offset = (pc + 1 + target as i16) as u16;
            self.set_pc(pc_with_offset);
          } else {
            self.set_pc(self.pc+1);
         }
        },
        PUSH_BC => {
          self.sp -= 1; 
          self.mmu.wb(self.sp, self.b); 
          self.sp -= 1; 
          self.mmu.wb(self.sp,self.c); 
          self.m=3;
          print!("=> ");
          self.debug("PUSH_BC");
        },
        POP_BC => {
          self.c=self.mmu.rb(self.sp); 
          self.sp += 1; 
          self.b=self.mmu.rb(self.sp); 
          self.sp += 1; 
          self.m=3;
          print!("<= ");
          self.debug("POP_BC");
        }
        RLA => {
          let new_carry = if (self.a & 0x80 == 0) { false } else { true };
          self.a=(self.a<<1) + (self.flags.carry as u8); self.a &= 255; 
          if (self.a == 0) { self.flags.zero = true; } else { self.flags.zero = false; }
          self.flags.carry = new_carry;
          self.m=2; 
          self.debug("RLr_c");
        },
        DEC_B => {
          self.b -= 1;
          self.b &= 255;
          self.flags.zero = (self.b == 0);
          self.m=1;
          self.debug("DEC_C")
        },
        LD_HL_IA => {
          self.mmu.wb(((self.h as u16)<<8) + (self.l as u16), self.a); 
          let (n, _) = self.l.overflowing_add(1);
          self.l = n;
          if(self.l == 0) {
            let (n, _) = self.h.overflowing_add(1); 
            self.h = n;
            self.m = 2;
          }
          self.pc += 2;
          self.debug("LD_HL_IA");
        },
        INC_HL => {
          let (n, _) = self.l.overflowing_add(1);
          self.l = n;
          if (self.l == 0) {
            let (n, _) = self.h.overflowing_add(1); 
            self.h = n;
          }
          self.m=1;
          self.debug("INC_HL");
        },
        INC_DE => {
          let (n, _) = self.e.overflowing_add(1);
          self.e = n;
          if (self.e == 0) {
            let (n, _) = self.d.overflowing_add(1);
            self.d = n;
          }
          self.m = 1;
          self.debug("INC_DE");
        },
        LD_A_E => {
          self.a = self.e;
          self.m = 1;
          self.debug("LD_A_E");
        },
        CP_N => {
          let i = self.a;
          let m = self.mmu.rb(self.pc+1);
          let (i, _) = i.overflowing_sub(1);
          self.pc += 1;
          self.flags.negative = (i < 0);
          self.flags.zero = (i == 0);
          self.flags.carry = ((self.a ^ i ^ m) & 0x10) != 0;
          self.m = 2;
          self.debug("CP_N");
        }
        CB_PREFIX => {
          match self.mmu.rb(self.pc+1) {
            BIT_7_H => {
              self.flags.zero = (self.h & 0x80) == 0;     
              self.debug("BIT_7_H");
              self.pc += 1;
            },
            RLr_c => { 
              let new_carry = if (self.c & 0x80 == 0) { false } else { true };
              self.c=(self.c<<1) + (self.flags.carry as u8); self.c&=255; 
              if (self.c == 0) { self.flags.zero = true; } else { self.flags.zero = false; }
              self.flags.carry = new_carry;
              self.m=2; 
              self.debug("RLr_c");
              self.pc += 1;
            }
            _ => {
              self.panic_and_die(instr);
            }
          }
      
        }
        code => { self.panic_and_die(instr) }
      }

      self.pc += 1;

      self.view_registers();

      self.clock_m += self.m as u32;
  }

  fn panic_and_die(&mut self, instr : u8) {
    self.mmu.gpu.dump_vram();
    print!("unimplemented opcode 0x{:x?} {:x?} {:x?}, send help!\r\n", instr, self.mmu.rb(self.pc+1), self.mmu.rb(self.pc+2)); std::process::exit(0);
  }

  fn set_pc(&mut self, pc : u16) {
    self.pc = pc;
  }

  fn view_position_count(&mut self) {
    print!("[{:X?}] ", self.pc);
  }

  fn view_registers(&mut self) {
   print!("CPU: [a: {}] [b: {}] [c: {}] [d: {}], [e: {}], [h: {}], [l: {}], [f: {}], [pc: {}], [sp: {}]\r\n", self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.f, self.pc, self.sp)
  }

  fn debug(&self, command : &str) {
    self.mmu.show_stack(self.sp);
    print!("{}\t\t", command);
  }
}

/* 
* Gameboy's CPU is similar to the Zilog Z80 with some minor additions and subtractions. 
* Clockspeed 4.194 Mhz
* Registers A, B, C, D, E, H & L are 8-bit.
* Registers PC (Program Counter) and SP (Stack Pointer) are 16bit
*/

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