mod emulator;

fn main() {

    let mut vram = Vec::new();
    let mut wram = Vec::new();

    for i in (0..8096) {
        vram.push(0);
    }

    for i in (0..65536) {
        wram.push(0);
    }

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
        gpu: &mut gpu 
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