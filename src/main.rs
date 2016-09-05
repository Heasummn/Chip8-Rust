mod cpu;
use cpu::Cpu;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::Read;
use std::fs::File;

fn read_rom(filename: &str, rom: &mut Vec<u8>) {
    let mut rom_data = match File::open(filename) {
        Ok(data) => data,
        Err(_) => {
            error!("Rom file {} not found.", filename);
            std::process::exit(1)
        },
    };

    rom_data.read_to_end(rom).unwrap();
}

fn main() {
    env_logger::init().unwrap();
    println!("Hello, world!");

    let mut processor = Cpu::new();

    processor.dump();
    let filename = "roms/PONG";
    let mut rom = Vec::new();

    read_rom(filename, &mut rom);
    processor.load_rom(rom);
}
