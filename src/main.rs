mod chip8;
mod instruction;
mod platform;

use chip8::Chip8;
use clap::Parser;
use platform::{Platform, PlatformType, terminal_platform::TerminalPlatform};
use std::{fs, path::PathBuf};

/// Chip-8 Emulator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ROM file
    #[arg(short, long)]
    rom: Option<PathBuf>,

    /// Display type
    #[arg(short, long, default_value_t = PlatformType::Terminal)]
    platform: PlatformType,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(short, long)]
    cycles: Option<u32>,

    #[arg(short, long, default_value_t = 60)]
    fps: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let chip8 = Chip8::new();
    let settings = platform::Settings {
        debug: args.debug,
        cycles: args.cycles,
        fps: args.fps,
    };
    let mut platform = match args.platform {
        PlatformType::Terminal => TerminalPlatform::new(chip8, settings),
    };
    platform.init()?;
    if let Some(rom) = args.rom {
        let bytes = fs::read(rom)?;
        platform.load(bytes);
    }
    platform.run()?;
    platform.cleanup()?;
    Ok(())
}
