#[derive(Debug)]
#[derive(PartialEq)]
pub enum Opcode {
    Goto(u16),
}

pub struct OpcodeDecoder {}

pub trait OpcodeDecoderTrait {
    fn decode_opcode(opcode_value: u16) -> Opcode;
}

impl OpcodeDecoder {
    pub fn decode_opcode(opcode_value: u16) -> Opcode {
        Opcode::Goto(opcode_value & 0x0FFF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_opcode_given_goto_returns_goto_enum() {
        assert_eq!(
            OpcodeDecoder::decode_opcode(0x1000),
            Opcode::Goto(0x000)
        );
        assert_eq!(
            OpcodeDecoder::decode_opcode(0x100F),
            Opcode::Goto(0x00F)
        );
        assert_eq!(
            OpcodeDecoder::decode_opcode(0x1320),
            Opcode::Goto(0x320)
        );
    }
}
