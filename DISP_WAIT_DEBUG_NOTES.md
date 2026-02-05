# DISP.WAIT Quirk Debugging Notes

**Last Updated:** February 4, 2026  
**Status:** ❌ Unresolved - Test shows SLOW

---

## The Problem

The `5-quirks.ch8` test ROM passes all quirk tests **except** DISP.WAIT, which displays "SLOW" instead of "ON" with a checkmark.

### What is DISP.WAIT?

On the original COSMAC VIP, the display was tied to the CPU interrupt. The `DRW` (draw sprite) instruction would **wait for the vertical blank interrupt** before drawing, effectively limiting execution to one draw per frame (60 Hz). This synchronized the display refresh with the CPU.

**Expected behavior:** When DISP.WAIT is ON, each `DRW` instruction should consume the remainder of the current frame, and the delay timer should decrement at exactly 60 Hz synchronized with these frame boundaries.

---

## How the 5-quirks Test Works

The test is located at addresses `0x076C` - `0x0798` in the ROM.

### Test Logic (Disassembled)

```
0x076C: LD V5, 0xB4      ; V5 = 180 (timer value)
0x076E: LD DT, V5        ; Set delay timer to 180
0x0770: LD V2, 0x00      ; V2 = 0 (outer loop counter)

OUTER_LOOP (0x0772):
0x0772: ADD V2, 0x01     ; V2++ (count outer loops)
0x0774: LD V3, 0x1E      ; V3 = 30 (inner loop count)

INNER_LOOP (0x0776):
0x0776: DRW V0, V1, 0x0  ; Draw sprite (triggers DISP.WAIT)
0x0778: ADD V3, 0xFF     ; V3--
0x077A: SE V3, 0x00      ; Skip if V3 == 0
0x077C: JP 0x0776        ; Loop back (30 DRWs per outer loop)

0x077E: LD V3, DT        ; V3 = current delay timer
0x0780: SE V3, 0x00      ; Skip if timer == 0
0x0782: JP 0x0772        ; Continue outer loop if timer > 0

; Timer is now 0, check V2
0x0784: LD V3, 0x06      ; V3 = 6 (expected value)
0x0786: SE V2, V3        ; If V2 == 6, show "ON"
0x0788: JP 0x078E        ; Else jump to error check
0x078A: JP 0x0296        ; SUCCESS - display "ON"

0x078E: LD V3, 0x06
0x0790: LD V4, V2
0x0792: SUB V4, V3       ; V4 = V2 - 6
0x0794: SE VF, 0x01      ; If no borrow (V2 >= 6)
0x0796: JP 0x02A0        ; Display "SLOW" (V2 < 6)
0x0798: JP 0x02BC        ; Display error (V2 > 6)
```

### Expected Calculation

- Timer starts at **180**
- Each outer loop runs **30 DRWs**
- With DISP.WAIT: 30 DRWs = 30 frames = 30 timer decrements
- Expected outer loops: **180 ÷ 30 = 6**
- **V2 must equal exactly 6 for "ON"**

| V2 Value | Result |
|----------|--------|
| V2 < 6   | SLOW   |
| V2 == 6  | ON ✓   |
| V2 > 6   | ERROR  |

---

## Current Implementation

### cpu.rs - cycle() returns bool

```rust
pub fn cycle(&mut self, memory: &Memory, display: &mut Display, keyboard: &Keyboard) -> bool {
    // ... execute opcode ...
    
    // Return true if DRW was executed (for DISP.WAIT)
    (opcode & 0xF000) == 0xD000
}
```

### main.rs - Frame loop with DISP.WAIT

```rust
const CYCLES_PER_FRAME: u32 = 11;

// Main loop
loop {
    let frame_start = Instant::now();
    
    // Decrement timers at start of frame
    cpu.tick_timers();
    
    // Run CPU cycles, break on DRW
    for _ in 0..CYCLES_PER_FRAME {
        let wait_for_vblank = cpu.cycle(&memory, &mut display, &keyboard);
        if wait_for_vblank {
            break;  // DRW executed, end frame early
        }
    }
    
    // Update display
    window.update_with_buffer(...);
    
    // Precise 60 Hz timing with spin-wait
    let frame_time = Duration::from_secs_f64(1.0 / 60.0);
    while frame_start.elapsed() < frame_time {
        std::hint::spin_loop();
    }
}
```

---

## What We Tried

### Attempt 1: Basic DISP.WAIT (Break after DRW)
- Made `cycle()` return `true` on DRW
- Main loop breaks when DRW detected
- **Result:** SLOW

### Attempt 2: Timer decrement AFTER CPU cycles
```rust
for _ in 0..CYCLES_PER_FRAME {
    if cpu.cycle(...) { break; }
}
cpu.tick_timers();  // After
```
- **Result:** SLOW

### Attempt 3: Timer decrement BEFORE CPU cycles
```rust
cpu.tick_timers();  // Before
for _ in 0..CYCLES_PER_FRAME {
    if cpu.cycle(...) { break; }
}
```
- **Result:** SLOW

### Attempt 4: Timer decrement ONLY when DRW executes
```rust
let mut did_drw = false;
for _ in 0..CYCLES_PER_FRAME {
    if cpu.cycle(...) { 
        did_drw = true;
        break; 
    }
}
if did_drw {
    cpu.tick_timers();
}
```
- **Result:** SLOW

### Attempt 5: Adjust CYCLES_PER_FRAME
- Tried values: 7, 11, 20
- **Result:** All SLOW

### Attempt 6: SDL2 for precise timing
- Attempted to switch from minifb to SDL2
- **Failed:** CMake/Visual Studio version mismatch
- Reverted to minifb

### Attempt 7: Precise spin-wait timing
- `set_target_fps(0)` to disable minifb's frame limiting
- Manual `sleep` + spin-wait loop for exact 60 Hz
- Achieved exactly **60.0 FPS** in debug output
- **Result:** Still SLOW

---

## Debug Output Analysis

During the test phase (when DISP.WAIT is being evaluated):

```
Cycles/sec: ~519
Frames/sec: 60.0
Avg cycles/frame: ~8.7
DRW breaks/sec: ~60-61
Timer decs/sec: ~60-61
```

The ratios look correct, but the test still shows SLOW.

### "FULL FRAME without DRW" Issue

We discovered frames where no DRW executed, causing extra timer decrements. But even fixing this (timer only on DRW) didn't help.

---

## Key Insights

1. **The timer and DRW counts seem synchronized** (60 DRW breaks ≈ 60 timer decrements)

2. **The test gets V2 < 6** which means the timer is running out too fast relative to the DRW loop iterations

3. **Possible issues:**
   - Timer decrementing when it shouldn't
   - DRW not properly halting the frame
   - Some edge case in the test loop we're not handling

4. **The instructions between DRWs** (in the inner loop: `DRW, ADD, SE, JP`) should be fast - only 4 instructions between draws

---

## Ideas for Next Session

### 1. Trace V2 Value
Add debug output to print V2 register value when the test completes:
```rust
// In cycle(), when jumping to result addresses
if pc == 0x0296 || pc == 0x02A0 || pc == 0x02BC {
    println!("V2 = {}", v[2]);
}
```

### 2. One Timer Dec Per DRW (1:1)
Instead of frame-based timing, decrement the timer exactly once per DRW:
```rust
if did_drw {
    cpu.tick_timers();
}
// Remove the frame-based timer tick
```

### 3. Check Octo's Implementation
Look at Octo emulator source code for exact DISP.WAIT logic.

### 4. Investigate Timer Decrement Timing
The test may be sensitive to WHEN in the frame the timer decrements relative to the DRW.

### 5. Consider Initial Frame State
What happens on the very first frame? Does the timer decrement before any DRW?

---

## Files to Reference

- [src/main.rs](src/main.rs) - Main emulation loop
- [src/cpu.rs](src/cpu.rs) - CPU with `cycle()` returning bool
- [roms/5-quirks_disasm.txt](roms/5-quirks_disasm.txt) - Full disassembly (if created)
- [Specification/CHIP-8_Specification.md](Specification/CHIP-8_Specification.md) - Reference spec
- [Specification/Opcodes.md](Specification/Opcodes.md) - Opcode details

---

## Current State of Code

- **CYCLES_PER_FRAME:** 11 (reset to reasonable default)
- **Timer decrement:** Before CPU cycles
- **DRW handling:** Returns true from `cycle()`, main loop breaks
- **Frame timing:** Precise 60 Hz with spin-wait
- **All other tests:** PASS ✓

---

## Resources

- [Timendus CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite)
- [Octo Emulator (reference)](https://github.com/JohnEarnest/Octo)
- [CHIP-8 Discord](https://discord.gg/chip8) - ArkoSammy12 provided hints

---

*Good luck! 🎮*
