use super::opcode_decoder::Opcode;
use super::CPU;

pub fn perform_goto(mut cpu: &dyn CPU, opcode: Opcode) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_perform_goto_sets_pc_to_goto_value() {
        let cpu = CPU::initialize();
        perform_goto(&cpu, Opcode::Goto(153));
        assert_eq!(cpu.program_counter, 153);
    }
}
