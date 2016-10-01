use instruction;
use instruction::Instruction::*;
use instruction::Instruction;
use std::io::{BufWriter, Write};

pub struct Cpu {
    opcode      : u16,
    memory      : [u8; 4096],
    registers   : [u8; 16],
    i           : u16,
    pc          : u16,
    sp          : u16,
    stack       : [u16; 16]
}

impl Cpu
{
    pub fn new() -> Cpu {
        Cpu {
            opcode: 0,
            memory: [0; 4096],
            registers: [0; 16],
            i: 0,
            pc: 512,
            sp: 0,
            stack: [0; 16]
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        {
            // TODO: Use constants
            let mut ram = BufWriter::new(&mut self.memory[0x200..4096]);
            match ram.write_all(rom.as_ref()) {
                Ok(x)   => x,
                Err(_)  => panic!("Internal Error!")
            }
        }

        if rom.len() > 4096 - 0x200 {
            panic!("ROM is too big!")
        }

    }

    pub fn dump(&self) {
        println!("opcode: {:#02X}", self.opcode);

        for i in 0..16 {
            print!("V{:x}: {}", i, self.registers[i]);
            if i != 15 {
                print!(", ")
            }
        }
        println!("");

        println!("I: {}", self.i);
        println!("PC: {}", self.pc);
        println!("SP: {}", self.sp);

        for i in 0..16 {
            print!("S{:x}: {}", i, self.stack[i]);
            if i != 15 {
                print!(", ")
            }
        }
        println!("")
    }

    fn run_op(&mut self, instr: Instruction) {
        match instr {
            AddO {reg, byte} => {
                    let reg = &mut self.registers[reg as usize];
                    *reg = reg.wrapping_add(byte);
            },
            Load {regx, regy} => {
                self.registers[regx as usize] = self.registers[regy as usize];
            },
            Or {regx, regy} => {
                self.registers[regx as usize] |= self.registers[regy as usize];
            }
            And {regx, regy} => {
                self.registers[regx as usize] &= self.registers[regy as usize];
            }

            Xor {regx, regy} => {
                self.registers[regx as usize] ^= self.registers[regy as usize];
            }

            Add {regx, regy} => {
                let x = self.registers[regx as usize] as u16;
                let y = self.registers[regy as usize] as u16;
                let result = x.wrapping_add(y);
                self.registers[0xF] = (result > 255) as u8;
                // as u8 casts down to a byte to make sure we don't overflow
                self.registers[regx as usize] = result as u8;
            },
            Sub {regx, regy} => {
                let x = self.registers[regx as usize] as u16;
                let y = self.registers[regy as usize] as u16;
                let result = x.wrapping_sub(y);
                self.registers[0xF] = (x > y) as u8;
                self.registers[regx as usize] = result as u8;
            },
            Shr {reg} => {
                let x = self.registers[reg as usize] as u16;
                let result = x >> 1;
                self.registers[0xF] = (x & 1) as u8;
                self.registers[reg as usize] = result as u8;
            },
            Subn {regx, regy} => {
                let x = self.registers[regx as usize] as u16;
                let y = self.registers[regy as usize] as u16;
                let result = y.wrapping_sub(x);
                self.registers[0xF] = (y > x) as u8;
                self.registers[regx as usize] = result as u8;
            },
            Shl {reg} => {
                let x = self.registers[reg as usize] as u16;
                let result = x << 1;
                self.registers[0xF] = (x >> 7) as u8;
                self.registers[reg as usize] = result as u8;
            },
            Sne {regx, regy} => {
                let x = self.registers[regx as usize] as u16;
                let y = self.registers[regy as usize] as u16;
                if x != y {
                    self.pc += 2;
                }
            },
            LdI {loc}       => { self.i = loc },
            JmpA {loc}      => { self.pc = self.registers[0] as u16 + loc},
            AddI {reg}      => { self.i = self.i.wrapping_add(self.registers[reg as usize] as u16) },
            LdF {reg}       => { self.i = (self.registers[reg as usize] as u16) * 5 },
            LdB {reg}       => {
                let i = self.i as usize;
                let x = self.registers[reg as usize];
                self.memory[i] = x.wrapping_div(100);
                self.memory[i + 1] = (x.wrapping_div(10)) % 10;
                self.memory[i + 2] = (x % 100) % 10;
            }
            Jmp {location}  => { self.pc = location },
            Unknown         => ()
        }
    }

    pub fn execute(&mut self) {
        loop {
            let pc = self.pc as usize;
            if self.memory.len() <= pc {
                break;
            }
            self.opcode = (self.memory[pc] as u16) << 8 | self.memory[pc + 1] as u16;
            let instr = instruction::convert_op(self.opcode);
            self.run_op(instr);
            self.pc += 2;
        }

    }

}

#[cfg(test)]
mod tests {
    use cpu::Cpu;
    use instruction::Instruction::*;

    fn start() -> Cpu
    {
        let mut cpu = Cpu::new();
        cpu.run_op(AddO {reg: 1, byte: 20 });
        cpu.run_op(AddO {reg: 7, byte: 12 });
        cpu.i = 7;
        cpu.registers[0] = 35;
        cpu
    }

    #[test]
    fn test_addo() {
        let mut processor = start();
        processor.run_op(AddO {reg: 4, byte: 7 });
        assert_eq!(processor.registers[4], 7);
        processor.run_op(AddO {reg: 4, byte: 25});
        assert_eq!(processor.registers[4], 32);
    }

    #[test]
    fn test_add() {
        let mut processor = start();

        processor.run_op(Add { regx: 4, regy: 7 }); // V4 += V7
        assert_eq!(processor.registers[4], processor.registers[7]); // V4 == V7
        assert_eq!(processor.registers[4], 12); // V4 == 12
    }

    #[test]
    fn test_sub() {
        let mut processor = start();
        processor.run_op(Sub {regx: 1, regy: 7}); // V1 -= V7
        assert_eq!(processor.registers[0xf], 1);
        assert_eq!(processor.registers[1], 8);
    }

    #[test]
    fn test_shr() {
        let mut processor = start();
        processor.run_op(Shr {reg: 1 }); // V1 >> 1
        assert_eq!(processor.registers[1], 10);
        assert_eq!(processor.registers[0xF], 0)
    }

    #[test]
    fn test_subn() {
        let mut processor = start();
        processor.run_op(Subn {regx: 1, regy: 7}); // V1 = V7 - V1
        assert_eq!(processor.registers[7], 12);
        assert_eq!(processor.registers[0xF], 0);
    }

    #[test]
    fn test_shl() {
        let mut processor = start();
        processor.run_op(Shl {reg: 1}); // V1 << 1
        assert_eq!(processor.registers[1], 40);
        assert_eq!(processor.registers[0xF], 0);
    }

    #[test]
    fn test_sne() {
        let mut processor = start();
        let pc = processor.pc;
        processor.run_op(Sne {regx: 1, regy: 1}); // Nothing
        assert_eq!(processor.pc, pc);
        processor.run_op(Sne {regx: 1, regy: 7}); // PC += 2
        assert_eq!(processor.pc, pc + 2);
    }

    #[test]
    fn test_ld_i() {
        let mut processor = start();
        processor.run_op(LdI {loc: 1024});
        assert_eq!(processor.i, 1024)
    }

    #[test]
    fn test_jmp_a() {
        let mut processor = start();
        processor.run_op(JmpA {loc: 1024});
        assert_eq!(processor.pc, 1024 + processor.registers[0] as u16)
    }

    #[test]
    fn test_ld_f() {
        let mut processor = start();
        processor.run_op(LdF {reg: 1});
        assert_eq!(processor.i, 100);
    }

    #[test]
    fn test_ld_b() {
        let mut processor = start();
        processor.run_op(LdB {reg: 7});
        let i = processor.i as usize;
        assert_eq!(processor.memory[i], 0); // 7 is loaded with 12, the first num of 12 is 0
        assert_eq!(processor.memory[i + 1], 1);
        assert_eq!(processor.memory[i + 2], 2);
    }

    #[test]
    fn test_jmp() {
        let mut processor = start();
        processor.run_op(Jmp {location: 756});
        assert_eq!(processor.pc, 756);
    }

    #[test]
    fn test_ld() {
        let mut processor = start();
        processor.run_op(Load {regx: 1, regy: 7});
        assert_eq!(processor.registers[1], processor.registers[7]);
    }

    #[test]
    fn test_or() {
        let mut processor = start();
        // perform 20 | 12
        processor.run_op(Or {regx: 7, regy: 1});
        assert_eq!(processor.registers[7], 28);

    }

    #[test]
    fn test_and() {
        let mut processor = start();
        // perform 20 & 12
        processor.run_op(And {regx: 1, regy: 7});
        assert_eq!(processor.registers[1], 0x4);
    }

    #[test]
    fn test_xor() {
        let mut processor = start();
        // perform 20 ^ 12
        processor.run_op(Xor {regx: 1, regy: 7});
        assert_eq!(processor.registers[1], 24);
    }
}