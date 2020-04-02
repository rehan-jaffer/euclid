const BITMASK : u16 = 0xF000;

/*
 Gameboy memory layout:
  
*/

impl MMU {
    pub fn rb(&self, addr : u16) -> u8 { 
      
      // mask the address and find the correct memory mapped region to read from
      match (addr & BITMASK) {
        0x0000 => {
          if (self.booting) {
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
        _ => { return 0 }
      }

    }

    pub fn rw(&self, addr : u16) -> u16 {
      let word : u16 = self.rb(addr) as u16  + ((self.rb(addr+1) as u16) << 8) as u16;
      return word;
    }

    pub fn load(&mut self, bytes: Vec<u8>) {
      self.rom = bytes.clone();
    }

    pub fn load_bios(&mut self, bios: Vec<u8>) {
      self.bios = bios.clone();
    }
  }
  
pub struct MMU {
    pub bios: Vec<u8>,
    pub rom: Vec<u8>,
    pub eram: Vec<u8>,
    pub wram: Vec<u8>,
    pub zram: Vec<u8>,
    pub booting: bool
}