use std::path::Path;

mod cpu;


fn main() {
    let mut cpu = cpu::CPU::initialize();

    match cpu.load_game_from_file(Path::new("./test_fixtures/test_game.ch8")) {
        Ok(()) => (),
        Err(e) => panic!("Error loading game: {}", e)
    };

    println!("Game loaded successfully");

    cpu.start_emulation();

}
