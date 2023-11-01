use std::path::Path;

use clap::Parser;
use emulator::Emulator;
use sdl2::{render::Canvas, video::Window, EventPump};

mod opcode;
mod emulator;

#[derive(Debug, clap::Parser)]
pub struct AppConfiguration {
    #[doc = "Specify the Chip8 rom path"]
    #[arg(long)]
    pub rom: String,

    #[doc = "Enables the hardware renderer"]
    #[arg(long, default_value_t = false)]
    pub hardware_canvas: bool,

    #[doc = "Specify the width of the window"]
    #[arg(long, default_value_t = 800)]
    pub width: u32,

    #[doc = "Specify the height of the window"]
    #[arg(long, default_value_t = 600)]
    pub height: u32
}

#[derive(Debug, PartialEq)]
pub enum AppStatus {
    Continue,
    Exit
}

fn main() {
    let configuration = AppConfiguration::parse();

    let sdl = sdl2::init().expect("Failed to init SDL!");
    let sdl_video = sdl.video().expect("Failed to init SDL Video!");

    let mut event_pump = sdl.event_pump().expect("Failed to init SDL Event Pump!");

    let window = sdl_video.window("CHIP8 Emulator", configuration.width, configuration.height)
    .allow_highdpi()
    .resizable()
    .build()
    .expect("Failed to init SDL Window!");

    let mut window_canvas = window.into_canvas()
    .present_vsync();

    if configuration.hardware_canvas {
        window_canvas = window_canvas.accelerated();
    } else {
        window_canvas = window_canvas.software();
    }

    let mut window_canvas = window_canvas.build()
    .expect("Failed to create window canvas!");

    let mut emulator = Emulator::new(std::fs::read(Path::new::<String>(&configuration.rom)).expect("Invalid rom path!"));

    'run_loop: loop {
        handle_input(&event_pump, &mut emulator);

        if update(&mut event_pump, &mut emulator) == AppStatus::Exit {
            break 'run_loop;
        }
        render(&mut window_canvas, &emulator);
    }
}

fn render(window_canvas: &mut Canvas<Window>, emulator: &Emulator) {
    window_canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    window_canvas.clear();

    window_canvas.set_scale(window_canvas.window().size().0 as f32 / 64.0, window_canvas.window().size().1 as f32 / 32.0).expect("Failed to set SDL Window Canvas Scale!");

    window_canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    for (row_iteration, row) in emulator.video_memory().iter().enumerate() {
        for (column_iteration, column) in row.iter().enumerate() {
            if *column != false {
                window_canvas.draw_point(sdl2::rect::Point::new(row_iteration as i32, column_iteration as i32)).expect("Failed to draw a Point!");
            }
        }
    }

    window_canvas.present();
}

fn update(event_pump: &mut EventPump, emulator: &mut Emulator) -> AppStatus {
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit { timestamp: _ } => {
                return AppStatus::Exit;
            },
            _ => {}
        }
    }

    emulator.next_cycle();

    AppStatus::Continue
}

fn handle_input(event_pump: &EventPump, emulator: &mut Emulator) {
    let keyboard_state = event_pump.keyboard_state();
    let scancodes: Vec<sdl2::keyboard::Scancode> = keyboard_state.pressed_scancodes().collect();

    if !scancodes.is_empty() {
        emulator.set_scancodes(scancodes);
    }
}