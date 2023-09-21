use num_derive::FromPrimitive;

#[repr(u16)]
#[derive(Debug, FromPrimitive)]
pub enum Opcode {
    ZeroOpcode = 0x0000,
    JpAddr = 0x1000,
    CallAddr = 0x2000,
    SeVxByte = 0x3000,
    SneVxByte = 0x4000,
    SeVxVy = 0x5000,
    LdVxByte = 0x6000,
    AddVxByte = 0x7000,
    EightOpcode = 0x8000,
    SneVxVy = 0x9000,
    LdIAddr = 0xA000,
    JpV0Addr = 0xB000,
    RndVxByte = 0xC000,
    DrwVxVy = 0xD000,
    FourteenOpcode = 0xE000,
    FifteenOpcode = 0xF000,
}

#[repr(u16)]
#[derive(Debug, FromPrimitive)]
pub enum ZeroOpcode {
    CLS = 0xE0,
    RET = 0xEE
}

#[repr(u16)]
#[derive(Debug, FromPrimitive)]
pub enum EightOpcode {
    LdVxVy = 0x0,
    OrVxVy = 0x1,
    AndVxVy = 0x2,
    XorVxVy = 0x3,
    AddVxVy = 0x4,
    SubVxVy = 0x5,
    ShrVx = 0x6,
    SubnVxVy = 0x7,
    ShlVx = 0xE,
}