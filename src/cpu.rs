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
            Add {reg, byte} => {
                {
                    let reg = &mut self.registers[reg as usize];
                    *reg = reg.wrapping_add(byte);
                }
            },
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
        Cpu::new()
    }

    #[test]
    fn test_add() {
        let mut processor = start();
        processor.run_op(Add {reg: 4, byte: 7 });
        assert_eq!(processor.registers[4], 7);
        processor.run_op(Add {reg: 4, byte: 25});
        assert_eq!(processor.registers[4], 32);
    }
}