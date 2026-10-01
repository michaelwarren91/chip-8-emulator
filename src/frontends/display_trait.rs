use crate::chip8::Cpu;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub trait Display {
    fn initialize(&mut self);
    fn update(&mut self, cpu: &Cpu);
    fn render(&mut self, framebuffer: &[bool; DISPLAY_WIDTH * DISPLAY_HEIGHT]);
    fn clear(&mut self);
    fn should_exit(&mut self) -> bool;
    fn should_step_cpu(&self) -> bool;
    fn deinitialize(&mut self);
}
