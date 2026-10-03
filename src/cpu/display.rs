use super::specs;

pub struct Display {
    framebuffer: [bool; (specs::DISPLAY_WIDTH * specs::DISPLAY_HEIGHT) as usize], // 64 x 32 Display
}

impl Display {
    pub fn new() -> Self {
        Self {
            framebuffer: [false; (specs::DISPLAY_WIDTH * specs::DISPLAY_HEIGHT) as usize],
        }
    }

    pub fn draw_sprite(&mut self, start_x: usize, start_y: usize, sprite: &[u8]) -> bool {
        let mut reset_pixel = false;

        for (row_index, row_byte) in sprite.iter().enumerate() {
            let y = (start_y + row_index) % specs::DISPLAY_HEIGHT as usize;

            for bit_index in 0..8 {
                let x = (start_x + bit_index) % specs::DISPLAY_WIDTH as usize;

                let frame_buffer_index = y * specs::DISPLAY_WIDTH as usize + x;
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

    pub fn get_framebuffer(
        &self,
    ) -> &[bool; (specs::DISPLAY_WIDTH * specs::DISPLAY_HEIGHT) as usize] {
        &self.framebuffer
    }
}
