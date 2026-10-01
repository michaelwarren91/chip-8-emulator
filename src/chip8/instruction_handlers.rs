use super::Cpu;

pub(super) fn execute_system_call(cpu: &mut Cpu, address: u16) {
    todo!();
}

pub(super) fn execute_clear_screen(cpu: &mut Cpu) {
    cpu.display.clear();
}

pub(super) fn execute_return(cpu: &mut Cpu) {
    todo!();
}

pub(super) fn execute_jump(cpu: &mut Cpu, address: u16) {
    cpu.program_counter = address;
}

pub(super) fn execute_call(cpu: &mut Cpu, address: u16) {
    todo!();
}

pub(super) fn execute_skip_if_register_equals_value(cpu: &mut Cpu, register: u8, value: u8) {
    todo!();
}

pub(super) fn execute_skip_if_register_not_equals_value(cpu: &mut Cpu, register: u8, value: u8) {
    todo!();
}

pub(super) fn execute_skip_if_registers_equal(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    todo!();
}

pub(super) fn execute_load_byte(cpu: &mut Cpu, register: u8, value: u8) {
    cpu.general_registers[register as usize] = value;
}

pub(super) fn execute_add_byte(cpu: &mut Cpu, register: u8, value: u8) {
    cpu.general_registers[register as usize] += value;
}

pub(super) fn execute_copy_register(cpu: &mut Cpu, destination_register: u8, source_register: u8) {
    todo!();
}

pub(super) fn execute_or(cpu: &mut Cpu, destination_register: u8, source_register: u8) {
    todo!();
}

pub(super) fn execute_and(cpu: &mut Cpu, destination_register: u8, source_register: u8) {
    todo!();
}

pub(super) fn execute_xor(cpu: &mut Cpu, destination_register: u8, source_register: u8) {
    todo!();
}

pub(super) fn execute_add_registers(cpu: &mut Cpu, destination_register: u8, source_register: u8) {
    todo!();
}

pub(super) fn execute_subtract_registers(
    cpu: &mut Cpu,
    destination_register: u8,
    source_register: u8,
) {
    todo!();
}

pub(super) fn execute_shift_right(cpu: &mut Cpu, destination_register: u8) {
    todo!();
}

pub(super) fn execute_subtract_from_register(
    cpu: &mut Cpu,
    destination_register: u8,
    source_register: u8,
) {
    todo!();
}

pub(super) fn execute_shift_left(cpu: &mut Cpu, destination_register: u8) {
    todo!();
}

pub(super) fn execute_skip_if_registers_not_equal(cpu: &mut Cpu, register_a: u8, register_b: u8) {
    todo!();
}

pub(super) fn execute_load_index_register(cpu: &mut Cpu, address: u16) {
    cpu.index_register = address;
}

pub(super) fn execute_jump_with_offset(cpu: &mut Cpu, offset: u16) {
    todo!();
}

pub(super) fn execute_random_byte(cpu: &mut Cpu, destination_register: u8, mask: u8) {
    todo!();
}

pub(super) fn execute_draw_sprite(cpu: &mut Cpu, x_register: u8, y_register: u8, height: u8) {
    let x = cpu.general_registers[x_register as usize];
    let y = cpu.general_registers[y_register as usize];

    let start = cpu.index_register as usize;
    let sprite = &cpu.ram[start..(start + height as usize)];

    let pixels_erased = cpu.display.draw_sprite(x as usize, y as usize, sprite);
    cpu.general_registers[15] = if pixels_erased { 1 } else { 0 };
}

pub(super) fn execute_skip_if_key_pressed(cpu: &mut Cpu, register: u8) {
    todo!();
}

pub(super) fn execute_skip_if_key_not_pressed(cpu: &mut Cpu, register: u8) {
    todo!();
}

pub(super) fn execute_read_delay_timer(cpu: &mut Cpu, destination_register: u8) {
    todo!();
}

pub(super) fn execute_wait_for_key_press(cpu: &mut Cpu, destination_register: u8) {
    todo!();
}

pub(super) fn execute_set_delay_timer(cpu: &mut Cpu, source_register: u8) {
    todo!();
}

pub(super) fn execute_set_sound_timer(cpu: &mut Cpu, source_register: u8) {
    todo!();
}

pub(super) fn execute_add_to_index_register(cpu: &mut Cpu, source_register: u8) {
    todo!();
}

pub(super) fn execute_load_font_address(cpu: &mut Cpu, register: u8) {
    todo!();
}

pub(super) fn execute_store_bcd(cpu: &mut Cpu, source_register: u8) {
    todo!();
}

pub(super) fn execute_store_registers(cpu: &mut Cpu, final_register: u8) {
    todo!();
}

pub(super) fn execute_load_registers(cpu: &mut Cpu, final_register: u8) {
    todo!();
}
