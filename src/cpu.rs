use rand::Rng;

use instruction;
use instruction::Instruction::*;
use instruction::Instruction;
use std::io::{BufWriter, Write};

use CHIP8_HEIGHT;
use CHIP8_WIDTH;

pub struct ExecutionState {
    pub finished: bool,
    pub screen: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub drawn: bool
}

pub struct Cpu {
    drawn       : bool,
    opcode      : u16,
    memory      : [u8; 4096],
    registers   : [u8; 16],
    i           : u16,
    pc          : u16,
    sp          : u16,
    stack       : [u16; 16],
    screen      : [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    keys        : [bool; 16],
    delay_timer : u8,
    rand_gen    : rand::rngs::ThreadRng,
}

impl Cpu
{
    pub fn new() -> Cpu {
        Cpu {
            drawn: false,
            opcode: 0,
            memory: [0; 4096],
            registers: [0; 16],
            i: 0,
            pc: 512,
            sp: 0,
            stack: [0; 16],
            screen: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            keys: [false; 16],
            delay_timer: 0,
            rand_gen: rand::thread_rng()
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        if rom.len() > 4096 - 0x200 {
            panic!("ROM is too big!")
        }

        {
            // TODO: Use constants
            let mut ram = BufWriter::new(&mut self.memory[0x200..4096]);
            match ram.write_all(rom.as_ref()) {
                Ok(_)   => (),
                Err(_)  => panic!("Internal Error!")
            }
        }

        {
            let fonts = [
                0xF0, 0x90, 0x90, 0x90, 0xF0,
                0x20, 0x60, 0x20, 0x20, 0x70,
                0xF0, 0x10, 0xF0, 0x80, 0xF0,
                0xF0, 0x10, 0xF0, 0x10, 0xF0,
                0x90, 0x90, 0xF0, 0x10, 0x10,
                0xF0, 0x80, 0xF0, 0x10, 0xF0,
                0xF0, 0x80, 0xF0, 0x90, 0xF0,
                0xF0, 0x10, 0x20, 0x40, 0x40,
                0xF0, 0x90, 0xF0, 0x90, 0xF0,
                0xF0, 0x90, 0xF0, 0x10, 0xF0,
                0xF0, 0x90, 0xF0, 0x90, 0x90,
                0xE0, 0x90, 0xE0, 0x90, 0xE0,
                0xF0, 0x80, 0x80, 0x80, 0xF0,
                0xE0, 0x90, 0x90, 0x90, 0xE0,
                0xF0, 0x80, 0xF0, 0x80, 0xF0,
                0xF0, 0x80, 0xF0, 0x80, 0x80,
            ];
            let mut sprites = BufWriter::new(&mut self.memory[0x000..0x200]);
            match sprites.write_all(&fonts) {
                Ok(_) => (),
                Err(_) => ()
            }
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

    // TODO: On jumps we currently decrement by 2 bc the tick function handles incremnting. Make this nicer?
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
            ConstantSe {reg, byte} => {
                let reg = self.registers[reg as usize];
                if reg == byte {
                    self.pc += 2;
                }
            },
            ConstantSne {reg, byte} => {
                let reg = self.registers[reg as usize];
                if reg != byte {
                    self.pc += 2;
                }
            },
            Shl {reg} => {
                let x = self.registers[reg as usize] as u16;
                let result = x << 1;
                self.registers[0xF] = (x >> 7) as u8;
                self.registers[reg as usize] = result as u8;
            },
            Se {x, y} => {
                let x = self.registers[x as usize];
                let y = self.registers[y as usize];
                if x == y {
                    self.pc += 2;
                }
            },
            Sne {regx, regy} => {
                let x = self.registers[regx as usize] as u16;
                let y = self.registers[regy as usize] as u16;
                if x != y {
                    self.pc += 2;
                }
            },
            LdI {loc}       => { self.i = loc },
            SetLong {reg}   => {
                for i in 0..reg+1 {
                    self.memory[self.i as usize + i as usize] = self.registers[i as usize];
                }
            },
            LdFont {reg}    => {
                // each digit is 5 bytes, starting at 0
                // memory location is then just 5 * digit
                self.i = self.registers[reg as usize] as u16 * 5;
            },
            LdLong {reg}    => {
                for i in 0..reg+1 {
                    self.registers[i as usize] = self.memory[self.i as usize + i as usize];
                }
            }
            JmpA {loc}      => { 
                self.pc = self.registers[0] as u16 + loc;
                self.pc -= 2;
            },
            AddI {reg}      => { self.i = self.i.wrapping_add(self.registers[reg as usize] as u16) },
            LdB {reg}       => {
                let i = self.i as usize;
                let x = self.registers[reg as usize];
                self.memory[i] = x.wrapping_div(100);
                self.memory[i + 1] = (x.wrapping_div(10)) % 10;
                self.memory[i + 2] = (x % 100) % 10;
            },
            LdDelay {reg}   => {
                self.registers[reg as usize] = self.delay_timer;
            },
            SetDelay {reg}  => {
                self.delay_timer = self.registers[reg as usize];
            },
            Jmp {location}  => { 
                self.pc = location;
                self.pc -= 2; // tick will increment
            },
            Call {location} => {
                // need to move forward one otherwise infinite recursion
                self.stack[self.sp as usize] = self.pc + 2;
                self.sp += 1;
                self.pc = location;
                self.pc -= 2; // tick will increment
            },
            Ret => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                self.pc -= 2; // tick will increment
            },
            Set {reg, byte} => {
                self.registers[reg as usize] = byte;
            },
            Skp {key} => {
                if self.keys[self.registers[key as usize] as usize] {
                    self.pc += 2;
                }
            },
            Sknp {key} => {
                if !self.keys[self.registers[key as usize] as usize] {
                    self.pc += 2;
                }
            },
            Random {reg, byte} => {
                let rand_num = self.rand_gen.gen_range(0, 255);
                self.registers[reg as usize] = byte & rand_num;
            },
            Draw {x, y, n}  => {
                self.drawn = true;
                self.registers[0x0F] = 0;

                for byte in 0..n {
                    let y = (self.registers[y as usize] as usize + byte as usize) % CHIP8_HEIGHT; // we love casting
                    for bit in 0..8 {
                        let x = (self.registers[x as usize] as usize + bit) % CHIP8_WIDTH;

                        // check the correct bit of the byte storing color as 1 or 0
                        let color = (self.memory[self.i as usize + byte as usize] >> (7 - bit)) & 1;

                        if self.screen[y as usize][x as usize] == 1 && color == 1 {
                            self.registers[0x0F] = 1;
                        }
                        self.screen[y as usize][x as usize] ^= color;
                    }
                }
                
            },
            Clear => {
                self.drawn = true;
                self.screen = [[0; CHIP8_WIDTH]; CHIP8_HEIGHT];
            }
            Unknown         => ()
        }
    }

    pub fn step(&mut self, keys: [bool; 16]) -> ExecutionState {
        self.keys = keys;
        self.drawn = false;
        let pc = self.pc as usize;
        self.opcode = (self.memory[pc] as u16) << 8 | self.memory[pc + 1] as u16;
        let instr = instruction::convert_op(self.opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        self.run_op(instr);
        self.pc += 2;
        
        let finished = self.memory.len() <= self.pc as usize;
        return ExecutionState {
            finished: finished,
            screen: self.screen,
            drawn: self.drawn
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
        assert_eq!(processor.pc, 1022 + processor.registers[0] as u16)
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
        assert_eq!(processor.pc, 754);
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