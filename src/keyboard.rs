use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Keyboard {
    pump: sdl2::EventPump
}

impl Keyboard {
    pub fn new(sdl_context: &sdl2::Sdl) -> Keyboard {
        Keyboard { pump: sdl_context.event_pump().unwrap() }
    }

    pub fn get_keys(&mut self) -> Option<[bool; 16]> {
        // if user hits quit or esc, return an Err and exit in main
        // TODO: uncouple this from main?
        for event in self.pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return None
            }
        }

        // borrowed from a gba emulator
        // gets keyboard and converts to scancode
        let keys: Vec<Keycode> = self.pump.keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut keyboard = [false; 0x10];
        for key in keys {
            let i = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
            
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
            
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),

                Keycode::Z => Some(0xA),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xB),
                // side keys
                Keycode::Num4 => Some(0xC),
                Keycode::R => Some(0xD),
                Keycode::F => Some(0xE),
                Keycode::V => Some(0xF),
                _ => None
            };

            if let Some(key_spot) = i {
                keyboard[key_spot] = true;
            }

        }
        Some(keyboard)
    }
}