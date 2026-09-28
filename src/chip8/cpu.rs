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

impl Default for Cpu {
    fn default() -> Self {
        Self {
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
            input_wait_register: None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_cpu() {
        let cpu = Cpu::default();

        // Sort of pointless checks right now...
        assert_eq!(cpu.program_counter, 0x200);
    }
}