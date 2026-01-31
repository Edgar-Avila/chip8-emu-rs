use crate::chip8::Chip8;

pub trait Renderer {
    fn init(&mut self);
    fn render(&mut self, chip8: &Chip8);
    fn cleanup(&mut self);
}
