# CHIP-8 Emulator

A CHIP-8 emulator written in Rust, built using Test-Driven Development (TDD).

## What is CHIP-8?

CHIP-8 is a simple, interpreted programming language from the late 1970s and early 1980s. It was originally used on DIY computer systems like the COSMAC VIP and DREAM 6800. Building a CHIP-8 emulator is a great introduction to emulator development due to its simplicity.

### Specifications

- **Memory**: 4KB (4096 bytes)
- **Display**: 64×32 pixels, monochrome
- **Registers**: 16 general-purpose 8-bit registers (V0-VF)
- **Input**: 16-key hexadecimal keypad
- **Timers**: Delay timer and sound timer (60Hz)
- **Instructions**: 35 opcodes

## Project Status

🚧 **Work in Progress**

| Component | Status |
|-----------|--------|
| Memory | ⬜ Not implemented |
| CPU | ⬜ Not implemented |
| Display | ⬜ Not implemented |
| Keyboard | ⬜ Not implemented |
| Timers | ⬜ Not implemented |
| Main Loop | ⬜ Not implemented |

## Building

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)

### Build

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Run Emulator

```bash
cargo run -- <path-to-rom>
```

## Project Structure

```
src/
├── main.rs       # Entry point
├── lib.rs        # Module exports
├── cpu.rs        # CPU (fetch, decode, execute)
├── memory.rs     # 4KB RAM
├── display.rs    # 64×32 pixel display
└── keyboard.rs   # 16-key input

Specification/
├── CHIP-8_Specification.md   # Full technical reference
└── Cowgod's CHIP-8 Technical Reference.pdf
```

## Documentation

- [Build Guide](GUIDE.md) - Step-by-step guide for building this emulator
- [CHIP-8 Specification](Specification/CHIP-8_Specification.md) - Complete opcode reference

## Resources

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite)

## License

MIT
