#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    // PC = location
    Jmp {location: u16},

    // V[reg] += byte
    AddO {reg: u8, byte: u8},

    // V[x] = V[y]
    Load {regx: u8, regy: u8},

    // V[x] |= V[y]
    Or {regx: u8, regy: u8},

    // V[x] ^= V[y]
    Xor {regx: u8, regy: u8},

    // V[x] &= V[y]
    And {regx: u8, regy: u8},

    // V[x] += V[y]
    Add {regx: u8, regy: u8},

    Unknown
}

// 0xF123 -> 0xF
fn high(op: u16) -> u16 {
    (op & 0xF000) >> 12
}

// 0xF123 -> 0x3
fn low(op: u16) -> u16 {
    (op & 0xF)
}

// 0xF123 -> 0x1
fn x(op: u16) -> u8 {
    ((op & 0x0F00) >> 8) as u8
}

// 0xF123 -> 0x2
fn y(op: u16) -> u8 {
    ((op & 0x00F0) >> 4) as u8
}

// 0xF123 -> 0x23
fn kk(op: u16) -> u8 {
    (op & 0x00FF) as u8
}

fn nnn(op: u16) -> u16 {
    (op & 0x0FFF)
}

pub fn convert_op(op: u16) -> Instruction {
    match high(op) {
        // 0x1nnn
        0x1     => Instruction::Jmp { location: nnn(op) },

        // 0x7xkk
        0x7     => Instruction::AddO { reg: x(op), byte: kk(op) },

        // 0x8xyN -> Binary ops
        0x8     => {
            let x = x(op);
            let y = y(op);
            match low(op) {
                // 0x8xy0
                0x0     => Instruction::Load{regx: x, regy: y},
                // 0x8xy1
                0x1     => Instruction::Or{regx: x, regy: y},
                // 0x8xy2
                0x2     => Instruction::And{regx: x, regy: y},
                // 0x8xy3
                0x3     => Instruction::Xor{regx: x, regy: y},
                // 0x8xy4
                0x4     => Instruction::Add {regx: x, regy: y},

                _       => Instruction::Unknown
            }
        }
        _       => Instruction::Unknown
    }
}

#[cfg(test)]
mod tests {
    use instruction::Instruction::*;
    use instruction::convert_op;

    #[test]
    fn test_conv_addo() {
        let instr = convert_op(0x7532);
        assert_eq!(instr, AddO{reg: 5, byte: 0x32})
    }

    #[test]
    fn test_conv_jmp() {
        let instr = convert_op(0x1501);
        assert_eq!(instr, Jmp{location: 0x501})
    }

    #[test]
    fn test_conv_binop() {
        // All bin ops are handled the same way
        let instr = convert_op(0x8354);
        assert_eq!(instr, Add{regx: 0x3, regy: 0x5})
    }
}