mod chip8;
mod instruction;
mod renderer;

use chip8::Chip8;
use clap::{Parser, ValueEnum};
use renderer::terminal_renderer::TerminalRenderer;
use std::{fmt::Display, fs, io, path::PathBuf, thread, time};

use crate::renderer::renderer::Renderer;

const TARGET_FPS: u64 = 60;

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

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(short, long)]
    cycles: Option<u32>,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let bytes = fs::read(args.rom)?;
    let target_ft = time::Duration::from_micros(1_000_000/TARGET_FPS);
    let mut chip8 = Chip8::new();
    let mut t_renderer = TerminalRenderer::new();
    t_renderer.init();
    chip8.load_rom(&bytes);
    if args.debug {
        chip8.enable_debug();
    }
    match args.cycles {
        None => loop {
            let frame_time = time::Instant::now();
            chip8.tick();
            t_renderer.render(&chip8);
            if let Some(remaining) = target_ft.checked_sub(frame_time.elapsed()) {
                thread::sleep(remaining);
            }
        }
        Some(cycles) => for _ in 0..cycles {
            let frame_time = time::Instant::now();
            chip8.tick();
            t_renderer.render(&chip8);
            if let Some(remaining) = target_ft.checked_sub(frame_time.elapsed()) {
                thread::sleep(remaining);
            }
        },
    }
    t_renderer.cleanup();
    Ok(())
}
