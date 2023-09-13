use std::path::Path;

use app::Emulator;

mod app;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let rom = std::fs::read(Path::new::<String>(&args[0])).expect("Invalid rom path!");

    let emulator = Emulator::new(rom);
}
