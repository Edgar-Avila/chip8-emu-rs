use crate::chip8::Chip8;

pub trait Renderer {
    fn render(&self, chip8: &Chip8);
}
