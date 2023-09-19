use crate::emulator::Emulator;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub trait App {
    fn update(&mut self);
    fn render(&mut self);
}

pub struct NaukaApp {
    event_pump: sdl2::EventPump,
    window: sdl2::video::Window,
    emulator: Emulator
}

impl NaukaApp {
    fn new(rom: Vec<u8>) -> Self {
        let sdl = sdl2::init().expect("Failed to init SDL!");
        let sdl_video = sdl.video().expect("Failed to init SDL Video!");

        let event_pump = sdl.event_pump().expect("Failed to init SDL Event Pump!");

        let window = sdl_video.window("CHIP8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .allow_highdpi()
        .build()
        .expect("Failed to init SDL Window!");

        Self {
            event_pump,
            window,
            emulator: Emulator::new(rom)
        }
    }
}