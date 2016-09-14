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
            Add {regx, regy} => {
                let x = self.registers[regx as usize] as u16;
                let y = self.registers[regy as usize] as u16;
                let result = x + y;
                self.registers[0xF] = (result > 255) as u8;
                // as u8 casts down to a byte to make sure we don't overflow
                self.registers[regx as usize] = result as u8;
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
        processor.run_op(AddO {reg: 7, byte: 12}); // Set V7 to 12;
        processor.run_op(Add {regx: 4, regy: 7}); // V4 += V7
        assert_eq!(processor.registers[4], processor.registers[7]); // V4 == V7
        assert_eq!(processor.registers[4], 12); // V4 == 12
    }
}