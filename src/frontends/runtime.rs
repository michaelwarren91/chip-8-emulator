use crate::chip8::Cpu;
use crate::frontends::display_trait;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

const CLEAR_COLOR: sdl2::pixels::Color = Color::RGB(0, 0, 0);
const WRITE_COLOR: sdl2::pixels::Color = Color::RGB(255, 255, 255);
const WINDOW_SCALE: u32 = 20;

struct SquareWave {
    phase: f32,
    phase_increment: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            *sample = if self.phase < 0.5 { 0.25 } else { -0.25 };

            self.phase = (self.phase + self.phase_increment) % 1.0;
        }
    }
}

#[derive(Default)]
pub struct RuntimeFrontend {
    sdl_context: Option<Sdl>,
    canvas: Option<Canvas<Window>>,
    event_pump: Option<EventPump>,

    // Audio
    audio_device: Option<AudioDevice<SquareWave>>,

    should_exit: bool,
    key_events: Vec<(u8, bool)>,
}

impl RuntimeFrontend {
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

    fn process_events(&mut self) {
        if let Some(event_pump) = &mut self.event_pump {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => self.should_exit = true,
                    Event::KeyDown {
                        keycode,
                        repeat: false,
                        ..
                    } if keycode.is_some() => {
                        if let Some(chip8key) = Self::keycode_to_chip8key(keycode.unwrap()) {
                            self.key_events.push((chip8key, true));
                        }
                    }
                    Event::KeyUp {
                        keycode,
                        repeat: false,
                        ..
                    } => {
                        if let Some(chip8key) = Self::keycode_to_chip8key(keycode.unwrap()) {
                            self.key_events.push((chip8key, false));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn update_audio(&self, sound_timer: u8) {
        if let Some(audio_device) = &self.audio_device {
            let status = audio_device.status();

            if sound_timer > 0 && (status == AudioStatus::Stopped || status == AudioStatus::Paused)
            {
                audio_device.resume();
            } else if sound_timer == 0 && status == AudioStatus::Playing {
                audio_device.pause();
            }
        }
    }
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

        // Audio stuff
        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: Some(1024),
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase: 0.0,
                phase_increment: 440.0 / spec.freq as f32,
            })
            .expect("Failed to create audio device");

        self.sdl_context = Some(sdl_context);
        self.canvas = Some(canvas);
        self.event_pump = Some(event_pump);
        self.audio_device = Some(device);

        // TODO: generate key events for keys that are already pressed at time of initialization
    }

    fn update(&mut self, cpu: &Cpu) {
        self.process_events();
        self.update_audio(cpu.sound_timer);
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
        self.should_exit
    }

    fn should_step_cpu(&self) -> bool {
        true
    }

    fn deinitialize(&mut self) {
        // Do nothing for now
    }

    fn consume_keyboard_events(&mut self) -> Vec<(u8, bool)> {
        let copied_events = self.key_events.clone();
        self.key_events.clear();

        copied_events
    }
}
