mod cpu;
use cpu::Cpu;

use std::io::Read;
use std::fs::File;

fn read_rom(filename: &str, rom: &mut Vec<u8>) {
    let mut rom_data = match File::open(filename) {
        Ok(data) => data,
        Err(_) => panic!("Rom file {} not found.", filename),
    };

    rom_data.read_to_end(rom).unwrap();
}

fn main() {
    println!("Hello, world!");

    let processor = Cpu::new();

    processor.dump();
    let filename = "roms/PONG";
    let mut rom = Vec::new();

    read_rom(filename, &mut rom);

    for byte in rom {
        println!("{}", byte);
    }
}
