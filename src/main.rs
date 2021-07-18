mod chip8;
use std::time::Duration;
use std::env;
extern crate minifb;

use minifb::{Key, Window, WindowOptions, Scale};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rom = if args.len() > 1 {args[1].as_str()}else {"pong.rom"};

    let mut cpu = chip8::create_cpu();
    cpu.initialize();
    cpu.load_rom(rom).expect("Unable to load ROM");

    let mut window = Window::new(
        "Chip-8 - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(Duration::from_millis(1000/90)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        window.get_keys().map(|keys| {
            cpu.keys = [false;16];
            for t in keys {
                match t {
                    Key::Key1 => cpu.keys[0x1] = true,
                    Key::Key2 => cpu.keys[0x2] = true,
                    Key::Key3 => cpu.keys[0x3] = true,
                    Key::Key4 => cpu.keys[0xc] = true,
                    Key::A => cpu.keys[0x4] = true,
                    Key::Z => cpu.keys[0x5] = true,
                    Key::E => cpu.keys[0x6] = true,
                    Key::R => cpu.keys[0xd] = true,
                    Key::Q => cpu.keys[0x7] = true,
                    Key::S => cpu.keys[0x8] = true,
                    Key::D => cpu.keys[0x9] = true,
                    Key::F => cpu.keys[0xe] = true,
                    Key::W => cpu.keys[0xa] = true,
                    Key::X => cpu.keys[0x0] = true,
                    Key::C => cpu.keys[0xb] = true,
                    Key::V => cpu.keys[0xf] = true,
                    _ => (),
                }
            }
        });

        cpu.do_cycle();
        
        window
            .update_with_buffer(&cpu.screen, 64, 32)
            .unwrap();
    }
}
