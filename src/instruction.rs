#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    // PC = location
    Jmp {location: u16},

    // Call; pc = nnn, sp++
    Call {location: u16},

    Ret,

    // Skip V[x] == V[y]
    Se {x: u8, y: u8},

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

    // V[x] += V[y]; VF = carry
    Add {regx: u8, regy: u8},

    // V[x] -= V[y]; VF = !borrow
    Sub {regx: u8, regy: u8},

    // V[x] >>= 1; VF = lsb(V[x])
    Shr {reg: u8},

    // V[x] = V[y] - V[x]; VF = !borrow
    Subn {regx: u8, regy: u8},

    // V[x] <<= 1; VF = msb(V[x])
    Shl {reg: u8},

    // If V[x] == byte -> pc += 2
    ConstantSe {reg: u8, byte: u8},

    // if V[x] != byte -> pc += 2
    ConstantSne {reg: u8, byte: u8},

    // If V[x] != V[y] -> pc += 2;
    Sne {regx: u8, regy: u8},

    // I = nnn
    LdI {loc: u16},

    // PC = V[0] + loc
    JmpA {loc: u16},

    // I += V[x]
    AddI {reg: u8},

    // mem[I] = V[x] / 100; mem[I + 1] = (V[x] / 10) % 10; mem[I + 2] = (V[x] % 100) % 10;
    LdB {reg: u8},

    // I = sprite of V[x]
    LdFont {reg: u8},

    // V[0..X] = mem[I..I+x]
    LdLong {reg: u8},

    // mem[I..I+x] = V[0..x]
    SetLong {reg: u8}, 

    // V[x] = byte
    Set {reg: u8, byte: u8},

    // V[x] = delay
    LdDelay {reg: u8},

    // delay = V[x]
    SetDelay {reg: u8},

    // skip if key pressed
    Skp {key: u8},

    // skip if key not pressed
    Sknp {key: u8},

    // V[x] = rand & byte
    Random {reg: u8, byte: u8},

    // Draw I at x, y
    Draw{x: u8, y: u8, n: u16},
    Clear,

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

        //0x00
        0x0     => {
            match low(op) {
                0x0 => {
                    Instruction::Clear
                },
                0xE => {
                    Instruction::Ret
                },
                _ => {println!("{:x}", op); Instruction::Unknown}
            }
        }

        // 0x1nnn
        0x1     => Instruction::Jmp { location: nnn(op) },
        
        //0x2nnn
        0x2     => Instruction::Call { location: nnn(op) },

        // 0x3xkk
        0x3     => Instruction::ConstantSe { reg: x(op), byte: kk(op) },

        // 0x4xkk
        0x4     => Instruction::ConstantSne { reg: x(op), byte: kk(op) },

        // 0x5xy0
        0x5     => Instruction::Se {x: x(op), y: y(op) },

        // 0x6xkk  
        0x6     => Instruction::Set { reg: x(op), byte: kk(op) },

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
                // 0x8xy5
                0x5     => Instruction::Sub {regx: x, regy: y},
                // 0x8x_6
                0x6     => Instruction::Shr {reg: x},
                // 0x8xy7
                0x7     => Instruction::Subn {regx: x, regy: y},
                // 0x8x_E
                0xE     => Instruction::Shl {reg: x},

                _       => {println!("{:x}", op); Instruction::Unknown}
            }
        }
        // 0x9xy0
        0x9     => {
            let x = x(op);
            let y = y(op);
            Instruction::Sne {regx: x, regy: y}
        },
        // 0xAnnn
        0xA     => {
            let nnn = nnn(op);
            Instruction::LdI{loc: nnn}
        },
        // 0xBnnn
        0xB     => {
            let nnn = nnn(op);
            Instruction::JmpA {loc: nnn}
        },

        // 0xCxkk
        0xC     => {
            Instruction::Random {reg: x(op), byte: kk(op)}
        },

        // 0xDxyn
        0xD     => {
            let x = x(op);
            let y = y(op);
            let n = low(op);
            Instruction::Draw {x, y, n}
        },
        0xE     => {
            let x = x(op);
            match kk(op) {
                0x9E => {
                    Instruction::Skp { key: x }
                },
                0xA1 => {
                    Instruction::Sknp { key: x }
                },
                _ => {
                    Instruction::Unknown
                }
            }
        },

        0xF     => {
            let x = x(op);
            match kk(op) {
                0x07        => {
                    Instruction::LdDelay {reg: x}
                },
                0x15        => {
                    Instruction::SetDelay {reg: x}
                },
                0x1E        => {
                    Instruction::AddI {reg: x}
                },
                0x29        => {
                    Instruction::LdFont {reg: x}
                }
                0x33        => {
                    Instruction::LdB {reg: x}
                },
                0x55        => {
                    Instruction::SetLong {reg: x}
                }
                0x65        => {
                    Instruction::LdLong {reg: x}
                }
                _       => {println!("{:x}", op); Instruction::Unknown}
            }
        }

        _       => {println!("{:x}", op); return Instruction::Unknown}
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

    #[test]
    fn test_conv_sne() {
        let instr = convert_op(0x9260);
        assert_eq!(instr, Sne {regx: 0x2, regy: 0x6});
    }

    #[test]
    fn test_conv_ld_i() {
        let instr = convert_op(0xA400);
        assert_eq!(instr, LdI {loc: 0x400});
    }

    #[test]
    fn test_conv_jmp_a() {
        let instr = convert_op(0xB400);
        assert_eq!(instr, JmpA {loc: 0x400});
    }

    #[test]
    fn test_conv_fx() {
        let instr = convert_op(0xF31E);
        assert_eq!(instr, AddI {reg: 3})
    }
}