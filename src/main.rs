mod audio;
mod cpu;
mod emulator;
mod frontend;
mod rom;

use audio::sdl_audio::SdlAudio;
use emulator::Emulator;
use frontend::game_frontend::GameFrontend;
use rom::Rom;

fn get_input_path() -> Result<String, String> {
    std::env::args()
        .nth(1)
        .ok_or_else(|| "No ROM file path given".to_string())
}

fn main() -> Result<(), String> {
    let rom_file_path = get_input_path()?;
    let rom = Rom::from_file_path(rom_file_path)?;

    let sdl_context = sdl2::init().unwrap();
    let mut emulator = Emulator::new(GameFrontend::new(&sdl_context), SdlAudio::new(&sdl_context));
    emulator.run(&rom);

    Ok(())
}
