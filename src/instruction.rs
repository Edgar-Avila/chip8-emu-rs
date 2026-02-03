#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /// 0x00E0 - Clear the display
    Cls,
    /// 0x00EE - Return from a subroutine
    Ret,
    /// 0x0NNN - Jump to a machine code routine at address NNN
    Sys(u16),
    /// 0x1NNN - Jump to address NNN
    Jump(u16),
    /// 0x2NNN - Call subroutine at NNN
    Call(u16),
    /// 0x3XNN - Skip next instruction if VX equals NN
    SkipEqByte(u8, u8),
    /// 0x4XNN - Skip next instruction if VX does not equal NN
    SkipNeByte(u8, u8),
    /// 0x5XY0 - Skip next instruction if VX equals VY
    SkipEqReg(u8, u8),
    /// 0x6XNN - Set VX to NN
    LoadByte(u8, u8),
    /// 0x7XNN - Add NN to VX
    AddByte(u8, u8),
    /// 0x8XY0 - Set VX to the value of VY
    LoadReg(u8, u8),
    /// 0x8XY1 - Set VX to VX OR VY
    OrReg(u8, u8),
    /// 0x8XY2 - Set VX to VX AND VY
    AndReg(u8, u8),
    /// 0x8XY3 - Set VX to VX XOR VY
    XorReg(u8, u8),
    /// 0x8XY4 - Add VY to VX. Set VF to 1 if there's a carry, else 0
    AddReg(u8, u8),
    /// 0x8XY5 - Subtract VY from VX. Set VF to 0 if there's a borrow, else 1
    SubReg(u8, u8),
    /// 0x8XY6 - Shift VX right by one. VF is set to the least significant bit of VX before the shift
    ShrReg(u8, u8),
    /// 0x8XY7 - Set VX to VY minus VX. Set VF to 0 if there's a borrow, else 1
    SubnReg(u8, u8),
    /// 0x8XYE - Shift VX left by one. VF is set to the most significant bit of VX before the shift
    ShlReg(u8, u8),
    /// 0x9XY0 - Skip next instruction if VX does not equal VY
    SkipNeReg(u8, u8),
    /// 0xANNN - Set I to address NNN
    LoadI(u16),
    /// 0xBNNN - Jump to address NNN plus V0
    JumpV0(u16),
    /// 0xCXNN - Set VX to a random byte AND NN
    Rand(u8, u8),
    /// 0xDXYN - Draw a sprite at (VX, VY) with N bytes of sprite data starting at the address stored in I
    Draw(u8, u8, u8),
    /// 0xEX9E - Skip next instruction if key with the value of VX is pressed
    SkipIfKey(u8),
    /// 0xEXA1 - Skip next instruction if key with the value of VX is not pressed
    SkipIfNotKey(u8),
    /// 0xFX07 - Set VX to the value of the delay timer
    LoadDT(u8),
    /// 0xFX0A - Wait for a key press and store the value of the key in VX
    WaitKey(u8),
    /// 0xFX15 - Set the delay timer to VX
    SetDT(u8),
    /// 0xFX18 - Set the sound timer to VX
    SetST(u8),
    /// 0xFX1E - Add VX to I
    AddI(u8),
    /// 0xFX29 - Set I to the location of the sprite for the character in VX
    LoadSprite(u8),
    /// 0xFX33 - Store the binary-coded decimal representation of VX at addresses I, I+1, and I+2
    Bcd(u8),
    /// 0xFX55 - Store registers V0 through VX in memory starting at address I
    DumpRegs(u8),
    /// 0xFX65 - Read registers V0 through VX from memory starting at address I
    LoadRegs(u8),
    /// No operation (invalid or unrecognized opcode)
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
