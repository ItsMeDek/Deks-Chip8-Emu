use std::path::Path;

use emulator::Emulator;

mod emulator;
mod app;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let rom = std::fs::read(Path::new::<String>(&args[0])).expect("Invalid rom path!");

    let mut emulator = Emulator::new(rom);
    // TODO: Replace
    loop {
        emulator.next_cycle();
    }
}
