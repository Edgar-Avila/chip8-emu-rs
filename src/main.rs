mod chip8;
mod instruction;
mod renderer;

use chip8::Chip8;
use clap::{Parser, ValueEnum};
use renderer::terminal_renderer::TerminalRenderer;
use std::{fmt::Display, fs, io, path::PathBuf};

use crate::renderer::renderer::Renderer;

#[derive(ValueEnum, Clone, Debug)]
enum RendererType {
    Terminal,
}

impl Display for RendererType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RendererType::Terminal => write!(f, "terminal"),
        }
    }
}

/// Chip-8 Emulator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ROM file
    #[arg(short, long)]
    rom: PathBuf,

    /// Display type
    #[arg(short, long, default_value_t = RendererType::Terminal)]
    display: RendererType,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let bytes = fs::read(args.rom)?;
    let mut chip8 = Chip8::new();
    let t_renderer = TerminalRenderer::new();
    chip8.load_rom(&bytes);
    chip8.enable_debug();
    loop {
        chip8.tick();
        t_renderer.render(&chip8);
    }
    Ok(())
}
