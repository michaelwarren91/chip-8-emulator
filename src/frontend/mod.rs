use crate::cpu::Chip8;

pub mod game_frontend;

pub enum FrontendEvent {
    KeyPressed(u8),
    KeyReleased(u8),
    Exit,
}

pub trait Frontend {
    fn render(&mut self, cpu: &Chip8);
    fn poll_events(&mut self) -> Vec<FrontendEvent>;
}
