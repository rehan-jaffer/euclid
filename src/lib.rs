extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use clap::{Arg, App, SubCommand};
mod emulator;

use wasm_bindgen::prelude::*;

const VRAM_SIZE : usize = 65536;
const WORKING_RAM_SIZE : usize = 65536;

fn init_mem(mem_size : usize) -> Vec<u8> {
    let mut mem_vec = Vec::new();
    for _ in (0..mem_size) {
        mem_vec.push(0);
    }
    return mem_vec.clone();
}

#[wasm_bindgen]
pub fn main() {

    let vram = init_mem(VRAM_SIZE);
    let wram = init_mem(WORKING_RAM_SIZE);

    let mut gpu = emulator::gpu::GPU {
        oam: Vec::new(), vram: vram.clone()
    };

    let mut mmu = emulator::mmu::MMU { 
        bios: Vec::new(), 
        booting: true, 
        eram: Vec::new(), 
        wram: wram, 
        rom: Vec::new(), 
        zram: Vec::new(),
        gpu: &mut gpu,
        stack_debugger_enabled: false
    };

    let mut cpu = emulator::cpu::CPU { 
        a: 0, b:0, c:0, d:0, e:0, h:0, l:0, 
        clock_m: 0, clock_t: 0, 
        f: 0, 
        sp: 0, pc: 0, m: 0, 
        mmu: &mut mmu,
        flags: emulator::cpu::Flags { 
            zero: false, 
            negative: false, 
            h: false, 
            carry: false
        } 
    };

    let mut emulator = emulator::Emulator { 
        cpu: &mut cpu,
        running: true
    };

    emulator.load(&"cpu_instrs.gb");
    emulator.loadBIOS();

    emulator.boot();
}