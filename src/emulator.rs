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
            "00e0a22a600c6108d01f7009a239d01fa2487008d01f7004a257d01f7008a266d01f7008a275d01f1228ff00ff003c003c003c003c00ff00ffff00ff0038003f003f003800ff00ff8000e000e00080008000e000e00080f800fc003e003f003b003900f800f8030007000f00bf00fb00f300e30043e000e0008000800080008000e000e0",
        ) {
            Ok(bytes) => {
                self.cpu.load_rom(&bytes);
                self.frontend.initialize();

                while !self.frontend.should_exit() {
                    for _ in 0..10 {
                        if self.frontend.should_step_cpu() {
                            self.cpu.step();
                        }
                    }

                    let framebuffer = self.cpu.display.get_framebuffer();
                    self.frontend.render(framebuffer);

                    let wait_time_millis = time::Duration::from_millis(16);
                    sleep(wait_time_millis);
                }

                self.frontend.deinitialize();
            }
            Err(error) => println!("Error reading bytes: {error}"),
        }
    }
}
