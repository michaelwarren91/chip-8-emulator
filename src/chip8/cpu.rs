use super::Instruction;
use super::instruction_handlers;

const MEMORY_SIZE: usize = 4096;
const PROGRAM_START_ADDRESS: usize = 0x200;
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
    pub(super) general_registers: [u8; 16],
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
    pub fn load_rom(&mut self, rom_data: &[u8]) {
        let max_program_size = 0xFFF - PROGRAM_START_ADDRESS;
        let rom_size = rom_data.len();

        if rom_size > max_program_size {
            panic!("ROM exeeds the maximum supported size");
        }

        let destination_slice =
            &mut self.ram[PROGRAM_START_ADDRESS..(PROGRAM_START_ADDRESS + rom_size)];
        destination_slice.copy_from_slice(rom_data);
    }

    pub fn step(&mut self) {
        if self.input_wait_register.is_some() {
            return;
        }

        let opcode = self.fetch_instruction();
        self.program_counter += 2;

        let instruction = self.decode_instruction(opcode);
        self.execute_instruction(instruction);
    }

    fn write_default_sprite(&mut self, number: usize, rows: [u8; 5]) {
        let start_address: usize = FONT_START_ADDRESS;
        let write_address = start_address + 5 * number;

        self.ram[write_address..(write_address + 5)].copy_from_slice(&rows);
    }

    fn fetch_instruction(&self) -> u16 {
        let higher_byte = (self.ram[self.program_counter as usize] as u16) << 1;
        let lower_byte = self.ram[(self.program_counter + 1) as usize] as u16;

        higher_byte | lower_byte
    }

    fn decode_instruction(&self, opcode: u16) -> Instruction {
        panic!("Unknown instruction: 0x{:04X}", opcode);
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SystemCall { address } => {
                instruction_handlers::execute_system_call(self, address)
            }

            Instruction::ClearScreen => instruction_handlers::execute_clear_screen(self),

            Instruction::Return => instruction_handlers::execute_return(self),

            Instruction::Jump { address } => instruction_handlers::execute_jump(self, address),

            Instruction::Call { address } => instruction_handlers::execute_call(self, address),

            Instruction::SkipIfRegisterEqualsValue { register, value } => {
                instruction_handlers::execute_skip_if_register_equals_value(self, register, value)
            }

            Instruction::SkipIfRegisterNotEqualsValue { register, value } => {
                instruction_handlers::execute_skip_if_register_not_equals_value(
                    self, register, value,
                )
            }

            Instruction::SkipIfRegistersEqual {
                register_a,
                register_b,
            } => {
                instruction_handlers::execute_skip_if_registers_equal(self, register_a, register_b)
            }

            Instruction::LoadByte { register, value } => {
                instruction_handlers::execute_load_byte(self, register, value)
            }

            Instruction::AddByte { register, value } => {
                instruction_handlers::execute_add_byte(self, register, value)
            }

            Instruction::CopyRegister {
                destination_register,
                source_register,
            } => instruction_handlers::execute_copy_register(
                self,
                destination_register,
                source_register,
            ),

            Instruction::Or {
                destination_register,
                source_register,
            } => instruction_handlers::execute_or(self, destination_register, source_register),

            Instruction::And {
                destination_register,
                source_register,
            } => instruction_handlers::execute_and(self, destination_register, source_register),

            Instruction::Xor {
                destination_register,
                source_register,
            } => instruction_handlers::execute_xor(self, destination_register, source_register),

            Instruction::AddRegisters {
                destination_register,
                source_register,
            } => instruction_handlers::execute_add_registers(
                self,
                destination_register,
                source_register,
            ),

            Instruction::SubtractRegisters {
                destination_register,
                source_register,
            } => instruction_handlers::execute_subtract_registers(
                self,
                destination_register,
                source_register,
            ),

            Instruction::ShiftRight {
                destination_register,
            } => instruction_handlers::execute_shift_right(self, destination_register),

            Instruction::SubtractFromRegister {
                destination_register,
                source_register,
            } => instruction_handlers::execute_subtract_from_register(
                self,
                destination_register,
                source_register,
            ),

            Instruction::ShiftLeft {
                destination_register,
            } => instruction_handlers::execute_shift_left(self, destination_register),

            Instruction::SkipIfRegistersNotEqual {
                register_a,
                register_b,
            } => instruction_handlers::execute_skip_if_registers_not_equal(
                self, register_a, register_b,
            ),

            Instruction::LoadIndexRegister { address } => {
                instruction_handlers::execute_load_index_register(self, address)
            }

            Instruction::JumpWithOffset { offset } => {
                instruction_handlers::execute_jump_with_offset(self, offset)
            }

            Instruction::RandomByte {
                destination_register,
                mask,
            } => instruction_handlers::execute_random_byte(self, destination_register, mask),

            Instruction::DrawSprite {
                x_register,
                y_register,
                height,
            } => instruction_handlers::execute_draw_sprite(self, x_register, y_register, height),

            Instruction::SkipIfKeyPressed { register } => {
                instruction_handlers::execute_skip_if_key_pressed(self, register)
            }

            Instruction::SkipIfKeyNotPressed { register } => {
                instruction_handlers::execute_skip_if_key_not_pressed(self, register)
            }

            Instruction::ReadDelayTimer {
                destination_register,
            } => instruction_handlers::execute_read_delay_timer(self, destination_register),

            Instruction::WaitForKeyPress {
                destination_register,
            } => instruction_handlers::execute_wait_for_key_press(self, destination_register),

            Instruction::SetDelayTimer { source_register } => {
                instruction_handlers::execute_set_delay_timer(self, source_register)
            }

            Instruction::SetSoundTimer { source_register } => {
                instruction_handlers::execute_set_sound_timer(self, source_register)
            }

            Instruction::AddToIndexRegister { source_register } => {
                instruction_handlers::execute_add_to_index_register(self, source_register)
            }

            Instruction::LoadFontAddress { register } => {
                instruction_handlers::execute_load_font_address(self, register)
            }

            Instruction::StoreBcd { source_register } => {
                instruction_handlers::execute_store_bcd(self, source_register)
            }

            Instruction::StoreRegisters { final_register } => {
                instruction_handlers::execute_store_registers(self, final_register)
            }

            Instruction::LoadRegisters { final_register } => {
                instruction_handlers::execute_load_registers(self, final_register)
            }
        }
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

    #[test]
    #[should_panic]
    fn load_program_invalid_size() {
        let mut cpu = Cpu::default();

        // Create dummy program that takes the entire RAM length
        let program = [0u8; MEMORY_SIZE];
        cpu.load_rom(&program);
    }
}
