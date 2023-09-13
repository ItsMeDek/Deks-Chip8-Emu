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
        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x00FF {
                    0xE0 => {
                        self.clear_screen();
                    },
                    _ => {
                        panic!("Unknown opcode 0x{:x}!", opcode)
                    }
                }
            },
            0x6000 => {
                let register_index = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;

                self.set_register(register_index, value);
            },
            0xA000 => {
                let value = opcode & 0x0FFF;

                self.set_i(value);
            },
            // TODO: Test it and clean it up
            0xD000 => {
                let sprite_size = (opcode & 0x000F) as u16;
                let y_register_index = ((opcode & 0x00F0) >> 4) as u8;
                let x_register_index = ((opcode & 0x0F00) >> 8) as u8;
                
                let sprite_x = self.vx[x_register_index as usize];
                let sprite_y = self.vx[y_register_index as usize];

                let sprite = self.read_ram(self.i, sprite_size);

                for iteration in 0..sprite_size {
                    if self.video_memory[(sprite_x + iteration as u8) as usize][sprite_y as usize] != 0 {
                        self.vx[15] = 1;
                    }
                    self.video_memory[(sprite_x + iteration as u8) as usize][sprite_y as usize] = self.video_memory[(sprite_x + iteration as u8) as usize][sprite_y as usize] ^ sprite[iteration as usize];
                }

                if self.vx[15] != 1 {
                    self.vx[15] = 0;
                }
            },
            _ => {
                panic!("Unknown opcode 0x{:x}!", opcode);
            }
        }

        self.pc += 2;
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

    #[doc = "Set a Vx register to the specified value"]
    fn set_register(&mut self, register_index: u8, value: u8) {
        self.vx[register_index as usize] = value;
    }

    #[doc = "Override the entire vram with 0's"]
    fn clear_screen(&mut self) {
        for row in self.video_memory {
            for mut column in row {
                column = 0;
            }
        }
    }

    #[doc = "Set the I register to the specified value"]
    fn set_i(&mut self, value: u16) {
        self.i = value;
    }

    #[doc = "Read `bytes` bytes from memory at the offset of `offset`"]
    fn read_ram(&self, offset: u16, bytes: u16) -> Vec<u8> {
        let mut read_bytes: Vec<u8> = Vec::new();

        for iteration in 0..bytes {
            read_bytes.push(self.memory[(offset + iteration) as usize]);
        }

        read_bytes
    }
}

pub struct NaukaApp {
    emulator: Emulator
}