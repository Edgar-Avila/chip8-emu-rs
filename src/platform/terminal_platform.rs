use super::{Platform, Settings};
use crate::chip8::Chip8;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::{ExecutableCommand, QueueableCommand, cursor, event, style, terminal};
use std::io::{self, Stdout, Write};
use std::{thread, time};

pub struct TerminalPlatform {
    stdout: Stdout,
    chip8: Chip8,
    settings: Settings,
    target_ft: time::Duration,
    running: bool,
}

impl TerminalPlatform {
    fn cycle(&mut self) {
        let frame_time = time::Instant::now();
        self.update();
        self.render();
        if let Some(remaining) = self.target_ft.checked_sub(frame_time.elapsed()) {
            thread::sleep(remaining);
        }
    }

    fn handle_event(&mut self, ev: event::Event) {
        match ev {
            Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Esc,
                ..
            }) => {
                self.running = false;
            }
            Event::Key(
                key_event @ KeyEvent {
                    kind: KeyEventKind::Press | KeyEventKind::Release,
                    code: KeyCode::Char(c),
                    ..
                },
            ) => {
                if let Some(k) = ch_to_key(c) {
                    self.chip8.keypad[k] = u8::from(key_event.is_press());
                }
            }
            _ => (),
        }
    }
}

impl Platform for TerminalPlatform {
    fn new(chip8: Chip8, settings: Settings) -> TerminalPlatform {
        let target_ft = time::Duration::from_micros(1_000_000 / settings.fps);
        TerminalPlatform {
            stdout: io::stdout(),
            chip8,
            settings,
            target_ft,
            running: false,
        }
    }

    fn render(&mut self) {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        for y in 0..32 {
            for x in 0..64 {
                let pixel = self.chip8.gfx[y * 64 + x];
                self.stdout
                    .queue(cursor::MoveTo(x as u16, y as u16))
                    .unwrap();
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
        if self.settings.debug {
            self.chip8.enable_debug();
        }
        self.chip8.memory[0x1FF] = 1;

        terminal::enable_raw_mode().unwrap();
        self.stdout.execute(terminal::EnterAlternateScreen).unwrap();
        self.stdout.execute(cursor::Hide).unwrap();
    }

    fn cleanup(&mut self) {
        self.stdout.execute(terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    fn load(&mut self, rom: Vec<u8>) {
        self.chip8.load_rom(&rom);
    }

    fn update(&mut self) {
        if event::poll(time::Duration::from_millis(1)).unwrap() {
            let ev = event::read().unwrap();
            self.handle_event(ev);
        }
        self.chip8.tick();
    }

    fn run(&mut self) {
        self.running = true;
        match self.settings.cycles {
            None => loop {
                if !self.running {
                    break;
                }
                self.cycle();
            },
            Some(cycles) => {
                for _ in 0..cycles {
                    if !self.running {
                        break;
                    }
                    self.cycle();
                }
            }
        }
    }
}

fn ch_to_key(c: char) -> Option<usize> {
    match c {
        '1' => Some(0x1),
        '2' => Some(0x2),
        '3' => Some(0x3),
        '4' => Some(0xC),
        'q' => Some(0x4),
        'w' => Some(0x5),
        'e' => Some(0x6),
        'r' => Some(0xD),
        'a' => Some(0x7),
        's' => Some(0x8),
        'd' => Some(0x9),
        'f' => Some(0xE),
        'z' => Some(0xA),
        'x' => Some(0x0),
        'c' => Some(0xB),
        'v' => Some(0xF),
        _ => None,
    }
}
