use std::time::Instant;

use super::Display;
use super::Instruction;
use super::instruction_handlers;

const MEMORY_SIZE: usize = 4096;
const PROGRAM_START_ADDRESS: usize = 0x200;
pub const FONT_START_ADDRESS: usize = 0x000;
const FONT_DATA: [[u8; 5]; 16] = [
    // Data for the number 0 (1 = pixel on, 0 = pixel off). All following numbers follow the same format
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
    pub(super) index_register: u16,
    pub(super) program_counter: u16,
    pub(super) stack_pointer: u8,

    // Memory
    pub(super) ram: [u8; MEMORY_SIZE],
    pub(super) subroutine_stack: [u16; 16],

    // Rendering
    pub display: Display,

    // Input
    pub(super) input_state: u16,
    pub(super) input_wait_register: Option<u8>, // Used for the Fx0A instruction

    // Timers
    pub(super) delay_timer: u8,
    pub sound_timer: u8,
    last_update_time: Instant,
    accumulated_time: f64,
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

    pub fn step_timers(&mut self) {
        let now = Instant::now();
        let elapsed_time = now - self.last_update_time;

        self.accumulated_time += elapsed_time.as_secs_f64();
        let target_tick_time = 1.0 / 60.0;

        while self.accumulated_time >= target_tick_time {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }

            self.accumulated_time -= target_tick_time;
        }

        self.last_update_time = now;
    }

    fn write_default_sprite(&mut self, number: usize, rows: [u8; 5]) {
        let start_address: usize = FONT_START_ADDRESS;
        let write_address = start_address + 5 * number;

        self.ram[write_address..(write_address + 5)].copy_from_slice(&rows);
    }

    fn fetch_instruction(&self) -> u16 {
        let higher_byte = (self.ram[self.program_counter as usize] as u16) << 8;
        let lower_byte = self.ram[(self.program_counter + 1) as usize] as u16;

        higher_byte | lower_byte
    }

    fn decode_instruction(&self, opcode: u16) -> Instruction {
        let address = opcode & 0x0FFF;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let byte = (opcode & 0x00FF) as u8;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => Instruction::ClearScreen,
                0x00EE => Instruction::Return,
                _ => Instruction::SystemCall { address },
            },

            0x1000 => Instruction::Jump { address },
            0x2000 => Instruction::Call { address },
            0x3000 => Instruction::SkipIfRegisterEqualsValue {
                register: x,
                value: byte,
            },
            0x4000 => Instruction::SkipIfRegisterNotEqualsValue {
                register: x,
                value: byte,
            },
            0x5000 => Instruction::SkipIfRegistersEqual {
                register_a: x,
                register_b: y,
            },
            0x6000 => Instruction::LoadByte {
                register: x,
                value: byte,
            },
            0x7000 => Instruction::AddByte {
                register: x,
                value: byte,
            },
            0x8000 => match n {
                0x0 => Instruction::CopyRegister {
                    destination_register: x,
                    source_register: y,
                },
                0x1 => Instruction::Or {
                    destination_register: x,
                    source_register: y,
                },
                0x2 => Instruction::And {
                    destination_register: x,
                    source_register: y,
                },
                0x3 => Instruction::Xor {
                    destination_register: x,
                    source_register: y,
                },
                0x4 => Instruction::AddRegisters {
                    destination_register: x,
                    source_register: y,
                },
                0x5 => Instruction::SubtractRegisters {
                    destination_register: x,
                    source_register: y,
                },
                0x6 => Instruction::ShiftRight {
                    destination_register: x,
                },
                0x7 => Instruction::SubtractFromRegister {
                    destination_register: x,
                    source_register: y,
                },
                0xE => Instruction::ShiftLeft {
                    destination_register: x,
                },
                _ => Instruction::Unknown { opcode },
            },
            0x9000 => Instruction::SkipIfRegistersNotEqual {
                register_a: x,
                register_b: y,
            },
            0xA000 => Instruction::LoadIndexRegister { address },
            0xB000 => Instruction::JumpWithOffset { address },
            0xC000 => Instruction::RandomByte {
                destination_register: x,
                mask: byte,
            },
            0xD000 => Instruction::DrawSprite {
                x_register: x,
                y_register: y,
                height: n,
            },
            0xE000 => match byte {
                0x9E => Instruction::SkipIfKeyPressed { register: x },
                0xA1 => Instruction::SkipIfKeyNotPressed { register: x },
                _ => Instruction::Unknown { opcode },
            },
            0xF000 => match byte {
                0x07 => Instruction::ReadDelayTimer {
                    destination_register: x,
                },
                0x0A => Instruction::WaitForKeyPress {
                    destination_register: x,
                },
                0x15 => Instruction::SetDelayTimer { source_register: x },
                0x18 => Instruction::SetSoundTimer { source_register: x },
                0x1E => Instruction::AddToIndexRegister { source_register: x },
                0x29 => Instruction::LoadFontAddress { register: x },
                0x33 => Instruction::StoreBcd { source_register: x },
                0x55 => Instruction::StoreRegisters { final_register: x },
                0x65 => Instruction::LoadRegisters { final_register: x },
                _ => Instruction::Unknown { opcode },
            },
            _ => Instruction::Unknown { opcode },
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Unknown { opcode } => {
                panic!("Unknown instruction reached: opcode={opcode}")
            }

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

            Instruction::JumpWithOffset { address } => {
                instruction_handlers::execute_jump_with_offset(self, address)
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
            program_counter: 0x200,
            stack_pointer: 0,

            ram: [0; MEMORY_SIZE],
            subroutine_stack: [0; 16],

            display: Display::new(),

            input_state: 0,
            input_wait_register: None,

            delay_timer: 0,
            sound_timer: 0,
            last_update_time: Instant::now(),
            accumulated_time: 0.0,
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
