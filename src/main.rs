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
