use crate::rom::Rom;
use crate::{
    audio::AudioBackend,
    cpu::Chip8,
    frontend::{Frontend, FrontendEvent},
};

use core::time;
use std::thread::sleep;

pub struct Emulator {
    cpu: Chip8,
    frontend: Box<dyn Frontend>,
    audio: Box<dyn AudioBackend>,

    // Runtime state
    is_paused: bool,
    should_exit: bool,
}

impl Emulator {
    pub fn new(
        frontend: impl Frontend + 'static,
        audio_backend: impl AudioBackend + 'static,
    ) -> Self {
        Self {
            cpu: Chip8::default(),
            frontend: Box::new(frontend),
            audio: Box::new(audio_backend),

            is_paused: false,
            should_exit: false,
        }
    }

    pub fn run(&mut self, rom: &Rom) {
        self.initialize(rom);

        while !self.should_exit {
            self.update_input();
            self.update_cpu();
            self.update_audio();
            self.update_display();

            // TODO: see how long it takes to finish updating everything, then
            // wait for the remaning amount of time (1.6 ms - time taken).
            let wait_time_micros = time::Duration::from_micros(1200);
            sleep(wait_time_micros);
        }
    }

    fn initialize(&mut self, rom: &Rom) {
        self.cpu.load_rom(rom.get_data());
    }

    fn update_input(&mut self) {
        for event in self.frontend.poll_events() {
            match event {
                FrontendEvent::KeyPressed(key) => self.handle_key_pressed(key),
                FrontendEvent::KeyReleased(key) => self.handle_key_released(key),
                FrontendEvent::Exit => self.should_exit = true,
            }
        }
    }

    fn handle_key_pressed(&mut self, key: u8) {
        self.cpu.handle_key_pressed(key);
    }

    fn handle_key_released(&mut self, key: u8) {
        self.cpu.handle_key_released(key);
    }

    fn update_cpu(&mut self) {
        if self.is_paused {
            return;
        }

        // Update timers
        self.cpu.step_timers();
        self.cpu.step();
    }

    fn update_audio(&mut self) {
        if self.is_paused {
            // The sound timer won't be updating if we're paused. We should stop the sound
            // if it's playing to avoid an infinite beep
            if self.audio.is_playing() {
                self.audio.stop_sound();
            }

            return;
        }

        let sound_timer_value = self.cpu.sound_timer;
        if sound_timer_value > 0 && !self.audio.is_playing() {
            self.audio.play_sound();
        } else if sound_timer_value == 0 && self.audio.is_playing() {
            self.audio.stop_sound();
        }
    }

    fn update_display(&mut self) {
        self.frontend.render(&self.cpu);
    }
}
