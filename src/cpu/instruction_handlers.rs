use super::chip8::FONT_START_ADDRESS;
use rand::Rng;

use super::Chip8;

pub(super) fn execute_system_call(_cpu: &mut Chip8, _address: u16) {
    // Intentionally ignored
}

pub(super) fn execute_clear_screen(cpu: &mut Chip8) {
    cpu.display.clear();
}

pub(super) fn execute_return(cpu: &mut Chip8) {
    assert!(
        cpu.stack_pointer > 0,
        "Cannot execute RET with an empty call stack"
    );

    cpu.stack_pointer -= 1;

    let return_address = cpu.subroutine_stack[cpu.stack_pointer as usize];
    cpu.program_counter = return_address;
}

pub(super) fn execute_jump(cpu: &mut Chip8, address: u16) {
    cpu.program_counter = address;
}

pub(super) fn execute_call(cpu: &mut Chip8, address: u16) {
    assert!(
        cpu.stack_pointer < 16,
        "Cannot execute CALL: subroutine stack is full"
    );

    cpu.subroutine_stack[cpu.stack_pointer as usize] = cpu.program_counter;
    cpu.stack_pointer += 1;

    cpu.program_counter = address;
}

pub(super) fn execute_skip_if_register_equals_value(cpu: &mut Chip8, register: u8, value: u8) {
    let register_value = cpu.general_registers[register as usize];
    if register_value == value {
        cpu.program_counter += 2;
    }
}

pub(super) fn execute_skip_if_register_not_equals_value(cpu: &mut Chip8, register: u8, value: u8) {
    let register_value = cpu.general_registers[register as usize];
    if register_value != value {
        cpu.program_counter += 2;
    }
}

pub(super) fn execute_skip_if_registers_equal(cpu: &mut Chip8, register_a: u8, register_b: u8) {
    let value_a = cpu.general_registers[register_a as usize];
    let value_b = cpu.general_registers[register_b as usize];

    if value_a == value_b {
        cpu.program_counter += 2;
    }
}

pub(super) fn execute_load_byte(cpu: &mut Chip8, register: u8, value: u8) {
    cpu.general_registers[register as usize] = value;
}

pub(super) fn execute_add_byte(cpu: &mut Chip8, register: u8, value: u8) {
    let register_value = cpu.general_registers[register as usize];
    let result = register_value.wrapping_add(value);

    cpu.general_registers[register as usize] = result;
}

pub(super) fn execute_copy_register(
    cpu: &mut Chip8,
    destination_register: u8,
    source_register: u8,
) {
    cpu.general_registers[destination_register as usize] =
        cpu.general_registers[source_register as usize];
}

pub(super) fn execute_or(cpu: &mut Chip8, destination_register: u8, source_register: u8) {
    let source_value = cpu.general_registers[source_register as usize];
    let destination_value = cpu.general_registers[destination_register as usize];

    cpu.general_registers[destination_register as usize] = source_value | destination_value;
}

pub(super) fn execute_and(cpu: &mut Chip8, destination_register: u8, source_register: u8) {
    let source_value = cpu.general_registers[source_register as usize];
    let destination_value = cpu.general_registers[destination_register as usize];

    cpu.general_registers[destination_register as usize] = source_value & destination_value;
}

pub(super) fn execute_xor(cpu: &mut Chip8, destination_register: u8, source_register: u8) {
    let source_value = cpu.general_registers[source_register as usize];
    let destination_value = cpu.general_registers[destination_register as usize];

    cpu.general_registers[destination_register as usize] = source_value ^ destination_value;
}

pub(super) fn execute_add_registers(
    cpu: &mut Chip8,
    destination_register: u8,
    source_register: u8,
) {
    let source_value = cpu.general_registers[source_register as usize];
    let destination_value = cpu.general_registers[destination_register as usize];
    let (result, did_overflow) = source_value.overflowing_add(destination_value);

    cpu.general_registers[destination_register as usize] = result;
    cpu.general_registers[15] = if did_overflow { 1 } else { 0 };
}

pub(super) fn execute_subtract_registers(
    cpu: &mut Chip8,
    destination_register: u8,
    source_register: u8,
) {
    let source_value = cpu.general_registers[source_register as usize];
    let destination_value = cpu.general_registers[destination_register as usize];
    let (result, did_borrow) = destination_value.borrowing_sub(source_value, false);

    cpu.general_registers[destination_register as usize] = result;
    cpu.general_registers[15] = if !did_borrow { 1 } else { 0 };
}

pub(super) fn execute_shift_right(cpu: &mut Chip8, destination_register: u8) {
    let register_value = cpu.general_registers[destination_register as usize];

    cpu.general_registers[destination_register as usize] >>= 1;
    cpu.general_registers[15] = if register_value & 0x01 > 0 { 1 } else { 0 };
}

pub(super) fn execute_subtract_from_register(
    cpu: &mut Chip8,
    destination_register: u8,
    source_register: u8,
) {
    let source_value = cpu.general_registers[source_register as usize];
    let destination_value = cpu.general_registers[destination_register as usize];
    let (result, did_borrow) = source_value.borrowing_sub(destination_value, false);

    cpu.general_registers[destination_register as usize] = result;
    cpu.general_registers[15] = if !did_borrow { 1 } else { 0 };
}

pub(super) fn execute_shift_left(cpu: &mut Chip8, destination_register: u8) {
    let register_value = cpu.general_registers[destination_register as usize];

    cpu.general_registers[destination_register as usize] <<= 1;
    cpu.general_registers[15] = if register_value & 0x80 > 0 { 1 } else { 0 };
}

pub(super) fn execute_skip_if_registers_not_equal(cpu: &mut Chip8, register_a: u8, register_b: u8) {
    let value_a = cpu.general_registers[register_a as usize];
    let value_b = cpu.general_registers[register_b as usize];

    if value_a != value_b {
        cpu.program_counter += 2;
    }
}

pub(super) fn execute_load_index_register(cpu: &mut Chip8, address: u16) {
    cpu.index_register = address;
}

pub(super) fn execute_jump_with_offset(cpu: &mut Chip8, address: u16) {
    let offset = cpu.general_registers[0] as u16;
    cpu.program_counter = address + offset;
}

pub(super) fn execute_random_byte(cpu: &mut Chip8, destination_register: u8, mask: u8) {
    let mut rng = rand::rng();

    let rand_number = rng.random_range(0..=255);
    let result = rand_number & mask;

    cpu.general_registers[destination_register as usize] = result;
}

pub(super) fn execute_draw_sprite(cpu: &mut Chip8, x_register: u8, y_register: u8, height: u8) {
    let x = cpu.general_registers[x_register as usize];
    let y = cpu.general_registers[y_register as usize];

    let start = cpu.index_register as usize;
    let sprite = &cpu.ram[start..(start + height as usize)];

    let did_erase_pixels = cpu.display.draw_sprite(x as usize, y as usize, sprite);
    cpu.general_registers[15] = if did_erase_pixels { 1 } else { 0 };
}

pub(super) fn execute_skip_if_key_pressed(cpu: &mut Chip8, register: u8) {
    let key = cpu.general_registers[register as usize];
    let key_mask = 1 << key;

    if cpu.input_state & key_mask > 0 {
        cpu.program_counter += 2;
    }
}

pub(super) fn execute_skip_if_key_not_pressed(cpu: &mut Chip8, register: u8) {
    let key = cpu.general_registers[register as usize];
    let key_mask = 1 << key;

    if cpu.input_state & key_mask == 0 {
        cpu.program_counter += 2;
    }
}

pub(super) fn execute_read_delay_timer(cpu: &mut Chip8, destination_register: u8) {
    cpu.general_registers[destination_register as usize] = cpu.delay_timer;
}

pub(super) fn execute_wait_for_key_press(cpu: &mut Chip8, destination_register: u8) {
    // Loop through all keys from 0 to F. If any key is pressed we'll use that key
    // and set the register value. Otherwise, we'll enter a waiting state
    for key in 0..=15 {
        let key_mask = 1 << key;

        if cpu.input_state & key_mask > 0 {
            cpu.general_registers[destination_register as usize] = key;
            return;
        }
    }

    cpu.input_wait_register = Some(destination_register);
}

pub(super) fn execute_set_delay_timer(cpu: &mut Chip8, source_register: u8) {
    cpu.delay_timer = cpu.general_registers[source_register as usize];
}

pub(super) fn execute_set_sound_timer(cpu: &mut Chip8, source_register: u8) {
    cpu.sound_timer = cpu.general_registers[source_register as usize];
}

pub(super) fn execute_add_to_index_register(cpu: &mut Chip8, source_register: u8) {
    let register_value = cpu.general_registers[source_register as usize] as u16;
    cpu.index_register += register_value;
}

pub(super) fn execute_load_font_address(cpu: &mut Chip8, register: u8) {
    let value = cpu.general_registers[register as usize] as usize;
    let sprite_address = (FONT_START_ADDRESS + value * 5) as u16;

    cpu.index_register = sprite_address;
}

pub(super) fn execute_store_bcd(cpu: &mut Chip8, source_register: u8) {
    let register_value = cpu.general_registers[source_register as usize];
    let base_address = cpu.index_register as usize;

    cpu.ram[base_address..=(base_address + 2)].copy_from_slice(&[
        register_value / 100,
        (register_value / 10) % 10,
        register_value % 10,
    ]);
}

pub(super) fn execute_store_registers(cpu: &mut Chip8, final_register: u8) {
    let base_address = cpu.index_register as usize;
    let register_count = final_register as usize + 1;

    cpu.ram[base_address..(base_address + register_count)]
        .copy_from_slice(&cpu.general_registers[..register_count]);
}

pub(super) fn execute_load_registers(cpu: &mut Chip8, final_register: u8) {
    let base_address = cpu.index_register as usize;
    let register_count = final_register as usize + 1;

    cpu.general_registers[..register_count]
        .copy_from_slice(&cpu.ram[base_address..(base_address + register_count)]);
}
