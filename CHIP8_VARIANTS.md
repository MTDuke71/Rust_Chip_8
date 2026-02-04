# CHIP-8 Variants and Extensions

## Overview

CHIP-8 has evolved over the decades, spawning several variants and extensions. This document explains the differences between them and what our emulator currently supports.

---

## Standard CHIP-8 (1970s) ✅ FULLY SUPPORTED

**Platform**: COSMAC VIP, DREAM 6800
**Resolution**: 64×32 pixels
**Opcodes**: 35 instructions
**Our Status**: ✅ **Complete implementation with all COSMAC VIP quirks**

### Key Features
- 4KB memory (0x000-0xFFF)
- 16 8-bit registers (V0-VF)
- 64×32 monochrome display
- 16-key hexadecimal keypad
- Delay and sound timers (60 Hz)
- 16-level stack
- 35 opcodes

### COSMAC VIP Quirks (All Implemented)
1. **VF RESET**: Logic operations (8xy1, 8xy2, 8xy3) set VF = 0
2. **MEMORY**: FX55/FX65 increment I register
3. **SHIFTING**: 8xy6/8xyE use Vy as source, store in Vx
4. **CLIPPING**: Sprites clip at screen edges (no wrapping during draw)
5. **DISP.WAIT**: VBlank synchronization (60 draws/sec)
6. **JUMPING**: Bnnn uses V0 (not Vx)

**Compatibility**: ~90% of CHIP-8 ROMs work perfectly

---

## Super-CHIP (S-CHIP) (1990) ❌ NOT SUPPORTED

**Platform**: HP48 graphing calculators
**Resolution**: 64×32 (low-res) or 128×64 (high-res)
**Our Status**: ❌ **Not implemented** (test 8 fails)

### Additional Features Beyond CHIP-8
- **High-resolution mode**: 128×64 pixels
- **Scrolling instructions**: Scroll display in any direction
- **Extended sprite size**: 16×16 pixels for large sprites
- **RPL variables**: Store values in HP48's calculator memory
- **Additional opcodes**: ~10 new instructions

### New Opcodes (Not Implemented)
- `00Cn` - Scroll display down N pixels
- `00FB` - Scroll display right 4 pixels
- `00FC` - Scroll display left 4 pixels
- `00FD` - Exit interpreter
- `00FE` - Disable high-res mode (return to 64×32)
- `00FF` - Enable high-res mode (128×64)
- `Dxy0` - Draw 16×16 sprite
- `Fx30` - Point I to 10-byte font character (0-9)
- `Fx75` - Store V0-VX in RPL variables
- `Fx85` - Load V0-VX from RPL variables

### Quirk Variations
Super-CHIP has **two** variants:
1. **Legacy Super-CHIP**: Original HP48 behavior
2. **Modern Super-CHIP**: Community-standardized behavior with fixes

Many quirks behave differently in low-res vs high-res mode.

### Why We Don't Support It
- Adds significant complexity (~10 new opcodes)
- High-res mode requires display buffer changes (128×64 vs 64×32)
- Scrolling operations need special handling
- Only needed for ~10% of ROMs (HP48-specific games)
- Would need mode switching logic

**Impact**: Some Super-CHIP games won't run (Octojam entries, HP48 ports)

---

## XO-CHIP (2014) ❌ NOT SUPPORTED

**Platform**: Modern emulators, Octo IDE
**Creator**: John Earnest
**Resolution**: Up to 128×64 pixels
**Our Status**: ❌ **Not implemented**

### Additional Features Beyond Super-CHIP
- **Extended memory**: Up to 64KB (vs 4KB)
- **Color support**: 4-color palette (vs monochrome)
- **Bitplanes**: Multiple drawing layers
- **Larger sprites**: Up to 256×256 pixels
- **Audio patterns**: Programmable audio beyond beep
- **More opcodes**: ~20 additional instructions

### New Opcodes (Not Implemented)
- `5xy2` - Save range of registers to memory
- `5xy3` - Load range of registers from memory
- `F000 nnnn` - Load I with 16-bit address
- `Fn01` - Select drawing plane (bitplane support)
- `F002` - Store 16 bytes in audio pattern buffer
- `Fx3A` - Set pitch for audio playback
- And many more...

### XO-CHIP Features
1. **Multiple bitplanes**: Draw in up to 4 color planes
2. **Extended audio**: Custom waveform patterns beyond simple beep
3. **Scrolling**: Like Super-CHIP but enhanced
4. **Memory pages**: Bank switching for large programs
5. **Long jumps**: 16-bit addressing beyond 4KB
6. **Save states**: Built-in state management

### Why We Don't Support It
- Very modern extension (2014) for contemporary game jams
- Requires major architectural changes:
  - 64KB memory vs 4KB
  - 4-color display vs monochrome
  - Multiple bitplanes
  - Audio synthesis system
- Primarily used by Octojam game jam entries
- Not needed for historical CHIP-8/Super-CHIP compatibility

**Impact**: Modern Octojam games won't run

---

## Comparison Table

| Feature | CHIP-8 | Super-CHIP | XO-CHIP |
|---------|--------|------------|---------|
| **Year** | 1977 | 1990 | 2014 |
| **Platform** | COSMAC VIP | HP48 Calculator | Octo IDE |
| **Resolution** | 64×32 | 64×32 or 128×64 | Up to 128×64 |
| **Colors** | Monochrome | Monochrome | 4 colors |
| **Memory** | 4KB | 4KB | Up to 64KB |
| **Opcodes** | 35 | ~45 | ~65+ |
| **Scrolling** | ❌ | ✅ | ✅ |
| **Large Sprites** | ❌ (8×15 max) | ✅ (16×16) | ✅ (256×256) |
| **Audio** | Simple beep | Simple beep | Programmable |
| **Our Support** | ✅ Complete | ❌ None | ❌ None |

---

## ROM Compatibility

### What Works With Our Emulator ✅
- **All standard CHIP-8 games**: Pong, Space Invaders, Tetris, Breakout, etc.
- **Timendus test suite**: Tests 1-7 (all standard CHIP-8 tests)
- **Public domain classics**: ~90% of vintage CHIP-8 software
- **COSMAC VIP programs**: Original 1970s-1980s software

### What Doesn't Work ❌
- **Super-CHIP games**: HP48 calculator games, some modern remakes
- **Timendus test suite**: Test 8 (scrolling test)
- **Octojam entries**: Modern game jam games (most use XO-CHIP)
- **High-res games**: Anything requiring 128×64 resolution

---

## Detection: How to Identify ROM Type

### Standard CHIP-8
- File size: Usually < 3.5KB (fits in 0x200-0xFFF)
- No opcodes beyond the standard 35
- Works on COSMAC VIP emulators

### Super-CHIP
- Uses high-res mode opcodes (0x00FE, 0x00FF)
- Scrolling opcodes (0x00Cn, 0x00FB, 0x00FC)
- Often has `.sc8` or `.s8` file extension
- May be labeled "Super-CHIP" or "SCHIP"

### XO-CHIP
- Uses extended opcodes (F000, 5xy2, 5xy3, etc.)
- May have `.xo8` file extension
- Often mentions "Octojam" or "XO-CHIP" in documentation
- Requires Octo IDE or XO-CHIP emulator

### Quick Test
Run the ROM:
- **Crashes on unknown opcode 0x00FE/0x00FF**: Super-CHIP
- **Crashes on unknown opcode 0xF000+**: XO-CHIP
- **Works perfectly**: Standard CHIP-8 ✅

---

## Future Enhancement Considerations

### Adding Super-CHIP Support

**Pros:**
- +10% ROM compatibility (HP48 games)
- Historically significant platform
- Well-documented behavior

**Cons:**
- Moderate complexity (~10 new opcodes)
- Requires display buffer redesign (128×64)
- Mode switching logic needed
- Two quirk sets to implement (legacy vs modern)

**Estimated Effort**: ~2-3 days

### Adding XO-CHIP Support

**Pros:**
- Access to modern Octojam games
- Color graphics support
- Enhanced audio capabilities

**Cons:**
- Major architectural changes needed:
  - Memory expansion (4KB → 64KB)
  - Color display system (1-bit → 2-bit per pixel)
  - Bitplane management
  - Audio synthesis
- Much more complex (~30+ new opcodes)
- Less historical significance

**Estimated Effort**: ~1-2 weeks

---

## Recommendation

### Current Implementation (Standard CHIP-8 Only) ✅

**Best For:**
- Learning emulator development
- Understanding computer architecture
- Running classic 1970s-1980s games
- Historical accuracy (COSMAC VIP)
- Minimal complexity

**Coverage**: ~90% of CHIP-8 software works

### If Adding Extensions

**Priority Order:**
1. **Super-CHIP first**: More ROMs benefit, less complex
2. **XO-CHIP second**: If you want modern games and color

**Alternative Approach:**
- Keep this as "pure CHIP-8" emulator
- Create separate Super-CHIP/XO-CHIP emulator project
- Use mode flags to toggle behavior

---

## Resources

### CHIP-8 Documentation
- [Cowgod's Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Tobias V. Langhoff's Guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [COSMAC VIP Research](https://laurencescotford.com/chip-8-on-the-cosmac-vip-index/)

### Super-CHIP Documentation
- [HP48 Super-CHIP Specification](https://github.com/Chromatophore/HP48-Superchip)
- [Gulrak's Extension Table](https://chip8.gulrak.net/)
- [Timendus Legacy Super-CHIP Doc](https://github.com/Timendus/chip8-test-suite/blob/main/legacy-superchip.md)

### XO-CHIP Documentation
- [XO-CHIP Specification](https://github.com/JohnEarnest/Octo/blob/gh-pages/docs/XO-ChipSpecification.md)
- [Octo IDE](http://octo-ide.com/)
- [Octojam Game Jam](https://itch.io/jam/octojam)

### Test Suites
- [Timendus CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite) (Tests all 3 platforms)

---

## Conclusion

Our emulator implements **100% of standard CHIP-8** with full COSMAC VIP quirks support. This represents the original, historically accurate platform and works with ~90% of CHIP-8 software.

**Super-CHIP** and **XO-CHIP** are later extensions that add features like high-resolution graphics, scrolling, and color. While popular for certain games, they're not necessary for a complete CHIP-8 experience.

For learning emulator development and running classic games, standard CHIP-8 is perfect. If you want to run modern Octojam games or HP48 software, you'll need the extensions.

---

*Last Updated: February 3, 2026*
*Based on v0.3 of this Rust CHIP-8 emulator*
