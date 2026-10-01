mod chip8;
mod emulator;
mod frontends;

use emulator::Emulator;

fn main() {
    let cpu = chip8::Cpu::default();
    let frontend = frontends::runtime::RuntimeFrontend::default();

    let mut emulator = Emulator::new(cpu, frontend);
    emulator.run();
}
