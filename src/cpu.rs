use std::io;
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
            pc: 0,
            sp: 0,
            stack: [0; 16]
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) -> Result<u32, io::Error>
    {
        {
            // TODO: Use constants
            let mut ram = BufWriter::new(&mut self.memory[0x200..4096]);
            try!(ram.write_all(rom.as_ref()));
        }

        if rom.len() > 4096 - 0x200 {
            error!("ROM is too big!")
        }

        let mut i = 0;
        for byte in self.memory.iter() {
            if i > 512 {
                debug!("{:#08X}", byte);
            }
            i = i + 1;
        }

        return Ok(1);
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        println!("opcode: {}", self.opcode);
        for i in 0..16 {
            print!("V{}: {}", i, self.registers[i]);
            if i != 15 {
                print!(", ")
            }
        }
        println!("");
        println!("I: {}", self.i);
        println!("PC: {}", self.pc);
        println!("SP: {}", self.sp);
    }

}
