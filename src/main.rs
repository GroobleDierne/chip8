mod chip8;
use std::thread::sleep;
use std::time::Duration;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rom = if args.len() > 1 {args[1].as_str()}else {"pong.rom"};

    let mut cpu = chip8::create_cpu();
    cpu.initialize();
    cpu.load_rom(rom).expect("Unable to load ROM");

    loop {
        cpu.do_cycle();
        sleep(Duration::from_millis(1000/75));
    }
}
