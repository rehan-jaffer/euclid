impl GPU {
  pub fn dump_vram(&self) {
    for k in 0..128 {
      for i in 0..8 {
        for j in 0..8 {
          let digit = if self.vram[(k*128)+((i*8)+j)] > 0 { 1 } else { 0 };
          print!("{} ", self.vram[(k*128)+((i*8)+j)]);
        }
        print!("\r\n");
      }
  
    }
  }
}

#[derive(Default)]
pub struct GPU {
  pub oam: Vec<u8>,
  pub vram: Vec<u8>
}