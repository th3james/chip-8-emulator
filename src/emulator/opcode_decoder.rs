#[derive(Debug)]
#[derive(PartialEq)]
pub enum Opcode {
    Unhandled(u16),
    Goto(u16),
    SkipIfRegisterEqLiteral(u8, u8),
}

pub struct OpcodeDecoder {}

pub trait OpcodeDecoderTrait {
    fn decode_opcode(opcode_value: u16) -> Opcode;
}

impl OpcodeDecoder {
    pub fn decode_opcode(opcode_value: u16) -> Opcode {
        // TODO bitwise match might be cleaner
        if opcode_value >= 0x1000 && opcode_value < 0x2000 {
            Opcode::Goto(opcode_value & 0x0FFF)
        } else if opcode_value >= 0x3000 && opcode_value < 0x4000 {
            Opcode::SkipIfRegisterEqLiteral(
                ((opcode_value & 0x0F00) >> 8) as u8,
                (opcode_value & 0x00FF) as u8,
            )
        } else {
            Opcode::Unhandled(opcode_value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_opcode_returns_unimplemented_opcode() {
        assert_eq!(
            OpcodeDecoder::decode_opcode(0x0000),
            Opcode::Unhandled(0x0000)
        );
    }

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

    #[test]
    fn test_decode_opcode_skip_if_register_equal_literal() {
        assert_eq!(
            OpcodeDecoder::decode_opcode(0x3101),
            Opcode::SkipIfRegisterEqLiteral(0x1, 0x01)
        );
        assert_eq!(
            OpcodeDecoder::decode_opcode(0x3A2A),
            Opcode::SkipIfRegisterEqLiteral(0xA, 0x2A)
        );
    }
}
