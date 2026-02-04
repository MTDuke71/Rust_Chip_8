# CHIP-8 ROMs Directory

Place your CHIP-8 ROM files (.ch8) in this directory.

## Where to Find ROMs

You can find CHIP-8 ROMs from various sources:

1. **Public Domain ROMs**: Many classic CHIP-8 games are in the public domain
2. **Test Suites**: 
   - [Timendus CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite)
   - Test programs for verifying emulator accuracy
3. **ROM Collections**: Search for "CHIP-8 ROMs" in retro gaming communities

## Popular Games

- `pong.ch8` - Classic two-player tennis
- `tetris.ch8` - Block stacking puzzle
- `space_invaders.ch8` - Alien shooter
- `breakout.ch8` - Brick breaker
- `brix.ch8` - Breakout variant
- `tictac.ch8` - Tic-tac-toe

## Running ROMs

```bash
# From the project root
cargo run --release -- roms/pong.ch8
```

## Legal Note

Only use ROMs that you have the legal right to use. Many classic CHIP-8 programs are freely available, but verify the licensing before use.
