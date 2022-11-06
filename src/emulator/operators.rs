use super::opcode_decoder::Opcode;
use super::CPU;

pub fn perform_goto(mut cpu: &CPU, opcode: Opcode) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perform_goto_given_cpu_does_not_panic() {
        let cpu = CPU::initialize();
        perform_goto(&cpu, Opcode::Goto(0));
        todo!();
    }
}
