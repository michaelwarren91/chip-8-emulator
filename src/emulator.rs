use crate::chip8::Cpu;
use crate::frontends::display_trait::Display;
use crate::rom::Rom;

use core::time;
use std::thread::sleep;

pub struct Emulator {
    cpu: Cpu,
    frontend: Box<dyn Display>,
}

impl Emulator {
    pub fn new(cpu: Cpu, frontend: impl Display + 'static) -> Self {
        Self {
            cpu,
            frontend: Box::new(frontend),
        }
    }

    pub fn load_rom(&mut self, rom: &Rom) {
        self.cpu.load_rom(rom.get_data());
    }

    pub fn run(&mut self) {
        self.frontend.initialize();

        while !self.frontend.should_exit() {
            self.cpu.step_timers();
            self.frontend.update(&self.cpu);

            // Handle key events
            let key_events = self.frontend.consume_keyboard_events();
            for (key, pressed) in key_events {
                if pressed {
                    self.cpu.handle_key_pressed(key);
                } else {
                    self.cpu.handle_key_released(key);
                }
            }

            if self.frontend.should_step_cpu() {
                self.cpu.step();
            }

            let framebuffer = self.cpu.display.get_framebuffer();
            self.frontend.render(framebuffer);

            let wait_time_micros = time::Duration::from_micros(1666);
            sleep(wait_time_micros);
        }

        self.frontend.deinitialize();
    }
}
