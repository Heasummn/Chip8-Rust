extern crate chip8_cpu;

use chip8_cpu::cpu::Cpu;

use std::io::Read;
use std::fs::File;

fn read_rom(filename: &str, rom: &mut Vec<u8>) {
    let mut rom_data = match File::open(filename) {
        Ok(data) => data,
        Err(_) => {
            panic!("Rom file {} not found.", filename);
            std::process::exit(1)
        },
    };

    rom_data.read_to_end(rom).unwrap();
}

fn main() {
    let mut processor = Cpu::new();

    let filename = "roms/PONG";
    {
        let mut rom = Vec::new();

        read_rom(filename, &mut rom);
        processor.load_rom(rom);
    }

    processor.execute();
}
