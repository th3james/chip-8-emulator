struct CPU {
    // opcodes are two bytes
    opcode: u16,
    // CHIP-8 has 4k memory
    memory: [u8; 4096],

    v_registers: [u8; 16],
    index_register: u16,
    program_counter: u16,

    // 64 x 32 resolution black & white pixels
    frame_buffer: [bool; 64 * 32],

    // These decrement on each cycle
    delay_timer: u8,
    sound_timer: u8,

    // stack for handling jumps
    stack: [u16; 16],
    stack_pointer: u16,

    keypad_state: [bool; 16],
}

impl CPU {
    fn initialize() -> CPU {
        CPU {
            opcode: 0,
            memory: [0; 4096],
            v_registers: [0; 16],
            index_register: 0,
            program_counter: 0,
            frame_buffer: [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
            keypad_state: [false; 16],
        }
    }
}

fn main() {
    let cpu = CPU::initialize();

    println!("Hello, world!");
}
