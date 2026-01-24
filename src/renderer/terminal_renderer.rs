use crate::{chip8::Chip8, renderer::renderer::Renderer};

pub struct TerminalRenderer {}

impl TerminalRenderer {
    pub fn new() -> Self {
        TerminalRenderer {}
    }
}

impl Renderer for TerminalRenderer {
    fn render(&self, chip8: &Chip8) {
        for y in 0..32 {
            for x in 0..64 {
                let pixel = chip8.gfx[y * 64 + x];
                if pixel == 1 {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
