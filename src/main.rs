use std::{fs, io};

const OPS_START_ADDRESS: u16 = 0x200;
const FONTSET_START_ADDRESS: u16 = 0x50;
const FONTSET_SIZE: u16 = 80;
const VIDEO_WIDTH: u16 = 64;
const VIDEO_HEIGHT: u16 = 32;

#[derive(Debug)]
struct Chip8 {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    gfx: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u8,
    keypad: [u8; 16],
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Cls,                // CLS
    Ret,                // RET
    Sys(u16),           // SYS addr
    Jump(u16),          // JP addr
    Call(u16),          // CALL addr
    SkipEqByte(u8, u8), // SE Vx, byte
    SkipNeByte(u8, u8), // SNE Vx, byte
    SkipEqReg(u8, u8),  // SE Vx, Vy
    LoadByte(u8, u8),   // LD Vx, byte
    AddByte(u8, u8),    // ADD Vx, byte
    LoadReg(u8, u8),    // LD Vx, Vy
    OrReg(u8, u8),      // OR Vx, Vy
    AndReg(u8, u8),     // AND Vx, Vy
    XorReg(u8, u8),     // XOR Vx, Vy
    AddReg(u8, u8),     // ADD Vx, Vy
    SubReg(u8, u8),     // SUB Vx, Vy
    ShrReg(u8, u8),     // SHR Vx {, Vy}
    SubnReg(u8, u8),    // SUBN Vx, Vy
    ShlReg(u8, u8),     // SHL Vx {, Vy}
    SkipNeReg(u8, u8),  // SNE Vx, Vy
    LoadI(u16),         // LD I, addr
    JumpV0(u16),        // JP V0, addr
    Rand(u8, u8),       // RND Vx, byte
    Draw(u8, u8, u8),   // DRW Vx, Vy, nibble
    SkipIfKey(u8),      // SKP Vx
    SkipIfNotKey(u8),   // SKNP Vx
    LoadDT(u8),         // LD Vx, DT
    WaitKey(u8),        // LD Vx, K
    SetDT(u8),          // LD DT, Vx
    SetST(u8),          // LD ST, Vx
    AddI(u8),           // ADD I, Vx
    LoadSprite(u8),     // LD F, Vx
    Bcd(u8),            // LD B, Vx
    DumpRegs(u8),       // LD [I], Vx
    LoadRegs(u8),       // LD Vx, [I]
    NoOp,
}

impl From<u16> for Instruction {
    fn from(opcode: u16) -> Self {
        match opcode {
            0x00E0 => Instruction::Cls,
            0x00EE => Instruction::Ret,
            _ if opcode & 0xF000 == 0x0000 => Instruction::Sys(opcode & 0x0FFF),
            _ if opcode & 0xF000 == 0x1000 => Instruction::Jump(opcode & 0x0FFF),
            _ if opcode & 0xF000 == 0x2000 => Instruction::Call(opcode & 0x0FFF),
            _ if opcode & 0xF000 == 0x3000 => {
                Instruction::SkipEqByte(((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8)
            }
            _ if opcode & 0xF000 == 0x4000 => {
                Instruction::SkipNeByte(((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8)
            }
            _ if opcode & 0xF00F == 0x5000 => Instruction::SkipEqReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF000 == 0x6000 => {
                Instruction::LoadByte(((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8)
            }
            _ if opcode & 0xF000 == 0x7000 => {
                Instruction::AddByte(((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8)
            }
            _ if opcode & 0xF00F == 0x8000 => Instruction::LoadReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF00F == 0x8001 => Instruction::OrReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF00F == 0x8002 => Instruction::AndReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF00F == 0x8003 => Instruction::XorReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF00F == 0x8004 => Instruction::AddReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF00F == 0x8005 => Instruction::SubReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF00F == 0x8006 => Instruction::ShrReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF00F == 0x8007 => Instruction::SubnReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF00F == 0x800E => Instruction::ShlReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF00F == 0x9000 => Instruction::SkipNeReg(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
            ),
            _ if opcode & 0xF000 == 0xA000 => Instruction::LoadI(opcode & 0x0FFF),
            _ if opcode & 0xF000 == 0xB000 => Instruction::JumpV0(opcode & 0x0FFF),
            _ if opcode & 0xF000 == 0xC000 => {
                Instruction::Rand(((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8)
            }
            _ if opcode & 0xF000 == 0xD000 => Instruction::Draw(
                ((opcode & 0x0F00) >> 8) as u8,
                ((opcode & 0x00F0) >> 4) as u8,
                (opcode & 0x000F) as u8,
            ),
            _ if opcode & 0xF0FF == 0xE09E => {
                Instruction::SkipIfKey(((opcode & 0x0F00) >> 8) as u8)
            }
            _ if opcode & 0xF0FF == 0xE0A1 => {
                Instruction::SkipIfNotKey(((opcode & 0x0F00) >> 8) as u8)
            }
            _ if opcode & 0xF0FF == 0xF007 => Instruction::LoadDT(((opcode & 0x0F00) >> 8) as u8),
            _ if opcode & 0xF0FF == 0xF00A => Instruction::WaitKey(((opcode & 0x0F00) >> 8) as u8),
            _ if opcode & 0xF0FF == 0xF015 => Instruction::SetDT(((opcode & 0x0F00) >> 8) as u8),
            _ if opcode & 0xF0FF == 0xF018 => Instruction::SetST(((opcode & 0x0F00) >> 8) as u8),
            _ if opcode & 0xF0FF == 0xF01E => Instruction::AddI(((opcode & 0x0F00) >> 8) as u8),
            _ if opcode & 0xF0FF == 0xF029 => {
                Instruction::LoadSprite(((opcode & 0x0F00) >> 8) as u8)
            }
            _ if opcode & 0xF0FF == 0xF033 => Instruction::Bcd(((opcode & 0x0F00) >> 8) as u8),
            _ if opcode & 0xF0FF == 0xF055 => Instruction::DumpRegs(((opcode & 0x0F00) >> 8) as u8),
            _ if opcode & 0xF0FF == 0xF065 => Instruction::LoadRegs(((opcode & 0x0F00) >> 8) as u8),
            _ => Instruction::NoOp,
        }
    }
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: OPS_START_ADDRESS,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keypad: [0; 16],
        }
    }

    pub fn load_rom(&mut self, bytes: &Vec<u8>) {
        let start = OPS_START_ADDRESS as usize;
        let end = start + bytes.len();
        self.memory[start..end].copy_from_slice(&bytes);
    }

    pub fn pop_opcode(&mut self) -> Instruction {
        let high_byte = self.memory[self.pc as usize];
        let low_byte = self.memory[(self.pc + 1) as usize];
        self.pc += 2;
        let val = (u16::from(high_byte) << 8) | u16::from(low_byte);
        Instruction::from(val)
    }

    pub fn emulate(&mut self) -> () {
        loop {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.pop_opcode();
        self.execute(opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn execute(&mut self, ins: Instruction) {
        match ins {
            Instruction::Cls => {
                self.gfx.fill(0);
            }
            Instruction::Ret => {
                self.sp -= 1;
                self.pc = self.stack[self.pc as usize];
            }
            Instruction::Sys(_) => {}
            Instruction::Jump(addr) => self.pc = addr,
            Instruction::Call(addr) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = addr;
            }
            Instruction::SkipEqByte(x, byte) => {
                if self.v[x as usize] == byte {
                    self.pc += 2;
                }
            }
            Instruction::SkipNeByte(x, byte) => {
                if self.v[x as usize] != byte {
                    self.pc += 2;
                }
            }
            Instruction::SkipEqReg(x, y) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            Instruction::LoadByte(x, byte) => {
                self.v[x as usize] = byte;
            }
            Instruction::AddByte(x, byte) => {
                self.v[x as usize] += byte;
            }
            Instruction::LoadReg(x, y) => {
                self.v[x as usize] = self.v[y as usize];
            }
            Instruction::OrReg(x, y) => {
                self.v[x as usize] |= self.v[y as usize];
            }
            Instruction::AndReg(x, y) => {
                self.v[x as usize] &= self.v[y as usize];
            }
            Instruction::XorReg(x, y) => {
                self.v[x as usize] ^= self.v[y as usize];
            }
            Instruction::AddReg(x, y) => {
                let sum = u16::from(self.v[x as usize] + self.v[y as usize]);
                self.v[0xF] = u8::from(sum > 255);
                self.v[x as usize] = (sum & 0xFF) as u8;
            }
            Instruction::SubReg(x, y) => {
                let x = x as usize;
                let y = y as usize;
                self.v[0xF] = u8::from(self.v[x] > self.v[y]);
                self.v[x] -= self.v[y];
            }
            Instruction::ShrReg(x, _) => {
                let x = x as usize;
                self.v[0xF] = self.v[x] & 0x01;
                self.v[x] >>= 1;
            }
            Instruction::SubnReg(x, y) => {
                let x = x as usize;
                let y = y as usize;
                self.v[0xF] = u8::from(self.v[y] > self.v[x]);
                self.v[x] = self.v[y] - self.v[x];
            }
            Instruction::ShlReg(x, _) => {
                let x = x as usize;
                self.v[0xF] = (self.v[x] & 0x80) >> 7;
                self.v[x] <<= 1;
            }
            Instruction::SkipNeReg(x, y) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            Instruction::LoadI(addr) => {
                self.i = addr;
            }
            Instruction::JumpV0(addr) => {
                self.pc = addr + u16::from(self.v[0]);
            }
            Instruction::Rand(x, byte) => {
                let rnd: u8 = rand::random();
                self.v[x as usize] = rnd & byte;
            }
            Instruction::Draw(x, y, n) => {
                let vx = self.v[x as usize] as u16;
                let vy = self.v[y as usize] as u16;
                self.v[0xF] = 0;

                for byte_index in 0..n {
                    let sprite_byte = self.memory[(self.i + byte_index as u16) as usize];
                    for bit_index in 0..8 {
                        let pixel_value = (sprite_byte >> (7 - bit_index)) & 0x01;
                        let x_coord = (vx + bit_index) % VIDEO_WIDTH;
                        let y_coord = (vy + byte_index as u16) % VIDEO_HEIGHT;
                        let gfx_index = (x_coord + y_coord * VIDEO_WIDTH) as usize;

                        if pixel_value == 1 {
                            if self.gfx[gfx_index] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[gfx_index] ^= 1;
                        }
                    }
                }
            }
            Instruction::SkipIfKey(x) => {
                if self.keypad[self.v[x as usize] as usize] != 0 {
                    self.pc += 2;
                }
            }
            Instruction::SkipIfNotKey(x) => {
                if self.keypad[self.v[x as usize] as usize] == 0 {
                    self.pc += 2;
                }
            }
            Instruction::LoadDT(x) => {
                self.v[x as usize] = self.delay_timer;
            }
            Instruction::WaitKey(x) => {
                for (i, &key) in self.keypad.iter().enumerate() {
                    if key != 0 {
                        self.v[x as usize] = i as u8;
                        return;
                    }
                }
                self.pc -= 2;
            }
            Instruction::SetDT(x) => {
                self.delay_timer = self.v[x as usize];
            }
            Instruction::SetST(x) => {
                self.sound_timer = self.v[x as usize];
            }
            Instruction::AddI(x) => {
                self.i += u16::from(self.v[x as usize]);
            }
            Instruction::LoadSprite(x) => {
                self.i = FONTSET_START_ADDRESS + u16::from(self.v[x as usize]) * 5;
            }
            Instruction::Bcd(x) => {
                let value = self.v[x as usize];
                self.memory[self.i as usize] = value / 100;
                self.memory[(self.i + 1) as usize] = (value % 100) / 10;
                self.memory[(self.i + 2) as usize] = value % 10;
            }
            Instruction::DumpRegs(x) => {
                for idx in 0..=x {
                    self.memory[(self.i + idx as u16) as usize] = self.v[idx as usize];
                }
            }
            Instruction::LoadRegs(x) => {
                for idx in 0..=x {
                    self.v[idx as usize] = self.memory[(self.i + idx as u16) as usize];
                }
            }
            Instruction::NoOp => {}
        }
    }
}

fn main() -> Result<(), io::Error> {
    let bytes = fs::read("examples/test_opcode.ch8")?;
    let mut chip8 = Chip8::new();
    chip8.load_rom(&bytes);
    chip8.emulate();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_chip8() -> (Chip8, Vec<u8>) {
        let mut chip8 = Chip8::new();
        let bytes = vec![
            // 0x00E0 CLS, 0x00EE RET
            0x00, 0xE0, 0x00, 0xEE, // 0x0nnn SYS addr, 0x1nnn JP addr, 0x2nnn CALL addr
            0x01, 0x23, 0x11, 0x23, 0x21, 0x23,
            // 0x3xkk SE Vx, byte; 0x4xkk SNE Vx, byte
            0x31, 0x02, 0x41, 0x02, // 0x5xy0 SE Vx, Vy
            0x51, 0x20, // 0x6xkk LD Vx, byte; 0x7xkk ADD Vx, byte
            0x61, 0x02, 0x71, 0x02,
            // 0x8xy0..8xyE ALU ops (LD, OR, AND, XOR, ADD, SUB, SHR, SUBN, SHL)
            0x81, 0x20, 0x81, 0x21, 0x81, 0x22, 0x81, 0x23, 0x81, 0x24, 0x81, 0x25, 0x81, 0x26,
            0x81, 0x27, 0x81, 0x2E, // 0x9xy0 SNE Vx, Vy
            0x91, 0x20, // 0xAnnn LD I, addr; 0xBnnn JP V0, addr
            0xA1, 0x23, 0xB1, 0x23, // 0xCxkk RND Vx, byte
            0xC1, 0x02, // 0xDxyn DRW Vx, Vy, nibble
            0xD1, 0x23, // 0xEx9E SKP Vx; 0xExA1 SKNP Vx
            0xE1, 0x9E, 0xE1, 0xA1, // 0xFx__ instructions
            0xF1, 0x07, 0xF1, 0x0A, 0xF1, 0x15, 0xF1, 0x18, 0xF1, 0x1E, 0xF1, 0x29, 0xF1, 0x33,
            0xF1, 0x55, 0xF1, 0x65,
        ];
        chip8.load_rom(&bytes);
        (chip8, bytes)
    }

    #[test]
    fn test_rom_loading() {
        let (chip8, bytes) = gen_test_chip8();
        let start = OPS_START_ADDRESS as usize;
        let end = (start + bytes.len()) as usize;
        assert!(chip8.memory[start..end] == bytes);
    }

    #[test]
    fn test_opcode_parsing() {
        let (mut chip8, _) = gen_test_chip8();
        assert!(chip8.pop_opcode() == Instruction::Cls);
        assert!(chip8.pop_opcode() == Instruction::Ret);
        assert!(chip8.pop_opcode() == Instruction::Sys(0x123));
        assert!(chip8.pop_opcode() == Instruction::Jump(0x123));
        assert!(chip8.pop_opcode() == Instruction::Call(0x123));
        assert!(chip8.pop_opcode() == Instruction::SkipEqByte(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::SkipNeByte(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::SkipEqReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::LoadByte(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::AddByte(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::LoadReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::OrReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::AndReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::XorReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::AddReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::SubReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::ShrReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::SubnReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::ShlReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::SkipNeReg(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::LoadI(0x123));
        assert!(chip8.pop_opcode() == Instruction::JumpV0(0x123));
        assert!(chip8.pop_opcode() == Instruction::Rand(0x1, 0x02));
        assert!(chip8.pop_opcode() == Instruction::Draw(0x1, 0x2, 0x3));
        assert!(chip8.pop_opcode() == Instruction::SkipIfKey(0x1));
        assert!(chip8.pop_opcode() == Instruction::SkipIfNotKey(0x1));
        assert!(chip8.pop_opcode() == Instruction::LoadDT(0x1));
        assert!(chip8.pop_opcode() == Instruction::WaitKey(0x1));
        assert!(chip8.pop_opcode() == Instruction::SetDT(0x1));
        assert!(chip8.pop_opcode() == Instruction::SetST(0x1));
        assert!(chip8.pop_opcode() == Instruction::AddI(0x1));
        assert!(chip8.pop_opcode() == Instruction::LoadSprite(0x1));
        assert!(chip8.pop_opcode() == Instruction::Bcd(0x1));
        assert!(chip8.pop_opcode() == Instruction::DumpRegs(0x1));
        assert!(chip8.pop_opcode() == Instruction::LoadRegs(0x1));
    }
}
