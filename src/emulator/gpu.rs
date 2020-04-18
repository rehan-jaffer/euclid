impl GPU {
  pub fn dump_vram(&self) {
    for k in 0..128 {
        let tile = &self.vram[(k*128)..(k+1)*128];
        for i in 0..8 {
          for j in 0..4 {
            let byte = tile[(i*8)+j];
            let one = if (byte & 0x0F) > 0 { "█" } else { " " };
            let two = if (byte >> 4) > 0 { "█" } else { " " };
            print!("{} {} ", one, two);
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