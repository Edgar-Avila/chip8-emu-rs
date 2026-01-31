use std::io::{self, Stdout, Write};
use crate::{chip8::Chip8, renderer::renderer::Renderer};
use crossterm::{ExecutableCommand, QueueableCommand, cursor, style, terminal};

pub struct TerminalRenderer {
    stdout: Stdout
}

impl TerminalRenderer {
    pub fn new() -> Self {
        TerminalRenderer {
            stdout: io::stdout(),
        }
    }
}

impl Renderer for TerminalRenderer {
    fn render(&mut self, chip8: &Chip8) {
        self.stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();
        for y in 0..32 {
            for x in 0..64 {
                let pixel = chip8.gfx[y * 64 + x];
                self.stdout.queue(cursor::MoveTo(x as u16, y as u16)).unwrap();
                if pixel == 1 {
                    self.stdout.queue(style::Print("█")).unwrap();
                } else {
                    self.stdout.queue(style::Print(" ")).unwrap();
                }
            }
            self.stdout.queue(style::Print("\n")).unwrap();
        }
        self.stdout.flush().unwrap();
    }

    fn init(&mut self) {
        self.stdout.execute(cursor::Hide).unwrap();
    }

    fn cleanup(&mut self) {}
}
