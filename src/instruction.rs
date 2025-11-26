#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
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
