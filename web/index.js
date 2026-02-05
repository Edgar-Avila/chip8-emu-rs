import init, * as wasm from "./pkg/wasm_platform.js";

await init();
const romInput = document.getElementById("romInput");
const canvas = document.getElementById("canvas");
const chip8 = new wasm.WasmPlatform();
const ctx = canvas.getContext("2d");
let loaded = false;
const chip8Width = 64;
const chip8Height = 32;
const clocksPerSec = 1000;

const animFrame = () => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    const pixelWidth = canvas.width / chip8Width;
    const pixelHeight = canvas.height / chip8Height;
    if(loaded) {
        for(let y=0;y<chip8Height;y++) {
            for(let x=0;x<chip8Width;x++) {
                const pixelOn = chip8.pixel_at(x, y);
                const pixelX = x * pixelWidth;
                const pixelY = y * pixelHeight;
                const pixelColor = pixelOn ? "#ffffff" : "#000000";
                ctx.fillStyle = pixelColor;
                ctx.fillRect(pixelX, pixelY, pixelWidth, pixelHeight);
            }
        }
    }
    requestAnimationFrame(animFrame);
};

const onInit = () => {
    canvas.width = 1280;
    canvas.height = 640;
    requestAnimationFrame(animFrame);
    setInterval(() => {
        if(loaded) {
            chip8.tick();
        }
    }, 1000/clocksPerSec);
};

romInput.addEventListener("change", (ev) => {
    const romFile = ev.target.files[0];
    const reader = new FileReader();
    const onReaderLoad = (loadEvent) => {
        const arr = new Uint8Array(loadEvent.target.result);
        chip8.load_rom(arr);
        loaded = true;
    };

    reader.onload = onReaderLoad;
    reader.readAsArrayBuffer(romFile);
});

window.addEventListener("keydown", (ev) => {
    chip8.keypress(ev.key, true);
});

window.addEventListener("keyup", (ev) => {
    chip8.keypress(ev.key, false);
});

onInit();
