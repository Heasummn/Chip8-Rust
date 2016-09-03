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

    #[allow(dead_code)]
    pub fn dump(self) {
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
