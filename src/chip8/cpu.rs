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
