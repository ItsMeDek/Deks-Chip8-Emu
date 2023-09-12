pub trait App {
    fn update(&mut self);
    fn render(&mut self);
}

struct Emulator {
    // Stack, ram, etc...
    stack: [u16; 16],
    memory: [u8; 4096],
    video_memory: [[u8; 32]; 64],
    // Pseudo-Registers
    sp: u8,
    pc: u16,
    // Normal registers
    vx: [u8; 16],
    i: u16,
    timers: [u8; 2]
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            stack: [0; 16],
            memory: [0; 4096],
            video_memory: [[0; 32]; 64],

            sp: 0,
            pc: 0x200, // 512 in decimal

            vx: [0; 16],
            i: 0,
            timers: [0; 2],
        }
    }
}

pub struct NaukaApp {
    emulator: Emulator
}