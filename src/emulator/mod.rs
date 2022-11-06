use std::path::Path;

use super::cpu::CPU;

pub mod opcode_decoder;
use opcode_decoder::OpcodeDecoder;

mod operators;

pub struct Emulator<'a> {
    pub cpu: &'a mut CPU,
    pub opcode_decoder: OpcodeDecoder,
}

impl<'a> Emulator<'a> {
    pub fn load_game_from_file(&mut self, game_path: &Path) -> Result<(), std::io::Error> {
        self.cpu.load_game_from_file(game_path)
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
    fn test_emulate_cycle_loads_and_processes_goto_opcode() {
        let mut cpu = CPU::initialize();
        let fake_game = vec![0x10, 0x11];
        let mut game_cursor = std::io::Cursor::new(fake_game);
        cpu.load_game(&mut game_cursor);

        let expected_cpu_state = cpu.clone();
        operators::perform_goto(
            &expected_cpu_state,
            opcode_decoder::OpcodeDecoder::decode_opcode(cpu.fetch_current_opcode()),
        );

        {
            Emulator {
                cpu: &mut cpu,
                opcode_decoder: OpcodeDecoder {},
            }
            .emulate_cycle();
        }

        assert_eq!(expected_cpu_state, cpu);
    }
}
