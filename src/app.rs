use crate::emulator::Emulator;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Debug, PartialEq)]
pub enum AppStatus {
    Continue,
    Exit
}

pub trait App {
    fn update(&mut self) -> AppStatus;
    fn render(&mut self);
    fn run(&mut self) {
        'run_loop: loop {
            if self.update() == AppStatus::Exit {
                break 'run_loop;
            }
            self.render();
        }
    }
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

impl App for NaukaApp {
    fn update(&mut self) -> AppStatus {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { timestamp: _ } => {
                    return AppStatus::Exit;
                },
                _ => {}
            }
        }

        self.emulator.next_cycle();

        AppStatus::Continue
    }

    fn render(&mut self) {
        
    }
}