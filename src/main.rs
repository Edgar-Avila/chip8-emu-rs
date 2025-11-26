mod chip8;
mod instruction;

use chip8::Chip8;
use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let bytes = fs::read("examples/test_opcode.ch8")?;
    let mut chip8 = Chip8::new();
    chip8.load_rom(&bytes);
    chip8.emulate();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::chip8::{Chip8, OPS_START_ADDRESS};
    use crate::instruction::Instruction;

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
