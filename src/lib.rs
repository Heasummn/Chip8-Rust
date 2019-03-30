pub mod cpu;
pub mod instruction;
pub mod graphics;
pub mod keyboard;
extern crate sdl2;
extern crate rand;

pub const CHIP8_HEIGHT: usize = 32;
pub const CHIP8_WIDTH: usize = 64;