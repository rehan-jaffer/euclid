mod emulator;

fn main() {
    let mmu = emulator::mmu::MMU { bios: Vec::new(), booting: true, eram: Vec::new(), wram: Vec::new(), rom: Vec::new(), zram: Vec::new() };
    let cpu = emulator::cpu::CPU { a: 0, b:0, c:0, d:0, e:0, h:0, l:0, clock_m: 0, clock_t: 0, f: 0, sp: 0, pc: 0 };
    let mut emulator = emulator::Emulator { 
        cpu: cpu,
        mmu: mmu,
        running: true
    };
    emulator.load(&"cpu_instrs.gb");
    emulator.loadBIOS();
    emulator.boot();
}