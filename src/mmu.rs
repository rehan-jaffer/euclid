impl MMU {
    fn rb() { }
    fn rw() {}
    pub fn load(&mut self, bytes: Vec<u8>) {
      self.rom = bytes.clone();
    }
  }
  
#[derive(Default)]
pub struct MMU {
    bios: Vec<u8>,
    rom: Vec<u8>,
    eram: Vec<u8>,
    wram: Vec<u8>,
    zram: Vec<u8>
}