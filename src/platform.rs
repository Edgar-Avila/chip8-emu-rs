pub mod terminal_platform;

use std::fmt::Display;
use clap::ValueEnum;
use crate::chip8::Chip8;

pub struct Settings {
    pub debug: bool,
    pub cycles: Option<u32>,
    pub fps: u64
}

pub trait Platform {
    fn new(chip8: Chip8, settings: Settings) -> Self;
    fn load(&mut self, rom: Vec<u8>);
    fn init(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn run(&mut self);
    fn cleanup(&mut self);
}

#[derive(ValueEnum, Clone, Debug)]
pub enum PlatformType {
    Terminal,
}

impl Display for PlatformType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatformType::Terminal => write!(f, "terminal"),
        }
    }
}
