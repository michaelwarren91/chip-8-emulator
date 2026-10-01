use crate::chip8::Cpu;
use crate::frontends::display_trait::Display;

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

    pub fn run(&mut self) {
        // Test IBM loading program directly into cpu and attempting to execute program
        match hex::decode(
            "a254f06562018300640b6a1c6b0ca254f21ef165a267dab7f018f015222cdab7f115222c72025230120e1200e4a11238f0073000122c00ee6000613c620b00e0a267dab7f118e2a11244dab7f018e29e124c1240130a050a050a141e051e051e140a050a050a3c192ac88bc82a19",
        ) {
            Ok(bytes) => {
                self.cpu.load_rom(&bytes);
                self.frontend.initialize();

                while !self.frontend.should_exit() {
                    self.cpu.step_timers();
                    self.frontend.update(&self.cpu);

                    for _ in 0..10 {
                        if self.frontend.should_step_cpu() {
                            self.cpu.step();
                        }
                    }

                    let framebuffer = self.cpu.display.get_framebuffer();
                    self.frontend.render(framebuffer);

                    let wait_time_millis = time::Duration::from_micros(16666);
                    sleep(wait_time_millis);
                }

                self.frontend.deinitialize();
            }
            Err(error) => println!("Error reading bytes: {error}"),
        }
    }
}
