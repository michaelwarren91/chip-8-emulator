const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    framebuffer: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT] // 64 x 32 Display
}

impl Display {
    pub fn new() -> Self {
        Self {
            framebuffer: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT]
        }
    }

    pub fn draw_sprite(&mut self, start_x: usize, start_y: usize, sprite: &Vec<u8>) -> bool {
        let mut reset_pixel = false;

        for (row_index, row_byte) in sprite.iter().enumerate() {
            let y = (start_y + row_index) % DISPLAY_HEIGHT;

            for bit_index in 0..8 {
                let x = (start_x + bit_index) % DISPLAY_WIDTH;
                
                let frame_buffer_index = y * DISPLAY_WIDTH + x;
                let pixel_value = ((row_byte >> (7 - bit_index)) & 0x01) > 0;
                
                reset_pixel |= pixel_value && self.framebuffer[frame_buffer_index];
                self.framebuffer[frame_buffer_index] ^= pixel_value;
            }
        }
        
        reset_pixel
    }

    pub fn clear(&mut self) {
        self.framebuffer.fill(false);
    }

    pub fn get_framebuffer(&self) -> &[bool; DISPLAY_WIDTH * DISPLAY_HEIGHT] {
        &self.framebuffer
    }

    pub fn debug_print_buffer(&self) {
        let buffer_len = DISPLAY_WIDTH * DISPLAY_HEIGHT;
        
        for pixel in 0..buffer_len {
            let value = self.framebuffer[pixel] as u8;
            print!("{value}");

            if (pixel + 1) % DISPLAY_WIDTH == 0 {
                println!("");
            }
        }
    }
}
