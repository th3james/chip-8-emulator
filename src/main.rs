use std::path::Path;

mod cpu;
mod emulator;

fn main() {
    let mut cpu = cpu::Chip8CPU::initialize();
    let mut emulator = emulator::Emulator {
        cpu: &mut cpu,
        opcode_decoder: emulator::opcode_decoder::OpcodeDecoder {},
    };

    match emulator.load_game_from_file(Path::new("./test_fixtures/test_game.ch8")) {
        Ok(()) => (),
        Err(e) => panic!("Error loading game: {}", e),
    };

    println!("Game loaded successfully");

    emulator.start_emulation();
}
