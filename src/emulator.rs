use crate::opcode::{Opcode, ZeroOpcode, EightOpcode, FifteenOpcode};

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Emulator {
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

        for (i, byte) in FONT.iter().enumerate() {
            emulator.memory[0x50 + i] = *byte;
        }

        for (i, byte) in rom.iter().enumerate() {
            emulator.memory[0x200 + i] = *byte;
        }

        emulator
    }

    pub fn video_memory(&self) -> [[u8; 32]; 64] {
        self.video_memory
    }

    pub fn next_cycle(&mut self) {
        // Decrement the timers
        for mut timer in self.timers {
            if timer > 0 {
                timer -= 1;
            }
        }

        let opcode = self.fetch_opcode();
        match num::FromPrimitive::from_u16(opcode & 0xF000) {
            Some(Opcode::ZeroOpcode) => {
                match num::FromPrimitive::from_u16(opcode & 0x00FF) {
                    Some(ZeroOpcode::CLS) => {
                        self.clear_screen();
                        self.pc += 2;
                    },
                    Some(ZeroOpcode::RET) => {
                        self.pc = self.pop();
                        self.pc += 2;
                    },
                    _ => {
                        unreachable!();
                    }
                }
            },
            Some(Opcode::JpAddr) => {
                let value = opcode & 0x0FFF;
                self.jump(value);
            },
            Some(Opcode::CallAddr) => {
                let value = opcode & 0x0FFF;

                self.push(self.pc);
                self.jump(value);
            }, 
            Some(Opcode::SeVxByte) => {
                let register_index = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;

                if self.vx[register_index as usize] == value {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Some(Opcode::SneVxByte) => {
                let register_index = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;

                if self.vx[register_index as usize] != value {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Some(Opcode::SeVxVy) => {
                let first_register_index = ((opcode & 0x00F0) >> 4) as u8;
                let second_register_index = ((opcode & 0x0F00) >> 8) as u8;

                if self.vx[first_register_index as usize] == self.vx[second_register_index as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Some(Opcode::LdVxByte) => {
                let register_index = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;

                self.vx[register_index as usize] = value;
                self.pc += 2;
            },
            Some(Opcode::AddVxByte) => {
                let register_index = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;

                self.vx[register_index as usize] += value;
                self.pc += 2;
            },
            Some(Opcode::EightOpcode) => {
                match num::FromPrimitive::from_u16(opcode & 0x000F) {
                    Some(EightOpcode::LdVxVy) => {
                        let first_register_index = ((opcode & 0x0F00) >> 8) as u8;
                        let second_register_index = ((opcode & 0x00F0) >> 4) as u8;

                        self.vx[first_register_index as usize] = self.vx[second_register_index as usize];
                        self.pc += 2;
                    },
                    Some(EightOpcode::OrVxVy) => {
                        let first_register_index = ((opcode & 0x0F00) >> 8) as u8;
                        let second_register_index = ((opcode & 0x00F0) >> 4) as u8;

                        self.vx[first_register_index as usize] |= self.vx[second_register_index as usize];
                        self.pc += 2;
                    },
                    Some(EightOpcode::AndVxVy) => {
                        let first_register_index = ((opcode & 0x0F00) >> 8) as u8;
                        let second_register_index = ((opcode & 0x00F0) >> 4) as u8;

                        self.vx[first_register_index as usize] &= self.vx[second_register_index as usize];
                        self.pc += 2;
                    },
                    Some(EightOpcode::XorVxVy) => {
                        let first_register_index = ((opcode & 0x0F00) >> 8) as u8;
                        let second_register_index = ((opcode & 0x00F0) >> 4) as u8;

                        self.vx[first_register_index as usize] ^= self.vx[second_register_index as usize];
                        self.pc += 2;
                    },
                    Some(EightOpcode::AddVxVy) => {
                        let first_register_index = ((opcode & 0x0F00) >> 8) as u8;
                        let second_register_index = ((opcode & 0x00F0) >> 4) as u8;

                        let vx_value_before = self.vx[first_register_index as usize];

                        self.vx[first_register_index as usize] = self.vx[first_register_index as usize].wrapping_add(self.vx[second_register_index as usize]);

                        if self.vx[first_register_index as usize] < vx_value_before {
                            self.vx[15] = 1;
                        } else {
                            self.vx[15] = 0;
                        }
                        self.pc += 2;
                    },
                    Some(EightOpcode::SubVxVy) => {
                        let first_register_index = ((opcode & 0x0F00) >> 8) as u8;
                        let second_register_index = ((opcode & 0x00F0) >> 4) as u8;

                        self.vx[first_register_index as usize] = self.vx[first_register_index as usize].wrapping_sub(self.vx[second_register_index as usize]);

                        if self.vx[first_register_index as usize] > self.vx[second_register_index as usize] {
                            self.vx[15] = 1;
                        } else {
                            self.vx[15] = 0;
                        }
                        self.pc += 2;
                    },
                    Some(EightOpcode::ShrVx) => {
                        let first_register_index = ((opcode & 0x0F00) >> 8) as u8;
                        //let second_register_index = ((opcode & 0x00F0) >> 4) as u8;

                        self.vx[first_register_index as usize] = self.vx[first_register_index as usize].wrapping_shr(1);

                        if (self.vx[first_register_index as usize] & 0b00000001) == 1 {
                            self.vx[15] = 1;
                        } else {
                            self.vx[15] = 0;
                        }
                        self.pc += 2;
                    },
                    Some(EightOpcode::SubnVxVy) => {
                        let first_register_index = ((opcode & 0x0F00) >> 8) as u8;
                        let second_register_index = ((opcode & 0x00F0) >> 4) as u8;

                        self.vx[first_register_index as usize] = self.vx[second_register_index as usize].wrapping_sub(self.vx[first_register_index as usize]);

                        if self.vx[second_register_index as usize] > self.vx[first_register_index as usize] {
                            self.vx[15] = 1;
                        } else {
                            self.vx[15] = 0;
                        }
                        self.pc += 2;
                    },
                    Some(EightOpcode::ShlVx) => {
                        let first_register_index = ((opcode & 0x0F00) >> 8) as u8;
                        //let second_register_index = ((opcode & 0x00F0) >> 4) as u8;

                        self.vx[first_register_index as usize] = self.vx[first_register_index as usize].wrapping_shl(1);

                        if (self.vx[first_register_index as usize] & 0b10000000) == 1 {
                            self.vx[15] = 1;
                        } else {
                            self.vx[15] = 0;
                        }
                        self.pc += 2;
                    },
                    _ => {
                        unreachable!();
                    }
                }
            },
            Some(Opcode::SneVxVy) => {
                let first_register_index = ((opcode & 0x00F0) >> 4) as u8;
                let second_register_index = ((opcode & 0x0F00) >> 8) as u8;

                if self.vx[first_register_index as usize] != self.vx[second_register_index as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            Some(Opcode::LdIAddr) => {
                let value = opcode & 0x0FFF;

                self.set_i(value);
                self.pc += 2;
            },
            Some(Opcode::DrwVxVy) => {
                let sprite_size = (opcode & 0x000F) as u16;
                let y_register_index = ((opcode & 0x00F0) >> 4) as u8;
                let x_register_index = ((opcode & 0x0F00) >> 8) as u8;
                
                let sprite_x = (self.vx[x_register_index as usize] % 64) as usize;
                let sprite_y = (self.vx[y_register_index as usize] % 32) as usize;

                let sprite = self.read_ram(self.i, sprite_size);

                for column in 0..sprite.len() {
                    for row in 0..8 {
                        if (sprite[column] & (0x80 >> row)) != 0 {
                            let sprite_x = sprite_x + row;
                            let sprite_y = sprite_y + column;

                            if self.video_memory[sprite_x][sprite_y] ^ sprite[column] as u8 != self.video_memory[sprite_x][sprite_y] {
                                self.vx[15] = 1;
                            } else {
                                self.vx[15] = 0;
                            }
                            self.video_memory[sprite_x][sprite_y] ^= sprite[column] as u8;
                        }
                    }
                }
                self.pc += 2;
            },
            Some(Opcode::FifteenOpcode) => {
                match num::FromPrimitive::from_u16(opcode & 0x00FF) {
                    Some(FifteenOpcode::LdVxDt) => {
                        let register_index = ((opcode & 0x0F00) >> 8) as u8;

                        self.vx[register_index as usize] = self.timers[0];
                        self.pc += 2;
                    },
                    Some(FifteenOpcode::LdDtVx) => {
                        let register_index = ((opcode & 0x0F00) >> 8) as u8;
                        let register_value = self.vx[register_index as usize];

                        self.timers[0] = register_value;
                        self.pc += 2;
                    },
                    Some(FifteenOpcode::AddIVx) => {
                        let register_index = ((opcode & 0x0F00) >> 8) as u8;

                        self.i = self.i.wrapping_add(self.vx[register_index as usize] as u16);
                        self.pc += 2;
                    },
                    Some(FifteenOpcode::LdFVx) => {
                        let register_index = ((opcode & 0x0F00) >> 8) as u8;
                        let register_value = self.vx[register_index as usize];

                        self.i = 0x50 + (register_value * 5) as u16;
                        self.pc += 2;
                    },
                    Some(FifteenOpcode::LdBVx) => {
                        let register_index = ((opcode & 0x0F00) >> 8) as u8;
                        let register_value = self.vx[register_index as usize];

                        let bcd_values = vec![register_value / 100, register_value % 100 / 10, register_value % 10];

                        self.write_ram(self.i, bcd_values);
                        self.pc += 2;
                    },
                    Some(FifteenOpcode::LdIVx) => {
                        let value = ((opcode & 0x0F00) >> 8) as u8;

                        for i in 0..=value {
                            self.write_ram(self.i + i as u16, vec![self.vx[i as usize]]);
                        }

                        self.pc += 2;
                    },
                    Some(FifteenOpcode::LdVxI) => {
                        let value = ((opcode & 0x0F00) >> 8) as u8;

                        let read_memory = self.read_ram(self.i, (value + 1) as u16);

                        for i in 0..=value {
                            self.vx[i as usize] = read_memory[i as usize];
                        }

                        self.pc += 2;
                    },
                    _ => {
                        panic!("Unknown opcode 0x{:x}!", opcode);
                    }
                }
            },
            _ => {
                panic!("Unknown opcode 0x{:x}!", opcode);
            }
        }
    }

    fn fetch_opcode(&self) -> u16 {
        let nibble1 = self.memory[self.pc as usize];
        let nibble2 = self.memory[(self.pc + 1) as usize];
        let opcode: u16 = ((nibble1 as u16) << 8) | (nibble2 as u16);

        opcode
    }

    #[doc = "Jump to a specific place in memory"]
    fn jump(&mut self, location: u16) {
        self.pc = location;
    }

    #[doc = "Push a value to the stack"]
    fn push(&mut self, value: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = value;
    }

    #[doc = "Pop a value from the stack"]
    fn pop(&mut self) -> u16 {
        let result = self.stack[self.sp as usize];
        self.stack[self.sp as usize] = 0;
        self.sp -= 1;
        result
    }

    #[doc = "Override the entire vram with 0's"]
    fn clear_screen(&mut self) {
        for row in self.video_memory {
            for mut _column in row {
                _column = 0;
            }
        }
    }

    #[doc = "Set the I register to the specified value"]
    fn set_i(&mut self, value: u16) {
        self.i = value;
    }

    #[doc = "Read the specified number of bytes from memory at an offset"]
    fn read_ram(&self, offset: u16, number_of_bytes: u16) -> Vec<u8> {
        let mut read_bytes: Vec<u8> = Vec::new();

        for iteration in 0..number_of_bytes {
            read_bytes.push(self.memory[(offset + iteration) as usize]);
        }

        read_bytes
    }

    #[doc = "Write the specified number of bytes to the memory at an offset"]
    fn write_ram(&mut self, offset: u16, bytes: Vec<u8>) {
        for (i, byte) in bytes.iter().enumerate() {
            self.memory[offset as usize + i] = *byte;
        }
    }
}