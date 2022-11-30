use std::io::Read;

// CHIP-8 has 4k memory
const CHIP_8_MEMORY_SIZE: usize = 4096;
const APPLICATION_START_ADDRESS: usize = 0x200;
const MAX_GAME_SIZE: usize = (CHIP_8_MEMORY_SIZE - APPLICATION_START_ADDRESS) as usize;

pub trait CPU {
    fn load_game(&mut self, game: &mut dyn Read) -> Result<(), std::io::Error>;
    fn fetch_current_opcode(&self) -> u16;
    fn goto(&mut self, address: u16);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chip8CPU {
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

impl Chip8CPU {
    pub fn initialize() -> Self {
        Chip8CPU {
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
}

impl CPU for Chip8CPU {
    fn load_game(&mut self, game: &mut dyn Read) -> Result<(), std::io::Error> {
        let read_count: usize;
        // TODO After reading take, try reading more - input buffer should be consumed
        match game
            .take(MAX_GAME_SIZE as u64)
            .read(&mut self.memory[APPLICATION_START_ADDRESS..])
        {
            Err(e) => return Err(e),
            Ok(c) => read_count = c,
        };
        // If we have filled the memory, see if there's more to read
        if read_count == MAX_GAME_SIZE {
            match game.take(1).read(&mut [0, 1]) {
                Err(_e) => Ok(()), // Should be no more data to read
                Ok(_c) => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Input game too big, is this a CHIP-8 game?",
                )),
            }
        } else {
            Ok(())
        }
    }

    fn fetch_current_opcode(&self) -> u16 {
        (self.memory[self.program_counter as usize] as u16) << 8
            | self.memory[(self.program_counter + 1) as usize] as u16
    }

    fn goto(&mut self, address: u16) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_resets_program_counter() {
        assert_eq!(Chip8CPU::initialize().program_counter, 0x200);
    }

    #[test]
    fn test_initialize_zeros_memory() {
        assert_eq!(Chip8CPU::initialize().memory[0x200..0x210], [0; 0x10]);
    }

    #[test]
    fn test_load_game_populates_momeory() {
        use std::io::Cursor;
        let fake_game = [1,2,3,4];
        let mut fake_game_cursor = Cursor::new(fake_game);
        struct FakeReader;

        impl std::io::Read for FakeReader {
            fn read(&mut self, _buf: &mut [u8]) -> Result<usize, std::io::Error> {
                Ok(12)
            }
        }
        let mut fake_reader = FakeReader {};

        let mut cpu = Chip8CPU::initialize();
        cpu.load_game(&mut fake_game_cursor).unwrap();

        assert_eq!(
            cpu.memory
                [APPLICATION_START_ADDRESS..(APPLICATION_START_ADDRESS + fake_game.len())],
            fake_game
        );
    }

    #[test]
    fn test_load_game_too_long_errors() {
        // implement readable trait
        struct FakeReader;

        impl std::io::Read for FakeReader {
            fn read(&mut self, _buf: &mut [u8]) -> Result<usize, std::io::Error> {
                Ok(MAX_GAME_SIZE)
            }
        }
        let mut fake_reader = FakeReader {};

        let mut cpu = Chip8CPU::initialize();
        assert!(cpu.load_game(&mut fake_reader).is_err())
    }

    #[test]
    fn test_fetch_opcode() {
        // see "fetch opcode" in https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
        let mut cpu = Chip8CPU::initialize();
        let fake_game = vec![0xA2, 0xF0];
        let mut game_cursor = std::io::Cursor::new(fake_game);
        cpu.load_game(&mut game_cursor);

        assert_eq!(cpu.fetch_current_opcode(), 0xA2F0);
    }
}
