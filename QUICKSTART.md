# Quick Start Guide

## Installation

No installation needed - just build from source:

```bash
cd d:\repos\Rust_Chip_8
cargo build --release
```

The optimized binary will be at: `target\release\chip8_emulator.exe`

## Running Your First ROM

### Step 1: Get a ROM

Download a CHIP-8 ROM (`.ch8` file) and place it in the `roms/` directory.

For testing, you can find public domain ROMs at:
- https://github.com/kripod/chip8-roms
- https://github.com/Timendus/chip8-test-suite

### Step 2: Run the Emulator

```bash
cargo run --release -- roms/yourrom.ch8
```

Or directly:
```bash
target\release\chip8_emulator.exe roms/yourrom.ch8
```

### Step 3: Play!

Use your keyboard as the CHIP-8 keypad:

```
Your Keyboard → CHIP-8 Key
1 → 1    2 → 2    3 → 3    4 → C
Q → 4    W → 5    E → 6    R → D
A → 7    S → 8    D → 9    F → E
Z → A    X → 0    C → B    V → F
```

**Control Keys:**
- **P** - Pause/Resume
- **R** - Reset (reload ROM)
- **+/=** - Speed up (2x, 4x)
- **-/_** - Speed down (0.5x, 0.25x)
- **ESC** - Exit

The current speed and pause status appear in the window title.

## Testing

Verify everything works:

```bash
cargo test
```

Expected output: **83 tests passed**

## Troubleshooting

### "No such file or directory"
- Make sure the ROM path is correct
- Use forward slashes `/` or double backslashes `\\` in paths

### Window doesn't open
- Make sure minifb dependencies are installed
- Check graphics drivers are up to date

### ROM doesn't load
- Verify ROM file is valid CHIP-8 format
- Check file size (should be ≤ 3584 bytes)
- ROM must fit in memory from 0x200 to 0xFFF

### Game runs too fast/slow
- Use **+/-** keys to adjust speed (0.25x to 4.0x)
- Default speed is 1.0x (700 Hz CPU)
- Some ROMs may work better at different speeds

## Example Commands

```bash
# Test the emulator (no ROM needed)
cargo test

# Show usage help
cargo run --release

# Run Pong
cargo run --release -- roms/pong.ch8

# Run in debug mode (with full output)
cargo run -- roms/test.ch8
```

## Next Steps

1. Try different ROMs to see various games
2. Read [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) for technical details
3. Explore the source code in `src/` directory
4. Check out the test suite in `src/cpu.rs`, `src/display.rs`, etc.

## Performance

- **CPU**: 700 instructions per second
- **Timers**: 60 Hz update rate
- **Display**: 60 FPS
- **Memory**: 4 KB total

The emulator is optimized for accuracy, not maximum speed. It matches the original CHIP-8 timing specification.
