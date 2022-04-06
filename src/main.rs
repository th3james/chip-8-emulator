//use std::path::Path;

mod cpu;


fn main() {
    let cpu = cpu::CPU::initialize();

    //cpu::load_game(Path::new("../test_fixtures/test_game.ch8"));

    println!("Hello, world!");
}
