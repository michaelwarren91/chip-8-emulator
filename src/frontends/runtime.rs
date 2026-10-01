use crate::frontends::display_trait;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

const CLEAR_COLOR: sdl2::pixels::Color = Color::RGB(0, 0, 0);
const WRITE_COLOR: sdl2::pixels::Color = Color::RGB(255, 255, 255);
const WINDOW_SCALE: u32 = 10;

#[derive(Default)]
pub struct RuntimeFrontend {
    sdl_context: Option<Sdl>,
    canvas: Option<Canvas<Window>>,
    event_pump: Option<EventPump>,
}

impl display_trait::Display for RuntimeFrontend {
    fn initialize(&mut self) {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();

        let window_width = display_trait::DISPLAY_WIDTH as u32 * WINDOW_SCALE;
        let window_height = display_trait::DISPLAY_HEIGHT as u32 * WINDOW_SCALE;
        let window = video_subsystem
            .window("Chip-8 Emulator", window_width, window_height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        self.sdl_context = Some(sdl_context);
        self.canvas = Some(canvas);
        self.event_pump = Some(event_pump);
    }

    fn render(
        &mut self,
        framebuffer: &[bool; display_trait::DISPLAY_WIDTH * display_trait::DISPLAY_HEIGHT],
    ) {
        self.clear();

        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(WRITE_COLOR);
            for column in 0..display_trait::DISPLAY_WIDTH {
                for row in 0..display_trait::DISPLAY_HEIGHT {
                    let framebuffer_index = (row * display_trait::DISPLAY_WIDTH) + column;
                    let pixel_value = framebuffer[framebuffer_index];

                    if pixel_value {
                        let _ = canvas.fill_rect(Rect::new(
                            column as i32 * WINDOW_SCALE as i32,
                            row as i32 * WINDOW_SCALE as i32,
                            WINDOW_SCALE,
                            WINDOW_SCALE,
                        ));
                    }
                }
            }

            canvas.present();
        }
    }

    fn clear(&mut self) {
        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(CLEAR_COLOR);
            canvas.clear();
        }
    }

    fn should_exit(&mut self) -> bool {
        if let Some(event_pump) = &mut self.event_pump {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return true,
                    _ => {}
                }
            }
        }

        false
    }

    fn should_step_cpu(&self) -> bool {
        true
    }

    fn deinitialize(&mut self) {
        // Do nothing for now
    }
}
