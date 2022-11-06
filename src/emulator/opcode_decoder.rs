#[derive(Debug)]
pub enum Opcode {
    Goto,
}

pub struct OpcodeDecoder {}

pub trait OpcodeDecoderTrait {
    fn decode_opcode(opcode_value: u16) -> Opcode;
}

impl OpcodeDecoder {
    pub fn decode_opcode(opcode_value: u16) -> Opcode {
        Opcode::Goto
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_opcode_given_goto_returns_goto_enum() {
        assert!(matches!(OpcodeDecoder::decode_opcode(0x1000), Opcode::Goto));
    }
}
