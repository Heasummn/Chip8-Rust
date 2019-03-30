extern crate chip8_cpu;

use chip8_cpu::cpu::Cpu;
use chip8_cpu::graphics::Graphics;
use chip8_cpu::keyboard::Keyboard;

use std::io::Read;
use std::fs::File;

fn read_rom(filename: &str, rom: &mut Vec<u8>) {
    let mut rom_data = match File::open(filename) {
        Ok(data) => data,
        Err(_) => {
            panic!("Rom file {} not found.", filename);
        },
    };

    rom_data.read_to_end(rom).unwrap();
}

fn main() {
    // TODO: uncouple from main
    let context = sdl2::init().unwrap();
    let mut gfx = Graphics::new(&context);
    let mut kb = Keyboard::new(&context);

    let mut processor = Cpu::new();

    let filename = "roms/BRIX";
    {
        let mut rom = Vec::new();

        read_rom(filename, &mut rom);
        processor.load_rom(rom);
    }

    loop {
        let keys = kb.get_keys();
        if keys.is_none() {
            break;
        }
        let state = processor.step(keys.unwrap());
        if state.finished {
            break;
        }
        if state.drawn {
            gfx.draw(&state.screen);
        }
        //processor.dump();
    }
}
