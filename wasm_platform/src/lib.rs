use chip8::Chip8;
use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

#[wasm_bindgen]
pub struct WasmPlatform {
    chip8: Chip8
}

#[wasm_bindgen]
impl WasmPlatform {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmPlatform {
        WasmPlatform {
            chip8: Chip8::new(),
        }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.chip8.tick();
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, c: char, pressed: bool) {
        if let Some(k) = ch_to_key(c) {
            self.chip8.keypress(k, pressed);
        }
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, data: Uint8Array) {
        let v = data.to_vec();
        self.chip8.load_rom(&v);
    }

    #[wasm_bindgen]
    pub fn pixel_at(&mut self, x: u16, y: u16) -> bool {
        self.chip8.pixel_at(x, y) == 1
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
