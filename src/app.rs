pub trait App {
    fn update(&mut self);
    fn render(&mut self);
}

pub struct Emulator {
    // Stack, ram, etc...
    stack: [u16; 16],
    memory: [u8; 4096],
    video_memory: [[u8; 32]; 64],
    // Pseudo-Registers
    sp: u8,
    pc: usize,
    // Normal registers
    vx: [u8; 16],
    i: u16,
    timers: [u8; 2]
}

impl Emulator {
    pub fn new(rom: Vec<u8>) -> Self {
        let mut emulator = Self {
            stack: [0; 16],
            memory: [0; 4096],
            video_memory: [[0; 32]; 64],

            sp: 0,
            pc: 0x200, // 512 in decimal

            vx: [0; 16],
            i: 0,
            timers: [0; 2],
        };

        for (iteration, byte) in rom.iter().enumerate() {
            emulator.memory[0x200 + iteration] = *byte;
        }

        emulator
    }

    pub fn next_cycle(&mut self) {
        // Decrement the timers
        for mut timer in self.timers {
            if timer > 0 {
                timer -= 1;
            }
        }

        let opcode = self.fetch_opcode();
        match opcode {
            _ => {
                panic!("Unknown opcode 0x{:x}!", opcode);
            }
        }

        self.pc += 1;
    }

    fn fetch_opcode(&self) -> u16 {
        let nibble1 = self.memory[self.pc];
        let nibble2 = self.memory[self.pc + 1];
        let opcode: u16 = ((nibble1 as u16) << 8) | (nibble2 as u16);

        opcode
    }

    #[doc = "Jump to a specific place in memory"]
    fn jump(&mut self, location: u16) {
        self.pc = location as usize;
    }
}

pub struct NaukaApp {
    emulator: Emulator
}