mod chip8;
mod emulator;
mod frontends;
mod rom;

use emulator::Emulator;

use crate::rom::Rom;
use chip8::Cpu;
use frontends::runtime::RuntimeFrontend;

fn get_input_path() -> Result<String, String> {
    std::env::args()
        .nth(1)
        .ok_or_else(|| "No ROM file path given".to_string())
}

fn main() -> Result<(), String> {
    let rom_file_path = get_input_path()?;
    let rom = Rom::from_file_path(rom_file_path)?;

    let mut emulator = Emulator::new(Cpu::default(), RuntimeFrontend::default());
    emulator.load_rom(&rom);
    emulator.run();

    Ok(())
}
