extern crate clap;
use clap::{Arg, App, SubCommand};

mod emulator;

const VRAM_SIZE : usize = 8096;
const WORKING_RAM_SIZE : usize = 65536;

fn init_mem(mem_size : usize) -> Vec<u8> {
    let mut mem_vec = Vec::new();
    for _ in (0..mem_size) {
        mem_vec.push(0);
    }
    return mem_vec.clone();
}

fn main() {

    let matches = App::new("Euclid")
                          .version("0.001")
                          .author("Ray <pleasedont@emailme.com>")
                          .about("Gameboy emulator in Rust")
                          .arg(clap::Arg::with_name("STACK")
                               .short("s")
                               .long("stack-debugger")
                               .help("enables the stack debugger")
                               .takes_value(false))
                          .arg(clap::Arg::with_name("REGISTERS")
                               .short("r")
                               .long("register-debugger")
                               .help("enables the display of CPU registers")
                               .takes_value(false))
                          .get_matches();

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
        stack_debugger_enabled: matches.is_present("STACK")
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