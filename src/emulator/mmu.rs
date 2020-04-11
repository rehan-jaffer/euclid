const BITMASK : u16 = 0xF000;
use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Style;

/*
*  MMU Memory Map
*
* [Cartridge is loaded here]
* [0x0000->0x4000] 16kb ROM Bank #0
* [0x4000->0x8000] 16kb switchable ROM Bank
*
* [0x8000->0xA000] 8kb switchable RAM bank
* [0xA000->0xC000] 8kb Internal RAM
* [0xC000->0xE000] Sprite Attribute Memory (OAM)
* [0xE000->0xFE00] Empty but usable for I/O
* [0xFE00->0xFF4C] I/O Ports
* [0xFF4C->0xFF80] Empty but usable for I/O
* [0xFF80->0xFFFF] Internal RAM (stack memory etc.)
*/

impl<'a> MMU<'a> {
    pub fn rb(&self, addr : u16) -> u8 { 

      // mask the address and find the correct memory mapped region to read from
      match (addr & BITMASK) {
        0x0000 => {
          if (addr < 256) {
            return self.bios[addr as usize];
          }
          return self.rom[addr as usize];
        },
        0x1000 | 0x2000 | 0x3000 => {
            return self.rom[addr as usize];
        },
	    0x4000 | 0x5000 | 0x6000 | 0x7000 => {
            return self.rom[addr as usize];
        },
        _ => { return self.wram[addr as usize] }
      }

    }

    pub fn show_stack(&self, stack_pointer : u16) {
      if !self.stack_debugger_enabled { return () }

      print!("[{:x?}] \t[ ", stack_pointer);
      for i in (0..16) {
        let addr = 65073-i;
        if (addr == stack_pointer) {
          print!("{}  ", Style::new().bold().paint(format!("*0x{:x?}*", self.wram[addr as usize])));
        } else {
          print!("{}  ", Green.paint(format!("0x{:x?}", self.wram[addr as usize])));        }
      }
      print!("] \r\n");
    }

    pub fn rw(&self, addr : u16) -> u16 {
      let word : u16 = ((self.rb(addr+1) as u16) << 8) + self.rb(addr) as u16;
      return word;
    }

    pub fn wb(&mut self, addr : u16, value : u8) {

        // mask the address and find the correct memory mapped region to read from
        match (addr & BITMASK) {
          0x0000 => {
            if ((addr & BITMASK) < 256) {
                self.bios[addr as usize] = value;
            }
            self.rom[addr as usize] = value;
          },
          0x1000 | 0x2000 | 0x3000 => {
              self.rom[addr as usize] = value;
          },
          0x4000 | 0x5000 | 0x6000 | 0x7000 => {
              self.rom[addr as usize] = value;
          },
          0x8000 => {
            self.gpu.vram[(addr & 0x1FFF) as usize] = value;
          }
          _ => { self.wram[addr as usize] = value; }
        }      
        return;
    }

    pub fn ww(&mut self, addr : u16, word : u16) {
      let bytes = word.to_be_bytes();
      self.wb(addr+1, bytes[0]);
      self.wb(addr, bytes[1]);
    }

    pub fn load(&mut self, bytes: Vec<u8>) {
      self.rom = bytes.clone();
    }

    pub fn load_bios(&mut self, bios: Vec<u8>) {
      self.bios = bios.clone();
    }
  }

pub struct MMU<'a> {
    pub bios: Vec<u8>,
    pub rom: Vec<u8>,
    pub eram: Vec<u8>,
    pub wram: Vec<u8>,
    pub zram: Vec<u8>,
    pub booting: bool,
    pub gpu: &'a mut super::gpu::GPU,
    pub stack_debugger_enabled: bool
  }