# CHIP-8 Emulator

A fully functional CHIP-8 emulator written in Rust, featuring all 35 standard opcodes, display rendering, keyboard input, and sound timers. Built using Test-Driven Development (TDD) with 83 passing tests.

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

✅ **Complete and Functional!**

| Component | Status | Tests |
|-----------|--------|-------|
| Memory | ✅ Complete | 6 passing |
| CPU | ✅ Complete (35 opcodes) | 62 passing |
| Display | ✅ Complete | 10 passing |
| Keyboard | ✅ Complete | 5 passing |
| Timers | ✅ Complete | 4 passing |
| Main Loop | ✅ Complete | - |
| **Total** | **83 tests** | **All passing** |

### Features

- ✅ Complete CHIP-8 instruction set (35 opcodes)
- ✅ 64×32 monochrome display with sprite drawing
- ✅ 16-key hexadecimal keyboard input
- ✅ Stack overflow/underflow protection
- ✅ Delay and sound timers (60 Hz)
- ✅ 700 Hz CPU clock speed (configurable 0.25x - 4.0x)
- ✅ Window rendering with minifb
- ✅ Built-in disassembler utility
- ✅ Pause/Resume functionality
- ✅ Reset emulator on-the-fly
- ✅ Variable speed control

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
# Run with a ROM file
cargo run --release -- <path-to-rom>

# Example
cargo run --release -- roms/pong.ch8
```

### Disassemble a ROM

View the assembly code of any CHIP-8 ROM:

```bash
# Disassemble a ROM file
cargo run --bin disassembler -- <path-to-rom>

# Example
cargo run --bin disassembler -- roms/IBM_Logo.ch8
```

This will output the address, opcode, and instruction for each operation in the ROM.

## Keyboard Layout

The CHIP-8 hex keypad is mapped to your keyboard:

```
CHIP-8 Keypad:     Your Keyboard:
┌─┬─┬─┬─┐          ┌─┬─┬─┬─┐
│1│2│3│C│          │1│2│3│4│
├─┼─┼─┼─┤          ├─┼─┼─┼─┤
│4│5│6│D│          │Q│W│E│R│
├─┼─┼─┼─┤          ├─┼─┼─┼─┤
│7│8│9│E│          │A│S│D│F│
├─┼─┼─┼─┤          ├─┼─┼─┼─┤
│A│0│B│F│          │Z│X│C│V│
└─┴─┴─┴─┘          └─┴─┴─┴─┘
```

### Control Keys

- **P** - Pause/Resume emulation
- **R** - Reset emulator (reload ROM)
- **+/=** - Speed up (doubles speed, max 4.0x)
- **-/_** - Speed down (halves speed, min 0.25x)
- **ESC** - Exit emulator

The current speed and pause status are displayed in the window title.

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

## Architecture Details

### Hardware Components

- **Memory**: 4 KB RAM (0x000-0xFFF)
  - 0x000-0x1FF: Reserved for font sprites and interpreter
  - 0x200-0xFFF: Program/ROM space
- **Registers**: 16 8-bit general purpose (V0-VF)
  - VF is used as flag register (carry, borrow, collision)
- **Stack**: 16 levels, stores return addresses only
- **Display**: 64×32 pixels, monochrome, XOR sprite drawing
- **Keyboard**: 16 keys (0x0-0xF)

### Timing

- **CPU**: 700 Hz instruction execution
- **Timers**: 60 Hz decrement rate
- **Display**: 60 FPS update rate

### Special Behaviors

- **Stack**: Panics on overflow (17th call) or underflow (return on empty)
- **VF Register**: Automatically set by ALU, shift, and draw operations
- **Fx0A (Wait Key)**: Blocks by repeating instruction until key pressed
- **Display**: XOR sprite drawing with collision detection and screen wrapping
- **Timers**: Decrement independently at 60 Hz

## Finding ROMs

CHIP-8 ROMs are small (typically < 4 KB) and many are in the public domain. Classic games include:

- **Pong**: Two-player tennis game
- **Space Invaders**: Shoot descending aliens
- **Tetris**: Block-stacking puzzle
- **Breakout**: Brick-breaking game

You can find ROMs online through retro gaming communities. Create a `roms/` directory to organize your collection.

## Resources

- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite)

## License

MIT
