# CHIP-8 Emulator Build Guide

A step-by-step guide to building your first emulator in Rust using **Test-Driven Development (TDD)**.

## 🎉 PROJECT STATUS: COMPLETE!

**All phases completed!** This guide documents the TDD journey of building a fully functional CHIP-8 emulator.

**Final Stats:**
- ✅ **107 tests passing** (86 unit + 10 integration + 11 disassembler)
- ✅ **All 35 CHIP-8 opcodes implemented** with individual tests
- ✅ **Complete emulator** with graphics, sound, keyboard, and timing
- ✅ **Bonus utilities** including disassembler
- ✅ **Comprehensive documentation** for future learners

**Working Features:**
- 64×32 monochrome display with sprite rendering
- 16-key keyboard input with modern key mapping
- Sound timer with 440 Hz square wave beep
- 700 Hz CPU execution (configurable 0.25x - 4.0x), 60 Hz timers
- Stack overflow/underflow protection
- Pause/Resume and Reset functionality
- Variable speed control
- Runs real ROMs (IBM Logo, Pong, etc.)

---

## TDD Workflow

For every feature, we follow the **Red-Green-Refactor** cycle:

1. **🔴 RED**: Write a failing test that defines the expected behavior
2. **🟢 GREEN**: Write the minimum code to make the test pass
3. **🔵 REFACTOR**: Clean up the code while keeping tests green

### Why TDD for an Emulator?

Emulators are **perfect** for TDD because:
- Each opcode has **precisely defined behavior**
- Inputs and outputs are **deterministic**
- Tests act as **living documentation**
- Bugs are caught **immediately**
- You can refactor with **confidence**

### Test Structure

```
src/
├── main.rs
├── lib.rs
├── cpu.rs
├── memory.rs
├── display.rs
├── keyboard.rs
└── timer.rs

tests/                    # Integration tests
├── cpu_tests.rs
├── opcode_tests.rs
└── integration_tests.rs
```

Each module also contains unit tests in `#[cfg(test)]` blocks.

---

## Overview

Building an emulator involves simulating hardware in software. For CHIP-8, we need to simulate:

1. **Memory** - Where programs and data live
2. **CPU** - Fetches, decodes, and executes instructions
3. **Display** - 64×32 pixel screen
4. **Input** - 16-key hexadecimal keypad
5. **Timers** - Delay and sound timers running at 60Hz

---

## Project Phases

## Project Phases

**Note:** These phases can be tackled in different orders. The guide originally suggested a top-down approach (CPU first), but we successfully used a **bottom-up approach** (hardware modules first, CPU last). Both are valid!

### ✅ Phase 1: Project Setup & Core Structure (COMPLETE)
- [x] Initialize Rust project with Cargo
- [x] Set up project structure (modules)
- [x] Choose and add dependencies (minifb, rand)
- [x] Create basic structs with stub implementations
- [x] Verify `cargo test` runs

### ✅ Phase 2A: Memory Module (COMPLETE - 6 tests, 100% coverage)
- [x] **TEST**: Memory reads/writes work correctly
- [x] **TEST**: Memory is initialized to zero
- [x] **TEST**: Font data is loaded at correct addresses
- [x] **TEST**: ROM loads starting at 0x200
- [x] **TEST**: Default trait works correctly
- [x] Implement 4KB memory array
- [x] Load font data into memory (0x000-0x04F)
- [x] Implement ROM loading (starts at 0x200)

### ✅ Phase 2B: CPU Registers & State (COMPLETE - 62 tests)
- [x] Implement registers (V0-VF, I, PC, SP)
- [x] Implement the stack (16 levels with overflow/underflow protection)
- [x] Implement delay/sound timers
- [x] **TEST**: CPU initializes with PC=0x200, all else zero
- [x] **TEST**: Stack overflow panics on 17th CALL
- [x] **TEST**: Stack underflow panics on RET with empty stack

### ✅ Phase 5: Display Module (COMPLETE - 10 tests, 100% coverage)
- [x] **TEST**: Display initializes to all black
- [x] **TEST**: CLS clears all pixels
- [x] **TEST**: DRW draws sprite at correct position
- [x] **TEST**: DRW XORs pixels (not overwrites)
- [x] **TEST**: DRW sets VF=1 on collision
- [x] **TEST**: DRW wraps sprites around screen edges
- [x] **TEST**: Multi-row sprites work correctly
- [x] **TEST**: to_buffer() converts to RGB format
- [x] **TEST**: Default trait works correctly
- [x] Implement 64×32 display buffer
- [x] Implement clear() function
- [x] Implement get_pixel() and set_pixel()
- [x] Implement draw_sprite() with XOR and collision detection
- [x] Implement to_buffer() for rendering

### ✅ Phase 6: Keyboard/Input Module (COMPLETE - 5 tests, 100% coverage)
- [x] **TEST**: Key state can be set and queried
- [x] **TEST**: Key press/release works correctly
- [x] **TEST**: get_pressed_key() returns first pressed key
- [x] **TEST**: get_pressed_key() returns None when no keys pressed
- [x] **TEST**: Default trait works correctly
- [x] Implement keypad state (16 keys)
- [x] Implement is_key_pressed()
- [x] Implement set_key()
- [x] Implement get_pressed_key()

### ✅ Phase 3 & 4: CPU Module (COMPLETE - 62 tests, all 35 opcodes!)

**This combines:**
- CPU registers and state (Phase 2B) ✅
- Fetch & decode logic (Phase 3) ✅
- Execute all opcodes (Phase 4) ✅
- Timers (Phase 7) ✅

**Registers & State:**
- [x] **TEST**: CPU initializes with correct defaults
- [x] Implement 16 registers (V0-VF)
- [x] Implement I register, PC, SP
- [x] Implement stack (16 levels with bounds checking)
- [x] Implement delay and sound timers

**Fetch & Decode:**
- [x] **TEST**: Fetch reads 2 bytes and combines correctly
- [x] **TEST**: PC increments by 2 after fetch
- [x] Implement instruction fetch (big-endian)
- [x] Implement opcode decoding (extract nnn, n, x, y, kk)

**Execute Opcodes (35 total - ALL IMPLEMENTED!):**
- [x] **TEST**: Each opcode individually tested (39 opcode tests + 2 panic tests)
- [x] Flow control: 00E0 (CLS), 00EE (RET), 1nnn (JP), 2nnn (CALL), Bnnn (JP V0)
- [x] Conditionals: 3xkk, 4xkk, 5xy0, 9xy0 (SE/SNE)
- [x] Register loads: 6xkk, 8xy0, Annn, Fx07, Fx0A, Fx15, Fx18, Fx29, Fx33, Fx55, Fx65
- [x] Math: 7xkk, 8xy4, 8xy5, 8xy7, Fx1E (ADD/SUB)
- [x] Bitwise: 8xy1, 8xy2, 8xy3, 8xy6, 8xyE (OR/AND/XOR/SHR/SHL)
- [x] Random: Cxkk (RND)
- [x] Display: Dxyn (DRW) - uses our Display module!
- [x] Keyboard: Ex9E, ExA1 (SKP/SKNP) - uses our Keyboard module!

**Timers:**
- [x] **TEST**: Timers decrement at 60Hz
- [x] **TEST**: Timers stop at zero
- [x] Implement timer tick function
- [x] Integrate timer opcodes (Fx07, Fx15, Fx18)

### ✅ Phase 8: Main Loop & Timing (COMPLETE)
- [x] Create main emulation loop with minifb window
- [x] Implement proper timing (700 Hz for CPU, 60Hz for timers)
- [x] Handle events (keyboard input, window close, ESC to exit)
- [x] Map modern keyboard to CHIP-8 keypad (1234/QWER/ASDF/ZXCV)

### ✅ Phase 9: Testing & Debugging (COMPLETE)
- [x] Test with simple ROMs (IBM logo works perfectly!)
- [x] Test with game ROMs (Pong plays with sound!)
- [x] Create comprehensive test suite (107 tests total)
- [x] Create integration tests (10 tests for end-to-end scenarios)
- [x] Add disassembler utility for debugging (11 tests)

### ✅ Phase 10: Polish (COMPLETE)
- [x] Add ROM loading from command line
- [x] Add sound support (440 Hz square wave beep)
- [x] Window rendering at 60 FPS
- [x] Clean error messages
- [x] Documentation (README, QUICKSTART, PROJECT_SUMMARY)
- [x] .gitignore for ROM files
- [x] Pause/Resume functionality (P key)
- [x] Reset emulator functionality (R key)
- [x] Configurable CPU speed (0.25x - 4.0x with +/- keys)

### 🎉 BONUS: Additional Features Implemented
- [x] **Disassembler utility** - View assembly code of any ROM
- [x] **Sound system** - Rodio-based audio with square wave synthesis
- [x] **Integration tests** - 10 tests for multi-component scenarios
- [x] **Stack protection** - Overflow/underflow detection with panic tests
- [x] **Complete documentation** - Multiple guides and summaries

---

## Opcode Test Plan

Every opcode gets its own test(s). Here's the pattern:

### Test Template

```rust
#[test]
fn test_opcode_6xkk_ld_vx_byte() {
    // ARRANGE: Set up initial state
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    
    // Load instruction: 6522 (LD V5, 0x22)
    memory.write(0x200, 0x65);
    memory.write(0x201, 0x22);
    
    // ACT: Execute one cycle
    cpu.cycle(&mut memory, &mut Display::new(), &Keyboard::new());
    
    // ASSERT: Check the result
    assert_eq!(cpu.v[5], 0x22);
    assert_eq!(cpu.pc, 0x202);
}
```

### Opcode Test Checklist

#### Flow Control
| Opcode | Test Cases |
|--------|------------|
| 1nnn JP | PC set to nnn |
| 2nnn CALL | PC set to nnn, return addr on stack, SP incremented |
| 00EE RET | PC set to stack value, SP decremented |
| Bnnn JP V0 | PC set to nnn + V0 |

#### Skip Instructions
| Opcode | Test Cases |
|--------|------------|
| 3xkk SE | Skip (PC+4) when Vx == kk, No skip (PC+2) when Vx != kk |
| 4xkk SNE | Skip when Vx != kk, No skip when Vx == kk |
| 5xy0 SE | Skip when Vx == Vy, No skip when Vx != Vy |
| 9xy0 SNE | Skip when Vx != Vy, No skip when Vx == Vy |

#### Load Instructions
| Opcode | Test Cases |
|--------|------------|
| 6xkk LD Vx, byte | Vx = kk |
| 8xy0 LD Vx, Vy | Vx = Vy (Vy unchanged) |
| Annn LD I | I = nnn |
| Fx07 LD Vx, DT | Vx = delay timer value |
| Fx15 LD DT, Vx | DT = Vx |
| Fx18 LD ST, Vx | ST = Vx |
| Fx29 LD F, Vx | I = font address for digit Vx |
| Fx33 LD B, Vx | BCD of Vx stored at I, I+1, I+2 |
| Fx55 LD [I], Vx | V0-Vx stored starting at I |
| Fx65 LD Vx, [I] | V0-Vx loaded from I |

#### Math Instructions
| Opcode | Test Cases |
|--------|------------|
| 7xkk ADD | Vx = Vx + kk (no carry flag) |
| 8xy4 ADD | Vx = Vx + Vy, VF=1 if overflow, VF=0 if not |
| 8xy5 SUB | Vx = Vx - Vy, VF=1 if no borrow, VF=0 if borrow |
| 8xy7 SUBN | Vx = Vy - Vx, VF=1 if no borrow, VF=0 if borrow |
| Fx1E ADD I | I = I + Vx |

#### Bitwise Instructions
| Opcode | Test Cases |
|--------|------------|
| 8xy1 OR | Vx = Vx OR Vy |
| 8xy2 AND | Vx = Vx AND Vy |
| 8xy3 XOR | Vx = Vx XOR Vy |
| 8xy6 SHR | Vx = Vx >> 1, VF = old LSB |
| 8xyE SHL | Vx = Vx << 1, VF = old MSB |

#### Other
| Opcode | Test Cases |
|--------|------------|
| 00E0 CLS | Display cleared |
| Cxkk RND | Vx = random AND kk (test that mask works) |
| Dxyn DRW | Sprite drawn, VF collision flag |
| Ex9E SKP | Skip if key pressed |
| ExA1 SKNP | Skip if key not pressed |
| Fx0A LD Vx, K | Blocks until key, stores key value |

---

## TDD Example Walkthrough

Let's see how TDD works for implementing `8xy4` (ADD Vx, Vy with carry):

### Step 1: Write Failing Tests 🔴

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8xy4_add_no_overflow() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 10;
        cpu.v[1] = 20;
        
        cpu.execute(0x8014); // ADD V0, V1
        
        assert_eq!(cpu.v[0], 30);
        assert_eq!(cpu.v[0xF], 0); // No carry
    }

    #[test]
    fn test_8xy4_add_with_overflow() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 250;
        cpu.v[1] = 10;
        
        cpu.execute(0x8014); // ADD V0, V1
        
        assert_eq!(cpu.v[0], 4);   // 260 & 0xFF = 4
        assert_eq!(cpu.v[0xF], 1); // Carry set
    }

    #[test]
    fn test_8xy4_add_exact_overflow() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 255;
        cpu.v[1] = 1;
        
        cpu.execute(0x8014);
        
        assert_eq!(cpu.v[0], 0);
        assert_eq!(cpu.v[0xF], 1);
    }
}
```

Run `cargo test` - all tests FAIL ❌

### Step 2: Implement to Pass 🟢

```rust
fn execute_8xy4(&mut self, x: usize, y: usize) {
    let sum = self.v[x] as u16 + self.v[y] as u16;
    self.v[0xF] = if sum > 255 { 1 } else { 0 };
    self.v[x] = sum as u8;
}
```

Run `cargo test` - all tests PASS ✅

### Step 3: Refactor 🔵

Code looks clean, but we could add edge case tests:

```rust
#[test]
fn test_8xy4_add_same_register() {
    let mut cpu = Cpu::new();
    cpu.v[5] = 100;
    
    cpu.execute(0x8554); // ADD V5, V5
    
    assert_eq!(cpu.v[5], 200);
    assert_eq!(cpu.v[0xF], 0);
}
```

---

## Step-by-Step Instructions

### Step 1: Initialize the Project

```bash
cargo new chip8_emulator
cd chip8_emulator
```

Add dependencies to `Cargo.toml`:
```toml
[dependencies]
minifb = "0.25"    # Simple framebuffer for display
rand = "0.8"       # Random number generation for Cxkk opcode
```

### Step 2: Create Project Structure

```
src/
├── main.rs          # Entry point, main loop
├── lib.rs           # Module exports
├── cpu.rs           # CPU state and instruction execution
├── memory.rs        # Memory management
├── display.rs       # Display buffer and rendering
├── keyboard.rs      # Input handling
└── timer.rs         # Delay and sound timers

tests/               # Integration tests
└── integration_tests.rs
```

### Step 3: Set Up for TDD

Create `src/lib.rs` to expose modules for testing:

```rust
pub mod cpu;
pub mod memory;
pub mod display;
pub mod keyboard;
```

Each module starts with stubs and tests:

```rust
// src/memory.rs
pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        todo!()
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        todo!()
    }
    
    pub fn write(&mut self, addr: u16, value: u8) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_new_is_zeroed() {
        let mem = Memory::new();
        assert_eq!(mem.read(0x000), 0);
        assert_eq!(mem.read(0x200), 0);
        assert_eq!(mem.read(0xFFF), 0);
    }

    #[test]
    fn test_memory_write_read() {
        let mut mem = Memory::new();
        mem.write(0x200, 0x42);
        assert_eq!(mem.read(0x200), 0x42);
    }
}
```

Run `cargo test` → Tests fail → Implement → Tests pass!

### Step 4: Implement Memory (TDD)

**TDD Cycle**:
1. Write test for `Memory::new()` → Fail → Implement → Pass
2. Write test for `read`/`write` → Fail → Implement → Pass  
3. Write test for font loading → Fail → Implement → Pass
4. Write test for ROM loading → Fail → Implement → Pass

**Goal**: Create a 4KB memory array and load fonts.

```rust
// memory.rs
pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self { /* ... */ }
    pub fn read(&self, addr: u16) -> u8 { /* ... */ }
    pub fn write(&mut self, addr: u16, value: u8) { /* ... */ }
    pub fn load_rom(&mut self, data: &[u8]) { /* ... */ }
}
```

**Key Points**:
- Font data goes at `0x000` to `0x04F` (80 bytes)
- ROMs load starting at `0x200`

### Step 5: Implement CPU State (TDD)

**TDD Cycle**:
1. Write test for `Cpu::new()` initial state → Fail → Implement → Pass
2. Write test for PC starting at 0x200 → Fail → Implement → Pass

**Goal**: Create the CPU struct with all registers.

```rust
// cpu.rs
pub struct Cpu {
    v: [u8; 16],      // V0-VF registers
    i: u16,           // Index register
    pc: u16,          // Program counter (starts at 0x200)
    sp: u8,           // Stack pointer
    stack: [u16; 16], // Call stack
    delay_timer: u8,
    sound_timer: u8,
}
```

### Step 6: Fetch-Decode-Execute Cycle (TDD)

**TDD Cycle**:
1. Write test for fetch combining 2 bytes → Fail → Implement → Pass
2. Write test for PC increment → Fail → Implement → Pass
3. Write test for opcode extraction → Fail → Implement → Pass

**Goal**: Implement the core CPU loop.

```rust
impl Cpu {
    pub fn cycle(&mut self, memory: &mut Memory, display: &mut Display, keyboard: &Keyboard) {
        // 1. FETCH: Read 2 bytes from memory at PC
        let opcode = self.fetch(memory);
        
        // 2. DECODE & EXECUTE: Match opcode and perform action
        self.execute(opcode, memory, display, keyboard);
    }
    
    fn fetch(&mut self, memory: &Memory) -> u16 {
        let hi = memory.read(self.pc) as u16;
        let lo = memory.read(self.pc + 1) as u16;
        self.pc += 2;
        (hi << 8) | lo
    }
}
```

### Step 7: Decode Opcodes

**Goal**: Extract the parts of an opcode.

Given opcode `0xDXYN`:
- First nibble (`D`) = instruction type
- `X` = register index
- `Y` = register index  
- `N` = 4-bit immediate value
- `NN` = 8-bit immediate value (lower byte)
- `NNN` = 12-bit address (lower 12 bits)

```rust
fn execute(&mut self, opcode: u16, ...) {
    let nnn = opcode & 0x0FFF;        // 12-bit address
    let nn = (opcode & 0x00FF) as u8;  // 8-bit constant
    let n = (opcode & 0x000F) as u8;   // 4-bit constant
    let x = ((opcode & 0x0F00) >> 8) as usize;  // Register X
    let y = ((opcode & 0x00F0) >> 4) as usize;  // Register Y
    
    match opcode & 0xF000 {
        0x0000 => { /* 00E0, 00EE, etc */ }
        0x1000 => { /* JP addr */ }
        0x2000 => { /* CALL addr */ }
        // ... etc
        _ => panic!("Unknown opcode: {:04X}", opcode),
    }
}
```

### Step 8: Implement Instructions with TDD

**For EACH opcode**:
1. 🔴 Write test(s) for the opcode
2. 🟢 Implement just enough to pass
3. 🔵 Refactor if needed
4. Move to next opcode

Start with the simplest instructions and build up:

**Round 1 - Basic Flow**:
- `00E0` - CLS
- `1nnn` - JP addr
- `6xkk` - LD Vx, byte
- `7xkk` - ADD Vx, byte
- `Annn` - LD I, addr
- `Dxyn` - DRW (basic version)

**Round 2 - Subroutines**:
- `2nnn` - CALL
- `00EE` - RET

**Round 3 - Conditionals**:
- `3xkk`, `4xkk` - SE/SNE with byte
- `5xy0`, `9xy0` - SE/SNE with register

**Round 4 - Math & Logic** (8xxx instructions):
- `8xy0` - LD
- `8xy1` - OR
- `8xy2` - AND
- `8xy3` - XOR
- `8xy4` - ADD with carry
- `8xy5` - SUB
- `8xy6` - SHR
- `8xy7` - SUBN
- `8xyE` - SHL

**Round 5 - Everything Else**:
- `Bnnn` - JP V0, addr
- `Cxkk` - RND
- `Ex9E`, `ExA1` - Key instructions
- `Fx07`, `Fx15`, `Fx18` - Timer instructions
- `Fx1E`, `Fx29`, `Fx33`, `Fx55`, `Fx65` - Memory/misc

### Step 8: Implement Display

**Goal**: 64×32 monochrome display with XOR drawing.

```rust
// display.rs
pub struct Display {
    pixels: [[bool; 64]; 32],  // or [u8; 64 * 32]
}

impl Display {
    pub fn clear(&mut self) { /* set all to false */ }
    
    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        // XOR each sprite byte onto screen
        // Return true if any pixel was erased (collision)
    }
}
```

**TDD for Display**:
```rust
#[test]
fn test_display_clear() {
    let mut display = Display::new();
    display.set_pixel(10, 10, true);
    display.clear();
    assert_eq!(display.get_pixel(10, 10), false);
}

#[test]
fn test_draw_sprite_basic() {
    let mut display = Display::new();
    let sprite = [0b11110000]; // 4 pixels on
    let collision = display.draw_sprite(0, 0, &sprite);
    
    assert_eq!(display.get_pixel(0, 0), true);
    assert_eq!(display.get_pixel(3, 0), true);
    assert_eq!(display.get_pixel(4, 0), false);
    assert_eq!(collision, false);
}

#[test]
fn test_draw_sprite_collision() {
    let mut display = Display::new();
    display.set_pixel(0, 0, true);
    
    let sprite = [0b10000000];
    let collision = display.draw_sprite(0, 0, &sprite);
    
    assert_eq!(display.get_pixel(0, 0), false); // XOR turned it off
    assert_eq!(collision, true);
}
```

**Drawing Algorithm**:
1. For each byte in sprite (row)
2. For each bit in byte (column)
3. If bit is 1, XOR with screen pixel
4. If screen pixel goes from 1 to 0, set collision flag
5. Wrap coordinates around screen edges

### Step 10: Main Loop

```rust
fn main() {
    // Initialize
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let mut keyboard = Keyboard::new();
    
    // Load ROM
    let rom = std::fs::read("rom.ch8").unwrap();
    memory.load_rom(&rom);
    
    // Create window
    let mut window = Window::new("CHIP-8", 640, 320, ...).unwrap();
    
    // Main loop
    while window.is_open() {
        // Run several CPU cycles per frame
        for _ in 0..10 {
            cpu.cycle(&mut memory, &mut display, &keyboard);
        }
        
        // Update timers at 60Hz
        cpu.tick_timers();
        
        // Update display
        window.update_with_buffer(&display.to_buffer(), 64, 32).unwrap();
        
        // Handle input
        keyboard.update(&window);
        
        // ~60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }
}
```

---

## Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test memory::tests

# Run a specific test
cargo test test_8xy4_add_no_overflow

# Run tests with output shown
cargo test -- --nocapture

# Run tests and stop on first failure
cargo test -- --test-threads=1
```

---

## Code Coverage

First, install the coverage tool (one-time setup):
```bash
cargo install cargo-llvm-cov
```

Then run coverage:
```bash
# Run coverage for all tests (console output)
cargo llvm-cov

# Run coverage for a specific module
cargo llvm-cov -- memory::tests

# Generate HTML report and open in browser
cargo llvm-cov --html --open

# Generate HTML report for a specific module
cargo llvm-cov --html --open -- memory::tests
```

Coverage reports are saved to `target/llvm-cov/html/`.

---

## Common Pitfalls

1. **Off-by-one in PC**: Remember PC increments by 2 (instructions are 2 bytes)

2. **Stack operations**: CALL increments SP then stores, RET loads then decrements

3. **VF is special**: Many instructions modify VF - always set it AFTER reading operands

4. **Draw wrapping**: Sprites wrap around screen edges, don't clip

5. **Timing**: Too fast = unplayable, too slow = boring. ~500Hz CPU is a good start

6. **Fx55/Fx65 quirk**: Original CHIP-8 incremented I, modern interpreters don't

---

## Test ROMs

Test your emulator with these (in order):

1. **IBM Logo** - Tests basic drawing, JP, LD, DRW
2. **test_opcode.ch8** - Tests all opcodes systematically
3. **BC_test.ch8** - BCD instruction test
4. **PONG** - Classic game, tests input and timers

---

## When You're Ready

Let me know when you want to start! We can work through each phase together:

- **Phase 1-2**: "Let's set up the project and implement memory"
- **Phase 3-4**: "Let's implement the CPU and basic instructions"
- **Phase 5**: "Let's get something on screen"
- etc.

I'll provide guidance, review your code, and help debug issues as we go.

---

## Resources

- [CHIP-8_Specification.md](Specification/CHIP-8_Specification.md) - Full opcode reference
- [Tobias V. Langhoff's Guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) - Excellent detailed guide
- [CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite) - Comprehensive test ROMs
