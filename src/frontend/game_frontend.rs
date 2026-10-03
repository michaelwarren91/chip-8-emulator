use sdl2::{
    EventPump, Sdl, event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas,
    video::Window,
};

use super::Frontend;
use super::FrontendEvent;
use crate::cpu::specs;

const CLEAR_COLOR: sdl2::pixels::Color = Color::RGB(0, 0, 0);
const WRITE_COLOR: sdl2::pixels::Color = Color::RGB(255, 255, 255);
const WINDOW_SCALE: u32 = 20;

pub struct GameFrontend {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl GameFrontend {
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window_width = specs::DISPLAY_WIDTH * WINDOW_SCALE;
        let window_height = specs::DISPLAY_HEIGHT * WINDOW_SCALE;
        let window = video_subsystem
            .window("Chip-8 Emulator", window_width, window_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Self { canvas, event_pump }
    }

    fn map_to_frontend_event(sdl_event: Event) -> Option<FrontendEvent> {
        match sdl_event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => Some(FrontendEvent::Exit),

            Event::KeyDown {
                keycode: Some(keycode),
                repeat: false,
                ..
            } => {
                let chip8key = Self::keycode_to_chip8key(keycode)?;
                Some(FrontendEvent::KeyPressed(chip8key))
            }

            Event::KeyUp {
                keycode: Some(keycode),
                repeat: false,
                ..
            } => {
                let chip8key = Self::keycode_to_chip8key(keycode)?;
                Some(FrontendEvent::KeyReleased(chip8key))
            }

            _ => None,
        }
    }

    fn keycode_to_chip8key(keycode: Keycode) -> Option<u8> {
        match keycode {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xC),

            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),

            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),

            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),

            _ => None,
        }
    }
}

impl Frontend for GameFrontend {
    fn render(&mut self, cpu: &crate::cpu::Chip8) {
        // Clear window
        self.canvas.set_draw_color(CLEAR_COLOR);
        self.canvas.clear();

        self.canvas.set_draw_color(WRITE_COLOR);

        let framebuffer = cpu.display.get_framebuffer();
        let display_width = specs::DISPLAY_WIDTH;
        let display_height = specs::DISPLAY_HEIGHT;

        for column in 0..display_width {
            for row in 0..display_height {
                let framebuffer_index = (row * specs::DISPLAY_WIDTH) + column;
                let pixel_value = framebuffer[framebuffer_index as usize];

                if pixel_value {
                    let _ = self.canvas.fill_rect(Rect::new(
                        column as i32 * WINDOW_SCALE as i32,
                        row as i32 * WINDOW_SCALE as i32,
                        WINDOW_SCALE,
                        WINDOW_SCALE,
                    ));
                }
            }
        }

        self.canvas.present();
    }

    fn poll_events(&mut self) -> Vec<FrontendEvent> {
        self.event_pump
            .poll_iter()
            .filter_map(Self::map_to_frontend_event)
            .collect()
    }
}
