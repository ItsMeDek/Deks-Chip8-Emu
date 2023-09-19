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
    window_canvas: sdl2::render::Canvas<sdl2::video::Window>,
    emulator: Emulator
}

impl NaukaApp {
    pub fn new(rom: Vec<u8>) -> Self {
        let sdl = sdl2::init().expect("Failed to init SDL!");
        let sdl_video = sdl.video().expect("Failed to init SDL Video!");

        let event_pump = sdl.event_pump().expect("Failed to init SDL Event Pump!");

        let window = sdl_video.window("CHIP8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .allow_highdpi()
        .build()
        .expect("Failed to init SDL Window!");
        
        let window_canvas = window.into_canvas()
        .software()
        .present_vsync()
        .build()
        .expect("Failed to init SDL Window Canvas!");

        Self {
            event_pump,
            window_canvas,
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
        self.window_canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.window_canvas.clear();

        self.window_canvas.set_scale(WINDOW_WIDTH as f32 / 64.0, WINDOW_HEIGHT as f32 / 32.0).expect("Failed to set SDL Window Canvas Scale!");

        self.window_canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for (row_iteration, row) in self.emulator.video_memory().iter().enumerate() {
            for (column_iteration, column) in row.iter().enumerate() {
                if *column != 0 {
                    self.window_canvas.draw_point(sdl2::rect::Point::new(row_iteration as i32, column_iteration as i32)).expect("Failed to draw a Point!");
                }
            }
        }

        self.window_canvas.present();
    }
}