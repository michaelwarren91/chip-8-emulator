const FONT_START_ADDRESS: usize = 0x000;
const FONT_DATA: [[u8; 5]; 16] = [
    // Data for 0 (1 = pixel on, 0 = pixel off). All following numbers follow the same format
    // 11110000 - 0xF0
    // 10010000 - 9x90
    // 10010000 - 0x90
    // 10010000 - 0x90
    // 11110000 - 0xF0
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

pub struct Cpu {
    // Registers
    general_registers: [u8; 16],
    index_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,

    // Memory
    ram: [u8; 4096],
    subroutine_stack: [u16; 16],

    // Rendering
    framebuffer: [bool; 2048],

    // Input
    input_state: u16,
    input_wait_register: Option<u8>, // Used for the Fx0A instruction
}

impl Cpu {
    fn write_default_sprite(&mut self, number: usize, rows: [u8; 5]) {
        let start_address: usize = FONT_START_ADDRESS;
        let write_address = start_address + 5 * number;

        self.ram[write_address..(write_address + 5)].copy_from_slice(&rows);
    }
}

impl Default for Cpu {
    fn default() -> Self {
        let mut cpu = Self {
            general_registers: [0; 16],
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x200,
            stack_pointer: 0,

            ram: [0; 4096],
            subroutine_stack: [0; 16],

            framebuffer: [false; 2048],

            input_state: 0,
            input_wait_register: None,
        };

        for (number, font_data) in FONT_DATA.iter().enumerate().take(0xF) {
            cpu.write_default_sprite(number, *font_data);
        }

        cpu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_cpu_program_counter() {
        let cpu = Cpu::default();
        assert_eq!(cpu.program_counter, 0x200);
    }

    #[test]
    fn defualt_cpu_font_data() {
        let cpu = Cpu::default();
        let font_size = 5;

        for (number, font_data) in FONT_DATA.iter().enumerate().take(0xF) {
            for byte_index in 0..font_size {
                let address = FONT_START_ADDRESS + number * font_size + byte_index;
                assert_eq!(cpu.ram[address], font_data[byte_index]);
            }
        }
    }
}
