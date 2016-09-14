pub enum Instruction {
    // V[reg] += byte
    AddO {reg: u8, byte: u8},
    Add {regx: u8, regy: u8},
    // Jump to location
    Jmp {location: u16},
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
        // 0x1nnn -> PC = nnn
        0x1     => Instruction::Jmp { location: nnn(op) },

        // 0x7xkk -> Vx += kk
        0x7     => Instruction::AddO { reg: x(op), byte: kk(op) },

        // 0x8xyn -> Binary op
        0x8     => {
            let x = x(op);
            let y = y(op);
            match low(op) {
                0x4     => Instruction::Add {regx: x, regy: y},
                _       => Instruction::Unknown
            }
        }
        _       => Instruction::Unknown
    }
}