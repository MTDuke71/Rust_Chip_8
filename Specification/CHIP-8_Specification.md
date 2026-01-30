# Cowgod's CHIP-8 Technical Reference v1.0

## Table of Contents

- [1.0 - About Chip-8](#10---about-chip-8)
- [2.0 - Chip-8 Specifications](#20---chip-8-specifications)
  - [2.1 - Memory](#21---memory)
  - [2.2 - Registers](#22---registers)
  - [2.3 - Keyboard](#23---keyboard)
  - [2.4 - Display](#24---display)
  - [2.5 - Timers & Sound](#25---timers--sound)
- [3.0 - Chip-8 Instructions](#30---chip-8-instructions)
  - [3.1 - Standard Chip-8 Instructions](#31---standard-chip-8-instructions)
  - [3.2 - Super Chip-48 Instructions](#32---super-chip-48-instructions)

---

## 1.0 - About Chip-8

Chip-8 is a simple, interpreted, programming language which was first used on some do-it-yourself computer systems in the late 1970s and early 1980s. The COSMAC VIP, DREAM 6800, and ETI 660 computers are a few examples. These computers typically were designed to use a television as a display, had between 1 and 4K of RAM, and used a 16-key hexadecimal keypad for input. The interpreter took up only 512 bytes of memory, and programs, which were entered into the computer in hexadecimal, were even smaller.

In the early 1990s, the Chip-8 language was revived by a man named Andreas Gustafsson. He created a Chip-8 interpreter for the HP48 graphing calculator, called Chip-48. The HP48 was lacking a way to easily make fast games at the time, and Chip-8 was the answer. Chip-48 later begat Super Chip-48, a modification of Chip-48 which allowed higher resolution graphics, as well as other graphical enhancements.

---

## 2.0 - Chip-8 Specifications

This section describes the Chip-8 memory, registers, display, keyboard, and timers.

### 2.1 - Memory

The Chip-8 language is capable of accessing up to **4KB (4,096 bytes)** of RAM, from location `0x000` (0) to `0xFFF` (4095). The first 512 bytes, from `0x000` to `0x1FF`, are where the original interpreter was located, and should not be used by programs.

Most Chip-8 programs start at location `0x200` (512), but some begin at `0x600` (1536). Programs beginning at `0x600` are intended for the ETI 660 computer.

#### Memory Map

```
+---------------+= 0xFFF (4095) End of Chip-8 RAM
|               |
|               |
|               |
|               |
|               |
| 0x200 to 0xFFF|
|     Chip-8    |
| Program / Data|
|     Space     |
|               |
|               |
|               |
+- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
|               |
|               |
|               |
+---------------+= 0x200 (512) Start of most Chip-8 programs
| 0x000 to 0x1FF|
| Reserved for  |
|  interpreter  |
+---------------+= 0x000 (0) Start of Chip-8 RAM
```

### 2.2 - Registers

Chip-8 has **16 general purpose 8-bit registers**, usually referred to as **Vx**, where x is a hexadecimal digit (0 through F). There is also a **16-bit register called I**. This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.

The **VF register** should not be used by any program, as it is used as a flag by some instructions. See section 3.0, Instructions for details.

Chip-8 also has two special purpose 8-bit registers, for the **delay timer** and **sound timer**. When these registers are non-zero, they are automatically decremented at a rate of 60Hz. See the section 2.5, Timers & Sound, for more information on these.

There are also some "pseudo-registers" which are not accessible from Chip-8 programs:

- **PC (Program Counter)**: 16-bit, used to store the currently executing address
- **SP (Stack Pointer)**: 8-bit, used to point to the topmost level of the stack

The **stack** is an array of 16 16-bit values, used to store the address that the interpreter should return to when finished with a subroutine. Chip-8 allows for up to 16 levels of nested subroutines.

#### Register Summary

| Register | Size | Description |
|----------|------|-------------|
| V0-VF | 8-bit each | General purpose registers |
| I | 16-bit | Index register (12 bits used) |
| PC | 16-bit | Program counter |
| SP | 8-bit | Stack pointer |
| DT | 8-bit | Delay timer |
| ST | 8-bit | Sound timer |
| Stack | 16 × 16-bit | Subroutine return addresses |

### 2.3 - Keyboard

The computers which originally used the Chip-8 Language had a 16-key hexadecimal keypad with the following layout:

```
+---+---+---+---+
| 1 | 2 | 3 | C |
+---+---+---+---+
| 4 | 5 | 6 | D |
+---+---+---+---+
| 7 | 8 | 9 | E |
+---+---+---+---+
| A | 0 | B | F |
+---+---+---+---+
```

This layout must be mapped into various other configurations to fit the keyboards of today's platforms.

#### Common Keyboard Mapping

```
Original:           Modern Keyboard:
+---+---+---+---+   +---+---+---+---+
| 1 | 2 | 3 | C |   | 1 | 2 | 3 | 4 |
+---+---+---+---+   +---+---+---+---+
| 4 | 5 | 6 | D |   | Q | W | E | R |
+---+---+---+---+   +---+---+---+---+
| 7 | 8 | 9 | E |   | A | S | D | F |
+---+---+---+---+   +---+---+---+---+
| A | 0 | B | F |   | Z | X | C | V |
+---+---+---+---+   +---+---+---+---+
```

### 2.4 - Display

The original implementation of the Chip-8 language used a **64x32-pixel monochrome display** with this format:

```
(0,0)                    (63,0)
  +------------------------+
  |                        |
  |                        |
  |                        |
  |                        |
  +------------------------+
(0,31)                  (63,31)
```

Some other interpreters, most notably the one on the ETI 660, also had 64x48 and 64x64 modes. More recently, Super Chip-48, an interpreter for the HP48 calculator, added a **128x64-pixel mode**.

#### Sprites

Chip-8 draws graphics on screen through the use of **sprites**. A sprite is a group of bytes which are a binary representation of the desired picture. Chip-8 sprites may be up to 15 bytes, for a possible sprite size of **8x15**.

Programs may also refer to a group of sprites representing the hexadecimal digits 0 through F. These sprites are 5 bytes long, or 8x5 pixels. The data should be stored in the interpreter area of Chip-8 memory (`0x000` to `0x1FF`).

#### Hexadecimal Font Sprites

```
"0"                 "1"                 "2"
****    0xF0          *     0x20        ****    0xF0
*  *    0x90         **     0x60           *    0x10
*  *    0x90          *     0x20        ****    0xF0
*  *    0x90          *     0x20        *       0x80
****    0xF0         ***    0x70        ****    0xF0

"3"                 "4"                 "5"
****    0xF0        *  *    0x90        ****    0xF0
   *    0x10        *  *    0x90        *       0x80
****    0xF0        ****    0xF0        ****    0xF0
   *    0x10           *    0x10           *    0x10
****    0xF0           *    0x10        ****    0xF0

"6"                 "7"                 "8"
****    0xF0        ****    0xF0        ****    0xF0
*       0x80           *    0x10        *  *    0x90
****    0xF0          *     0x20        ****    0xF0
*  *    0x90         *      0x40        *  *    0x90
****    0xF0         *      0x40        ****    0xF0

"9"                 "A"                 "B"
****    0xF0        ****    0xF0        ***     0xE0
*  *    0x90        *  *    0x90        *  *    0x90
****    0xF0        ****    0xF0        ***     0xE0
   *    0x10        *  *    0x90        *  *    0x90
****    0xF0        *  *    0x90        ***     0xE0

"C"                 "D"                 "E"
****    0xF0        ***     0xE0        ****    0xF0
*       0x80        *  *    0x90        *       0x80
*       0x80        *  *    0x90        ****    0xF0
*       0x80        *  *    0x90        *       0x80
****    0xF0        ***     0xE0        ****    0xF0

"F"
****    0xF0
*       0x80
****    0xF0
*       0x80
*       0x80
```

#### Font Data (Hex Array)

```rust
const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
```

### 2.5 - Timers & Sound

Chip-8 provides 2 timers, a **delay timer** and a **sound timer**.

**Delay Timer**: Active whenever the delay timer register (DT) is non-zero. This timer does nothing more than subtract 1 from the value of DT at a rate of **60Hz**. When DT reaches 0, it deactivates.

**Sound Timer**: Active whenever the sound timer register (ST) is non-zero. This timer also decrements at a rate of **60Hz**, however, as long as ST's value is greater than zero, the Chip-8 buzzer will sound. When ST reaches zero, the sound timer deactivates.

The sound produced by the Chip-8 interpreter has only one tone. The frequency of this tone is decided by the author of the interpreter.

---

## 3.0 - Chip-8 Instructions

The original implementation of the Chip-8 language includes **36 different instructions**, including math, graphics, and flow control functions. Super Chip-48 added an additional 10 instructions, for a total of 46.

All instructions are **2 bytes long** and are stored **most-significant-byte first** (big-endian). In memory, the first byte of each instruction should be located at an even address. If a program includes sprite data, it should be padded so any instructions following it will be properly situated in RAM.

### Instruction Variables

In these listings, the following variables are used:

| Variable | Description |
|----------|-------------|
| `nnn` or `addr` | A 12-bit value, the lowest 12 bits of the instruction |
| `n` or `nibble` | A 4-bit value, the lowest 4 bits of the instruction |
| `x` | A 4-bit value, the lower 4 bits of the high byte of the instruction |
| `y` | A 4-bit value, the upper 4 bits of the low byte of the instruction |
| `kk` or `byte` | An 8-bit value, the lowest 8 bits of the instruction |

### 3.1 - Standard Chip-8 Instructions

#### 0nnn - SYS addr
Jump to a machine code routine at nnn.

This instruction is only used on the old computers on which Chip-8 was originally implemented. **It is ignored by modern interpreters.**

---

#### 00E0 - CLS
Clear the display.

---

#### 00EE - RET
Return from a subroutine.

The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.

---

#### 1nnn - JP addr
Jump to location nnn.

The interpreter sets the program counter to nnn.

---

#### 2nnn - CALL addr
Call subroutine at nnn.

The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.

---

#### 3xkk - SE Vx, byte
Skip next instruction if Vx = kk.

The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.

---

#### 4xkk - SNE Vx, byte
Skip next instruction if Vx != kk.

The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.

---

#### 5xy0 - SE Vx, Vy
Skip next instruction if Vx = Vy.

The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.

---

#### 6xkk - LD Vx, byte
Set Vx = kk.

The interpreter puts the value kk into register Vx.

---

#### 7xkk - ADD Vx, byte
Set Vx = Vx + kk.

Adds the value kk to the value of register Vx, then stores the result in Vx.

---

#### 8xy0 - LD Vx, Vy
Set Vx = Vy.

Stores the value of register Vy in register Vx.

---

#### 8xy1 - OR Vx, Vy
Set Vx = Vx OR Vy.

Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corresponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.

---

#### 8xy2 - AND Vx, Vy
Set Vx = Vx AND Vy.

Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corresponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.

---

#### 8xy3 - XOR Vx, Vy
Set Vx = Vx XOR Vy.

Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corresponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.

---

#### 8xy4 - ADD Vx, Vy
Set Vx = Vx + Vy, set VF = carry.

The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255), VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.

---

#### 8xy5 - SUB Vx, Vy
Set Vx = Vx - Vy, set VF = NOT borrow.

If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.

---

#### 8xy6 - SHR Vx {, Vy}
Set Vx = Vx SHR 1.

If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.

---

#### 8xy7 - SUBN Vx, Vy
Set Vx = Vy - Vx, set VF = NOT borrow.

If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.

---

#### 8xyE - SHL Vx {, Vy}
Set Vx = Vx SHL 1.

If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.

---

#### 9xy0 - SNE Vx, Vy
Skip next instruction if Vx != Vy.

The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.

---

#### Annn - LD I, addr
Set I = nnn.

The value of register I is set to nnn.

---

#### Bnnn - JP V0, addr
Jump to location nnn + V0.

The program counter is set to nnn plus the value of V0.

---

#### Cxkk - RND Vx, byte
Set Vx = random byte AND kk.

The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.

---

#### Dxyn - DRW Vx, Vy, nibble
Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.

---

#### Ex9E - SKP Vx
Skip next instruction if key with the value of Vx is pressed.

Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.

---

#### ExA1 - SKNP Vx
Skip next instruction if key with the value of Vx is not pressed.

Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.

---

#### Fx07 - LD Vx, DT
Set Vx = delay timer value.

The value of DT is placed into Vx.

---

#### Fx0A - LD Vx, K
Wait for a key press, store the value of the key in Vx.

All execution stops until a key is pressed, then the value of that key is stored in Vx.

---

#### Fx15 - LD DT, Vx
Set delay timer = Vx.

DT is set equal to the value of Vx.

---

#### Fx18 - LD ST, Vx
Set sound timer = Vx.

ST is set equal to the value of Vx.

---

#### Fx1E - ADD I, Vx
Set I = I + Vx.

The values of I and Vx are added, and the results are stored in I.

---

#### Fx29 - LD F, Vx
Set I = location of sprite for digit Vx.

The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.

---

#### Fx33 - LD B, Vx
Store BCD representation of Vx in memory locations I, I+1, and I+2.

The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.

---

#### Fx55 - LD [I], Vx
Store registers V0 through Vx in memory starting at location I.

The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.

---

#### Fx65 - LD Vx, [I]
Read registers V0 through Vx from memory starting at location I.

The interpreter reads values from memory starting at location I into registers V0 through Vx.

---

### 3.2 - Super Chip-48 Instructions

| Opcode | Description |
|--------|-------------|
| 00Cn | SCD nibble - Scroll display n lines down |
| 00FB | SCR - Scroll display 4 pixels right |
| 00FC | SCL - Scroll display 4 pixels left |
| 00FD | EXIT - Exit interpreter |
| 00FE | LOW - Disable extended screen mode |
| 00FF | HIGH - Enable extended screen mode (128x64) |
| Dxy0 | DRW Vx, Vy, 0 - Draw 16x16 sprite |
| Fx30 | LD HF, Vx - Point I to 10-byte font sprite for digit Vx |
| Fx75 | LD R, Vx - Store V0..Vx in RPL user flags (x <= 7) |
| Fx85 | LD Vx, R - Read V0..Vx from RPL user flags (x <= 7) |

---

## Opcode Quick Reference Table

| Opcode | Mnemonic | Description |
|--------|----------|-------------|
| 0nnn | SYS addr | Jump to machine code routine (ignored) |
| 00E0 | CLS | Clear display |
| 00EE | RET | Return from subroutine |
| 1nnn | JP addr | Jump to address nnn |
| 2nnn | CALL addr | Call subroutine at nnn |
| 3xkk | SE Vx, byte | Skip if Vx == kk |
| 4xkk | SNE Vx, byte | Skip if Vx != kk |
| 5xy0 | SE Vx, Vy | Skip if Vx == Vy |
| 6xkk | LD Vx, byte | Set Vx = kk |
| 7xkk | ADD Vx, byte | Set Vx = Vx + kk |
| 8xy0 | LD Vx, Vy | Set Vx = Vy |
| 8xy1 | OR Vx, Vy | Set Vx = Vx OR Vy |
| 8xy2 | AND Vx, Vy | Set Vx = Vx AND Vy |
| 8xy3 | XOR Vx, Vy | Set Vx = Vx XOR Vy |
| 8xy4 | ADD Vx, Vy | Set Vx = Vx + Vy, VF = carry |
| 8xy5 | SUB Vx, Vy | Set Vx = Vx - Vy, VF = !borrow |
| 8xy6 | SHR Vx | Set Vx = Vx >> 1, VF = LSB |
| 8xy7 | SUBN Vx, Vy | Set Vx = Vy - Vx, VF = !borrow |
| 8xyE | SHL Vx | Set Vx = Vx << 1, VF = MSB |
| 9xy0 | SNE Vx, Vy | Skip if Vx != Vy |
| Annn | LD I, addr | Set I = nnn |
| Bnnn | JP V0, addr | Jump to nnn + V0 |
| Cxkk | RND Vx, byte | Set Vx = random AND kk |
| Dxyn | DRW Vx, Vy, n | Draw sprite at (Vx, Vy), VF = collision |
| Ex9E | SKP Vx | Skip if key Vx pressed |
| ExA1 | SKNP Vx | Skip if key Vx not pressed |
| Fx07 | LD Vx, DT | Set Vx = delay timer |
| Fx0A | LD Vx, K | Wait for key, store in Vx |
| Fx15 | LD DT, Vx | Set delay timer = Vx |
| Fx18 | LD ST, Vx | Set sound timer = Vx |
| Fx1E | ADD I, Vx | Set I = I + Vx |
| Fx29 | LD F, Vx | Set I = sprite location for digit Vx |
| Fx33 | LD B, Vx | Store BCD of Vx at I, I+1, I+2 |
| Fx55 | LD [I], Vx | Store V0..Vx at I |
| Fx65 | LD Vx, [I] | Load V0..Vx from I |

---

## Credits

This document was compiled by Thomas P. Greene (cowgod@rockpile.com).

Sources include:
- David Winter's Chip-8 Emulator documentation
- Christian Egeberg's Chipper documentation
- Marcel de Kogel's Vision-8 source code
- Paul Hayter's DREAM MON documentation
- Paul Robson's web page
- Andreas Gustafsson's Chip-48 documentation

Original document: August 30, 1997
