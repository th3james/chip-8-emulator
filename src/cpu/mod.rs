use std::fs;
use std::path::Path;

pub struct CPU {
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
    pub fn initialize() -> CPU {
        CPU {
            opcode: 0,
            memory: [0; 4096],
            v_registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            frame_buffer: [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
            keypad_state: [false; 16],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_resets_program_counter() {
        assert_eq!(CPU::initialize().program_counter, 0x200);
    }

    #[test]
    fn test_initialize_zeros_memory() {
        assert_eq!(CPU::initialize().memory[0x200..0x210], [0; 0x10]);
    }

    #[test]
    fn test_load_game_populates_memory() {
        let test_game_path = Path::new("./test_fixtures/test_game.ch8");

        let file_contents = fs::read(test_game_path).expect(
            format!(
                "Couldn't read test fixture {}",
                test_game_path.canonicalize().unwrap().display()
            )
            .as_str(),
        );

        let cpu = CPU::initialize();
        cpu.load_game(test_game_path);

        assert_eq!(
            cpu.memory[0x200..0x210 + file_contents.len()],
            file_contents
        );
    }
}
