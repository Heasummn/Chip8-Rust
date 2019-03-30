use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use CHIP8_WIDTH;
use CHIP8_HEIGHT;

const SCALE: u32 = 20;
const HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE;
const WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE;

pub struct Graphics {
    screen: Canvas<Window>
}

impl Graphics {

    pub fn new(sdl_context: &sdl2::Sdl) -> Graphics {
        let video_sub = sdl_context.video().unwrap();
        let window = video_sub
            .window(
                "Chip-8 Emulator",
                WIDTH,
                HEIGHT
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut screen = window.into_canvas().build().unwrap();

        screen.set_draw_color(pixels::Color::RGB(0, 0, 0));
        screen.clear();
        screen.present();

        Graphics { screen }

    }

    pub fn draw(&mut self, screen: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for (y, row) in screen.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = x as u32 * SCALE;
                let y = y as u32 * SCALE;

                let mut color = pixels::Color::RGB(0, 0, 0);
                if col == 1 {
                    color = pixels::Color::RGB(0, 250, 0);
                }
                self.screen.set_draw_color(color);

                let _ = self.screen.fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE));
            }
        }

        self.screen.present();
    }
    
}