mod chip8;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut cpu = chip8::create_cpu();
    cpu.initialize();
    cpu.load_rom("pong.rom").expect("Unable to load ROM");

    loop {
        cpu.do_cycle();
        sleep(Duration::from_millis(1000/70));
    }
}
