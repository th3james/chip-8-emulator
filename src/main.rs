fn main() {
    // opcodes are two bytes
    let _opcode: u16;
    // CHIP-8 has 4k memory
    let _memory: [u8; 4096];

    let _v_registers: [u8; 16];
    let _index_register: u16;
    let _program_counter: u16;

    // 64 x 32 resolution black & white pixels
    let _frame_buffer: [bool; 64 * 32];

    // These decrement on each cycle
    let _delay_timer: u8;
    let _sound_timer: u8;

    // stack for handling jumps
    let _stack: [u16; 16];
    let _stack_pointer: u16;

    let _keypad_state: [bool; 16]

    println!("Hello, world!");
}
