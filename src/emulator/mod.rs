use std::path::Path;
use std::fs::File;
use std::io::Read;

use super::cpu::CPU;

pub mod opcode_decoder;
use opcode_decoder::OpcodeDecoder;

mod operators;

pub struct Emulator<'a> {
    pub cpu: &'a mut dyn CPU,
    pub opcode_decoder: OpcodeDecoder,
}

impl<'a> Emulator<'a> {
    pub fn load_game_from_file(&mut self, game_path: &Path) -> Result<(), std::io::Error> {
        let mut game_file = File::open(game_path).expect(
            format!(
                "Couldn't open file {}",
                game_path.canonicalize().unwrap().display()
            )
            .as_str(),
        );

        self.cpu.load_game(&mut game_file)
    }

    pub fn emulate_cycle(&mut self) {
        let opcode_value = self.cpu.fetch_current_opcode();
        let opcode = OpcodeDecoder::decode_opcode(opcode_value);
        println!("Opcode: {:?}", opcode);
    }

    pub fn start_emulation(&mut self) {
        self.emulate_cycle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_game_from_file_loads_game_from_file() {
        let test_game_path = Path::new("./test_fixtures/test_game.ch8");

        let file_contents = std::fs::read(test_game_path).expect(
            format!(
                "Couldn't read test fixture {}",
                test_game_path.canonicalize().unwrap().display()
            )
            .as_str(),
        );

        struct FakeCPU {
            read_file: Vec<u8>,
        }
        impl CPU for FakeCPU {
            fn load_game(&mut self, game_path: &mut dyn Read) -> Result<(), std::io::Error> {
                game_path.read_to_end(&mut self.read_file)?;
                Ok(())
            }

            fn fetch_current_opcode(&self) -> u16 { panic!("shouldn't be called") }
            fn goto(&mut self, _address: u16) { panic!("shouldn't be called") }
        }

        let mut cpu = Emulator {
            cpu: &mut FakeCPU { read_file: vec![] },
            opcode_decoder: OpcodeDecoder::new(),
        };
        cpu.load_game_from_file(test_game_path);

        assert_eq!(
            cpu.cpu.read_file,
            file_contents
        );
    }


    #[test]
    fn test_emulate_cycle_loads_and_processes_goto_opcode() {
        struct FakeCPU {
            received_opcode: Option<u16>,
        }
        impl CPU for FakeCPU {
            fn load_game(&mut self, game_path: &mut dyn Read) -> Result<(), std::io::Error> {
                panic!("shouldn't be called")
            }
            fn fetch_current_opcode(&self) -> u16 { 0x1011 }
            fn goto(&mut self, address: u16) { self.received_opcode = Some(address) }
        }

        let mut fake_cpu = FakeCPU {};
        let expected_goto_address = match opcode_decoder::OpcodeDecoder::decode_opcode(
            fake_cpu.fetch_current_opcode()
        ) {
            opcode_decoder::Opcode::Goto(address) => address,
            _ => panic!("Expected Goto opcode"),
        };

        Emulator {
            cpu: &mut fake_cpu,
            opcode_decoder: OpcodeDecoder {},
        }
        .emulate_cycle();

        assert_eq!(
            fake_cpu.received_opcode,
            expected_goto_address
        );
    }
}
