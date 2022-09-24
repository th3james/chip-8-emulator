use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// CHIP-8 has 4k memory
const CHIP_8_MEMORY_SIZE: usize = 4096;
const APPLICATION_START_ADDRESS: usize = 0x200;
const MAX_GAME_SIZE: usize = (CHIP_8_MEMORY_SIZE - APPLICATION_START_ADDRESS) as usize;

pub struct CPU {
    // opcodes are two bytes
    opcode: u16,
    memory: [u8; CHIP_8_MEMORY_SIZE],

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
            memory: [0; CHIP_8_MEMORY_SIZE as usize],
            v_registers: [0; 16],
            index_register: 0,
            program_counter: APPLICATION_START_ADDRESS as u16,
            frame_buffer: [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
            keypad_state: [false; 16],
        }
    }

    pub fn load_game(&mut self, game: &mut dyn Read) -> Result<(), std::io::Error> {
        // TODO After reading take, try reading more - input buffer should be consumed
        match game.take(MAX_GAME_SIZE as u64).read(&mut self.memory[APPLICATION_START_ADDRESS..]) {
           Err(e) => Err(e),
           Ok(_s) => Ok(()),
        }
    }

    pub fn load_game_from_file(&mut self, file_path: &Path) -> Result<(), std::io::Error>  {
        let mut game_file = File::open(file_path).expect(
            format!(
                "Couldn't open file {}",
                file_path.canonicalize().unwrap().display()
            )
            .as_str(),
        );

        self.load_game(&mut game_file)
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
    fn test_load_game_from_file_populates_memory() {
        let test_game_path = Path::new("./test_fixtures/test_game.ch8");

        let file_contents = fs::read(test_game_path).expect(
            format!(
                "Couldn't read test fixture {}",
                test_game_path.canonicalize().unwrap().display()
            )
            .as_str(),
        );

        let mut cpu = CPU::initialize();
        cpu.load_game_from_file(test_game_path);

        assert_eq!(
            cpu.memory[APPLICATION_START_ADDRESS..(APPLICATION_START_ADDRESS + file_contents.len())],
            file_contents
        );
    }

    #[test]
    fn test_load_game_too_long_errors() {
        // implement readable trait
        struct FakeReader;

        impl std::io::Read for FakeReader {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
                Ok(1)
            }
        }
        let mut fake_reader = FakeReader{};

        let mut cpu = CPU::initialize();
        assert!(cpu.load_game(&mut fake_reader).is_err())
    }
}
