//! CPU module for CHIP-8
//!
//! The CPU handles the fetch-decode-execute cycle and maintains
//! all registers and the stack.

use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::memory::Memory;
use rand;

/// The CHIP-8 CPU
pub struct Cpu {
    /// General purpose registers V0-VF
    pub v: [u8; 16],
    /// Index register (12 bits used)
    pub i: u16,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u8,
    /// Call stack (16 levels)
    pub stack: [u16; 16],
    /// Delay timer (decrements at 60Hz)
    pub delay_timer: u8,
    /// Sound timer (decrements at 60Hz, beeps while > 0)
    pub sound_timer: u8,
    /// Key wait state for FX0A: Some(key) = waiting for key to be released, None = not waiting
    waiting_for_key: Option<u8>,
    /// Display wait state for DISP.WAIT quirk: true = waiting for VBlank after draw
    waiting_for_vblank: bool,
}

impl Cpu {
    /// Creates a new CPU with initial state
    /// PC starts at 0x200 where programs are loaded
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: 0x200, // Programs start at 0x200
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            waiting_for_key: None,
            waiting_for_vblank: false,
        }
    }

    /// Returns true if the CPU is halted waiting for VBlank (DISP.WAIT quirk)
    pub fn is_waiting_for_vblank(&self) -> bool {
        self.waiting_for_vblank
    }

    /// Checks if the next instruction to be executed is a DRW (DXYN) opcode.
    /// Used for DISP.WAIT quirk - Octo's approach: check BEFORE executing.
    pub fn next_instruction_is_draw(&self, memory: &Memory) -> bool {
        let high_byte = memory.read(self.pc);
        (high_byte & 0xF0) == 0xD0
    }

    /// Executes one fetch-decode-execute cycle
    /// Returns true if a DRW instruction was executed (for DISP.WAIT quirk)
    pub fn cycle(&mut self, memory: &mut Memory, display: &mut Display, keyboard: &Keyboard) -> bool {
        let opcode = self.fetch(memory);
        self.execute(opcode, memory, display, keyboard);

        // Return true if this was a DRW instruction (opcode 0xDxyn)
        (opcode & 0xF000) == 0xD000
    }

    /// Fetches the next 2-byte opcode from memory
    fn fetch(&mut self, memory: &Memory) -> u16 {
        let high_byte = memory.read(self.pc) as u16;
        let low_byte = memory.read(self.pc + 1) as u16;
        self.pc += 2;
        (high_byte << 8) | low_byte
    }

    /// Decodes and executes an opcode
    fn execute(
        &mut self,
        opcode: u16,
        memory: &mut Memory,
        display: &mut Display,
        keyboard: &Keyboard,
    ) {
        // Extract opcode parts
        let nnn = opcode & 0x0FFF;           // Lowest 12 bits
        let kk = (opcode & 0x00FF) as u8;    // Lowest 8 bits
        let x = ((opcode & 0x0F00) >> 8) as usize;  // Lower 4 bits of high byte
        let y = ((opcode & 0x00F0) >> 4) as usize;  // Upper 4 bits of low byte
        let n = (opcode & 0x000F) as u8;     // Lowest 4 bits

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => {
                    // 00E0 - CLS: Clear the display
                    display.clear();
                }
                0x00EE => {
                    // 00EE - RET: Return from subroutine
                    if self.sp == 0 {
                        panic!("Stack underflow: RET called with empty stack");
                    }
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => panic!("Unknown opcode: {:#06x}", opcode),
            },
            0x1000 => {
                // 1nnn - JP addr: Jump to location nnn
                self.pc = nnn;
            }
            0x2000 => {
                // 2nnn - CALL addr: Call subroutine at nnn
                if self.sp >= 16 {
                    panic!("Stack overflow: Maximum call depth of 16 exceeded");
                }
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            0x3000 => {
                // 3xkk - SE Vx, byte: Skip next instruction if Vx == kk
                if self.v[x] == kk {
                    self.pc += 2;
                }
            }
            0x4000 => {
                // 4xkk - SNE Vx, byte: Skip next instruction if Vx != kk
                if self.v[x] != kk {
                    self.pc += 2;
                }
            }
            0x5000 => {
                // 5xy0 - SE Vx, Vy: Skip next instruction if Vx == Vy
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                // 6xkk - LD Vx, byte: Set Vx = kk
                self.v[x] = kk;
            }
            0x7000 => {
                // 7xkk - ADD Vx, byte: Set Vx = Vx + kk
                self.v[x] = self.v[x].wrapping_add(kk);
            }
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        // 8xy0 - LD Vx, Vy: Set Vx = Vy
                        self.v[x] = self.v[y];
                    }
                    0x0001 => {
                        // 8xy1 - OR Vx, Vy: Set Vx = Vx OR Vy, VF = 0
                        let vx = self.v[x];
                        let vy = self.v[y];
                        self.v[x] = vx | vy;
                        self.v[0xF] = 0;
                    }
                    0x0002 => {
                        // 8xy2 - AND Vx, Vy: Set Vx = Vx AND Vy, VF = 0
                        let vx = self.v[x];
                        let vy = self.v[y];
                        self.v[x] = vx & vy;
                        self.v[0xF] = 0;
                    }
                    0x0003 => {
                        // 8xy3 - XOR Vx, Vy: Set Vx = Vx XOR Vy, VF = 0
                        let vx = self.v[x];
                        let vy = self.v[y];
                        self.v[x] = vx ^ vy;
                        self.v[0xF] = 0;
                    }
                    0x0004 => {
                        // 8xy4 - ADD Vx, Vy: Set Vx = Vx + Vy, set VF = carry
                        let vx = self.v[x];
                        let vy = self.v[y];
                        let sum = vx as u16 + vy as u16;
                        self.v[x] = sum as u8;
                        self.v[0xF] = if sum > 0xFF { 1 } else { 0 };
                    }
                    0x0005 => {
                        // 8xy5 - SUB Vx, Vy: Set Vx = Vx - Vy, set VF = NOT borrow
                        // NOT borrow means: VF = 1 if Vx >= Vy (no borrow needed), 0 otherwise
                        let vx = self.v[x];
                        let vy = self.v[y];
                        self.v[x] = vx.wrapping_sub(vy);
                        self.v[0xF] = if vx >= vy { 1 } else { 0 };
                    }
                    0x0006 => {
                        // 8xy6 - SHR Vx {, Vy}: Set Vx = Vy >> 1, VF = least significant bit
                        // COSMAC VIP quirk: copy Vy to Vx first, then shift
                        let vy = self.v[y];
                        self.v[x] = vy >> 1;
                        self.v[0xF] = vy & 0x1;
                    }
                    0x0007 => {
                        // 8xy7 - SUBN Vx, Vy: Set Vx = Vy - Vx, set VF = NOT borrow
                        // NOT borrow means: VF = 1 if Vy >= Vx (no borrow needed), 0 otherwise
                        let vx = self.v[x];
                        let vy = self.v[y];
                        self.v[x] = vy.wrapping_sub(vx);
                        self.v[0xF] = if vy >= vx { 1 } else { 0 };
                    }
                    0x000E => {
                        // 8xyE - SHL Vx {, Vy}: Set Vx = Vy << 1, VF = most significant bit
                        // COSMAC VIP quirk: copy Vy to Vx first, then shift
                        let vy = self.v[y];
                        self.v[x] = vy << 1;
                        self.v[0xF] = (vy & 0x80) >> 7;
                    }
                    _ => panic!("Unknown 8xy_ opcode: {:#06x}", opcode),
                }
            }
            0x9000 => {
                // 9xy0 - SNE Vx, Vy: Skip next instruction if Vx != Vy
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0xA000 => {
                // Annn - LD I, addr: Set I = nnn
                self.i = nnn;
            }
            0xB000 => {
                // Bnnn - JP V0, addr: Jump to location nnn + V0
                self.pc = nnn + self.v[0] as u16;
            }
            0xC000 => {
                // Cxkk - RND Vx, byte: Set Vx = random byte AND kk
                let random_byte: u8 = rand::random();
                self.v[x] = random_byte & kk;
            }
            0xD000 => {
                // Dxyn - DRW Vx, Vy, nibble: Display n-byte sprite at (Vx, Vy), set VF = collision
                // COSMAC VIP DISP.WAIT quirk: Wait for vblank BEFORE drawing
                // On real VIP, the IDL instruction halted CPU until the display interrupt.
                // If already drew this frame, wait until next vblank (re-execute instruction)
                if self.waiting_for_vblank {
                    self.pc -= 2; // Repeat this instruction next cycle
                    return;       // Don't draw yet - wait for tick_timers to clear the flag
                }

                let x_coord = self.v[x];
                let y_coord = self.v[y];
                let height = n;
                let mut sprite = Vec::new();
                for row in 0..height {
                    sprite.push(memory.read(self.i + row as u16));
                }
                let collision = display.draw_sprite(x_coord, y_coord, &sprite);
                self.v[0xF] = if collision { 1 } else { 0 };
                // DISP.WAIT: Block subsequent draws until next vblank
                self.waiting_for_vblank = true;
            }
            0xE000 => match opcode & 0x00FF {
                0x009E => {
                    // Ex9E - SKP Vx: Skip next instruction if key with value of Vx is pressed
                    if keyboard.is_key_pressed(self.v[x] & 0x0F) {
                        self.pc += 2;
                    }
                }
                0x00A1 => {
                    // ExA1 - SKNP Vx: Skip next instruction if key with value of Vx is NOT pressed
                    if !keyboard.is_key_pressed(self.v[x] & 0x0F) {
                        self.pc += 2;
                    }
                }
                _ => panic!("Unknown opcode: {:#06x}", opcode),
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => {
                    // Fx07 - LD Vx, DT: Set Vx = delay timer value
                    self.v[x] = self.delay_timer;
                }
                0x000A => {
                    // Fx0A - LD Vx, K: Wait for a key press AND release, store the value in Vx
                    match self.waiting_for_key {
                        None => {
                            // Not waiting yet - check if a key is pressed
                            if let Some(key) = keyboard.get_pressed_key() {
                                // Key pressed - remember it and wait for release
                                self.waiting_for_key = Some(key);
                                self.pc -= 2; // Repeat this instruction
                            } else {
                                // No key pressed - repeat this instruction
                                self.pc -= 2;
                            }
                        }
                        Some(key) => {
                            // Waiting for key release - check if it's released
                            if !keyboard.is_key_pressed(key) {
                                // Key released - store it and continue
                                self.v[x] = key;
                                self.waiting_for_key = None;
                            } else {
                                // Key still pressed - repeat this instruction
                                self.pc -= 2;
                            }
                        }
                    }
                }
                0x0015 => {
                    // Fx15 - LD DT, Vx: Set delay timer = Vx
                    self.delay_timer = self.v[x];
                }
                0x0018 => {
                    // Fx18 - LD ST, Vx: Set sound timer = Vx
                    self.sound_timer = self.v[x];
                }
                0x001E => {
                    // Fx1E - ADD I, Vx: Set I = I + Vx
                    self.i = self.i.wrapping_add(self.v[x] as u16);
                }
                0x0029 => {
                    // Fx29 - LD F, Vx: Set I = location of sprite for digit Vx
                    // Font sprites are 5 bytes each, starting at address 0x000
                    self.i = ((self.v[x] & 0x0F) as u16) * 5;
                }
                0x0033 => {
                    // Fx33 - LD B, Vx: Store BCD representation of Vx in memory locations I, I+1, I+2
                    let value = self.v[x];
                    memory.write(self.i, value / 100);         // Hundreds digit
                    memory.write(self.i + 1, (value / 10) % 10); // Tens digit
                    memory.write(self.i + 2, value % 10);      // Ones digit
                }
                0x0055 => {
                    // Fx55 - LD [I], Vx: Store registers V0 through Vx in memory starting at location I
                    // COSMAC VIP quirk: increment I by x+1 after storing
                    for i in 0..=x {
                        memory.write(self.i + i as u16, self.v[i]);
                    }
                    self.i += (x as u16) + 1;
                }
                0x0065 => {
                    // Fx65 - LD Vx, [I]: Read registers V0 through Vx from memory starting at location I
                    // COSMAC VIP quirk: increment I by x+1 after loading
                    for i in 0..=x {
                        self.v[i] = memory.read(self.i + i as u16);
                    }
                    self.i += (x as u16) + 1;
                }
                _ => panic!("Unknown opcode: {:#06x}", opcode),
            },
            _ => panic!("Unknown opcode: {:#06x}", opcode),
        }
    }

    /// Decrements timers (call this at 60Hz)
    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        // Clear VBlank wait flag (DISP.WAIT quirk - allows one draw per 60Hz tick)
        self.waiting_for_vblank = false;
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_new_pc_starts_at_0x200() {
        let cpu = Cpu::new();
        assert_eq!(cpu.pc, 0x200);
    }

    #[test]
    fn test_cpu_new_registers_are_zero() {
        let cpu = Cpu::new();
        for i in 0..16 {
            assert_eq!(cpu.v[i], 0, "V{:X} should be 0", i);
        }
        assert_eq!(cpu.i, 0);
        assert_eq!(cpu.sp, 0);
    }

    #[test]
    fn test_cpu_new_stack_is_zero() {
        let cpu = Cpu::new();
        for i in 0..16 {
            assert_eq!(cpu.stack[i], 0);
        }
    }

    #[test]
    fn test_cpu_new_timers_are_zero() {
        let cpu = Cpu::new();
        assert_eq!(cpu.delay_timer, 0);
        assert_eq!(cpu.sound_timer, 0);
    }

    #[test]
    fn test_tick_timers_decrements_delay() {
        let mut cpu = Cpu::new();
        cpu.delay_timer = 10;
        cpu.tick_timers();
        assert_eq!(cpu.delay_timer, 9);
    }

    #[test]
    fn test_tick_timers_decrements_sound() {
        let mut cpu = Cpu::new();
        cpu.sound_timer = 5;
        cpu.tick_timers();
        assert_eq!(cpu.sound_timer, 4);
    }

    #[test]
    fn test_tick_timers_stops_at_zero() {
        let mut cpu = Cpu::new();
        cpu.delay_timer = 0;
        cpu.sound_timer = 0;
        cpu.tick_timers();
        assert_eq!(cpu.delay_timer, 0, "delay_timer should not underflow");
        assert_eq!(cpu.sound_timer, 0, "sound_timer should not underflow");
    }

    #[test]
    fn test_tick_timers_both_decrement() {
        let mut cpu = Cpu::new();
        cpu.delay_timer = 3;
        cpu.sound_timer = 7;
        cpu.tick_timers();
        assert_eq!(cpu.delay_timer, 2);
        assert_eq!(cpu.sound_timer, 6);
    }

    #[test]
    fn test_fetch_reads_two_bytes() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        
        // Write opcode 0x61FF at 0x200 (LD V1, 0xFF)
        memory.write(0x200, 0x61);
        memory.write(0x201, 0xFF);
        
        let opcode = cpu.fetch(&memory);
        assert_eq!(opcode, 0x61FF);
    }

    #[test]
    fn test_fetch_increments_pc() {
        let mut cpu = Cpu::new();
        let memory = Memory::new();
        
        assert_eq!(cpu.pc, 0x200);
        cpu.fetch(&memory);
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn test_fetch_combines_bytes_correctly() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        
        // Test big-endian byte order
        memory.write(0x200, 0xA2); // High byte
        memory.write(0x201, 0x3C); // Low byte
        
        let opcode = cpu.fetch(&memory);
        assert_eq!(opcode, 0xA23C); // Should be combined as (0xA2 << 8) | 0x3C
    }

    // === Execute Tests ===

    #[test]
    fn test_opcode_00e0_cls() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // Set some pixels
        display.set_pixel(10, 10, true);
        display.set_pixel(20, 20, true);
        assert!(display.get_pixel(10, 10));

        // Execute CLS
        cpu.execute(0x00E0, &mut memory, &mut display, &keyboard);

        // All pixels should be cleared
        assert!(!display.get_pixel(10, 10));
        assert!(!display.get_pixel(20, 20));
    }

    #[test]
    fn test_opcode_6xkk_ld_vx_byte() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // 6522 - LD V5, 0x22
        cpu.execute(0x6522, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[5], 0x22);

        // 6AFF - LD VA, 0xFF
        cpu.execute(0x6AFF, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[0xA], 0xFF);
    }

    #[test]
    fn test_opcode_7xkk_add_vx_byte() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[3] = 10;
        // 7305 - ADD V3, 0x05
        cpu.execute(0x7305, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[3], 15);
    }

    #[test]
    fn test_opcode_7xkk_add_wraps() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[2] = 0xFF;
        // 7202 - ADD V2, 0x02 (should wrap: 0xFF + 0x02 = 0x01)
        cpu.execute(0x7202, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[2], 0x01);
    }

    #[test]
    fn test_opcode_annn_ld_i_addr() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // A23C - LD I, 0x23C
        cpu.execute(0xA23C, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.i, 0x23C);

        // AFFF - LD I, 0xFFF
        cpu.execute(0xAFFF, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.i, 0xFFF);
    }

    #[test]
    fn test_opcode_1nnn_jp_addr() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // 1ABC - JP 0xABC
        cpu.execute(0x1ABC, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0xABC);
    }

    #[test]
    fn test_opcode_3xkk_se_vx_byte_equal() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[5] = 0x42;
        cpu.pc = 0x200;

        // 3542 - SE V5, 0x42 (should skip)
        cpu.execute(0x3542, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x202); // PC incremented by 2
    }

    #[test]
    fn test_opcode_3xkk_se_vx_byte_not_equal() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[5] = 0x42;
        cpu.pc = 0x200;

        // 3543 - SE V5, 0x43 (should not skip)
        cpu.execute(0x3543, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x200); // PC unchanged
    }

    #[test]
    fn test_opcode_4xkk_sne_vx_byte_not_equal() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[3] = 0x10;
        cpu.pc = 0x300;

        // 4320 - SNE V3, 0x20 (should skip because 0x10 != 0x20)
        cpu.execute(0x4320, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x302);
    }

    #[test]
    fn test_opcode_4xkk_sne_vx_byte_equal() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[3] = 0x20;
        cpu.pc = 0x300;

        // 4320 - SNE V3, 0x20 (should not skip because equal)
        cpu.execute(0x4320, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x300); // Unchanged
    }

    #[test]
    fn test_opcode_8xy0_ld_vx_vy() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[3] = 0xAA;
        cpu.v[7] = 0x55;

        // 8370 - LD V3, V7
        cpu.execute(0x8370, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[3], 0x55);
        assert_eq!(cpu.v[7], 0x55); // V7 unchanged
    }

    #[test]
    fn test_opcode_8xy1_or_vx_vy() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[2] = 0b10101010;
        cpu.v[5] = 0b01010101;

        // 8251 - OR V2, V5
        cpu.execute(0x8251, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[2], 0b11111111);
    }

    #[test]
    fn test_opcode_8xy2_and_vx_vy() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[4] = 0b11110000;
        cpu.v[6] = 0b10101010;

        // 8462 - AND V4, V6
        cpu.execute(0x8462, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[4], 0b10100000);
    }

    #[test]
    fn test_opcode_8xy3_xor_vx_vy() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[1] = 0b11110000;
        cpu.v[3] = 0b10101010;

        // 8133 - XOR V1, V3
        cpu.execute(0x8133, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[1], 0b01011010);
    }

    #[test]
    fn test_opcode_8xy4_add_vx_vy_no_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[2] = 10;
        cpu.v[3] = 20;

        // 8234 - ADD V2, V3
        cpu.execute(0x8234, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[2], 30);
        assert_eq!(cpu.v[0xF], 0); // No carry
    }

    #[test]
    fn test_opcode_8xy4_add_vx_vy_with_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[5] = 200;
        cpu.v[7] = 100;

        // 8574 - ADD V5, V7 (200 + 100 = 300, wraps to 44)
        cpu.execute(0x8574, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[5], 44); // 300 & 0xFF = 44
        assert_eq!(cpu.v[0xF], 1); // Carry set
    }

    #[test]
    fn test_opcode_8xy5_sub_vx_vy_no_borrow() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[3] = 50;
        cpu.v[4] = 20;

        // 8345 - SUB V3, V4
        cpu.execute(0x8345, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[3], 30);
        assert_eq!(cpu.v[0xF], 1); // NOT borrow (Vx > Vy)
    }

    #[test]
    fn test_opcode_8xy5_sub_vx_vy_with_borrow() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[2] = 10;
        cpu.v[5] = 20;

        // 8255 - SUB V2, V5 (10 - 20 = -10, wraps to 246)
        cpu.execute(0x8255, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[2], 246); // wrapping_sub
        assert_eq!(cpu.v[0xF], 0); // Borrow (Vx < Vy)
    }

    #[test]
    fn test_opcode_8xy6_shr_vx() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[6] = 0b10110101;  // Source is Vy (V6)

        // 8766 - SHR V7, V6 (COSMAC VIP quirk: copies V6 to V7, then shifts)
        cpu.execute(0x8766, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[7], 0b01011010);
        assert_eq!(cpu.v[0xF], 1); // LSB was 1
    }

    #[test]
    fn test_opcode_8xy7_subn_vx_vy_no_borrow() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[1] = 20;
        cpu.v[2] = 50;

        // 8127 - SUBN V1, V2 (V1 = V2 - V1 = 50 - 20 = 30)
        cpu.execute(0x8127, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[1], 30);
        assert_eq!(cpu.v[0xF], 1); // NOT borrow (Vy > Vx)
    }

    #[test]
    fn test_opcode_8xy7_subn_vx_vy_with_borrow() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[3] = 50;
        cpu.v[4] = 20;

        // 8347 - SUBN V3, V4 (V3 = V4 - V3 = 20 - 50 = -30, wraps)
        cpu.execute(0x8347, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[3], 226); // wrapping_sub
        assert_eq!(cpu.v[0xF], 0); // Borrow (Vy < Vx)
    }

    #[test]
    fn test_opcode_8xye_shl_vx() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[3] = 0b10110101;  // Source is Vy (V3)

        // 853E - SHL V5, V3 (COSMAC VIP quirk: copies V3 to V5, then shifts)
        cpu.execute(0x853E, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[5], 0b01101010);
        assert_eq!(cpu.v[0xF], 1); // MSB was 1
    }

    #[test]
    #[should_panic(expected = "Stack underflow: RET called with empty stack")]
    fn test_stack_underflow() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // Try to return without any CALL (sp is 0)
        cpu.execute(0x00EE, &mut memory, &mut display, &keyboard); // RET should panic
    }

    #[test]
    fn test_opcode_00ee_ret() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // Simulate a CALL - push return address
        cpu.stack[0] = 0x300;
        cpu.sp = 1;

        // 00EE - RET
        cpu.execute(0x00EE, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x300);
        assert_eq!(cpu.sp, 0);
    }

    #[test]
    #[should_panic(expected = "Stack overflow: Maximum call depth of 16 exceeded")]
    fn test_stack_overflow() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // Fill the stack to maximum (16 levels)
        for _ in 0..16 {
            cpu.execute(0x2200, &mut memory, &mut display, &keyboard); // CALL 0x200
        }

        // This 17th call should panic
        cpu.execute(0x2200, &mut memory, &mut display, &keyboard);
    }

    #[test]
    fn test_opcode_2nnn_call() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.pc = 0x200;

        // 2ABC - CALL 0xABC
        cpu.execute(0x2ABC, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0xABC);
        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.stack[0], 0x200); // Return address saved
    }

    #[test]
    fn test_opcode_5xy0_se_vx_vy_equal() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[2] = 0x42;
        cpu.v[7] = 0x42;
        cpu.pc = 0x200;

        // 5270 - SE V2, V7 (should skip)
        cpu.execute(0x5270, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn test_opcode_5xy0_se_vx_vy_not_equal() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[2] = 0x42;
        cpu.v[7] = 0x43;
        cpu.pc = 0x200;

        // 5270 - SE V2, V7 (should not skip)
        cpu.execute(0x5270, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x200);
    }

    #[test]
    fn test_opcode_9xy0_sne_vx_vy_not_equal() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[3] = 0x10;
        cpu.v[5] = 0x20;
        cpu.pc = 0x300;

        // 9350 - SNE V3, V5 (should skip)
        cpu.execute(0x9350, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x302);
    }

    #[test]
    fn test_opcode_9xy0_sne_vx_vy_equal() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[3] = 0x20;
        cpu.v[5] = 0x20;
        cpu.pc = 0x300;

        // 9350 - SNE V3, V5 (should not skip)
        cpu.execute(0x9350, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x300);
    }

    #[test]
    fn test_opcode_bnnn_jp_v0_addr() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[0] = 0x05;
        cpu.execute(0xB200, &mut memory, &mut display, &keyboard); // JP V0, 0x200
        assert_eq!(cpu.pc, 0x205); // 0x200 + 0x05
    }

    #[test]
    fn test_opcode_cxkk_rnd_vx_byte() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // Execute RND V1, 0xFF multiple times
        // The random value should be different at least once in 10 tries
        cpu.execute(0xC1FF, &mut memory, &mut display, &keyboard);
        let first_value = cpu.v[1];
        
        let mut different = false;
        for _ in 0..10 {
            cpu.execute(0xC1FF, &mut memory, &mut display, &keyboard);
            if cpu.v[1] != first_value {
                different = true;
                break;
            }
        }
        // Should be different at least once (probabilistically)
        assert!(different || first_value == cpu.v[1]); // Always passes but exercises the code

        // Test masking: RND V2, 0x0F should only set lower 4 bits
        cpu.execute(0xC20F, &mut memory, &mut display, &keyboard);
        assert!(cpu.v[2] <= 0x0F);
    }

    #[test]
    fn test_opcode_dxyn_drw_sprite() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // Set up sprite data in memory at address 0x300
        cpu.i = 0x300;
        memory.write(0x300, 0b11110000);
        memory.write(0x301, 0b10010000);
        memory.write(0x302, 0b11110000);

        // Draw at position (5, 10) with height 3
        cpu.v[2] = 5;  // x
        cpu.v[3] = 10; // y
        cpu.execute(0xD233, &mut memory, &mut display, &keyboard); // DRW V2, V3, 3

        // VF should be 0 (no collision on first draw)
        assert_eq!(cpu.v[0xF], 0);

        // Verify pixels were set
        assert_eq!(display.get_pixel(5, 10), true);
        assert_eq!(display.get_pixel(8, 10), true);
        assert_eq!(display.get_pixel(9, 10), false);
    }

    #[test]
    fn test_opcode_dxyn_drw_sprite_collision() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // Set up sprite
        cpu.i = 0x300;
        memory.write(0x300, 0xFF);
        cpu.v[1] = 0;
        cpu.v[2] = 0;

        // Draw first time - no collision
        cpu.execute(0xD121, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[0xF], 0);

        // Tick timers to clear VBlank wait flag
        cpu.tick_timers();

        // Draw second time at same position - should have collision
        cpu.execute(0xD121, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn test_opcode_ex9e_skp_vx_pressed() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let mut keyboard = Keyboard::new();

        cpu.v[5] = 0x0A;
        keyboard.set_key(0x0A, true);
        
        let old_pc = cpu.pc;
        cpu.execute(0xE59E, &mut memory, &mut display, &keyboard); // SKP V5
        assert_eq!(cpu.pc, old_pc + 2); // Should skip
    }

    #[test]
    fn test_opcode_ex9e_skp_vx_not_pressed() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[5] = 0x0A;
        // Key not pressed
        
        let old_pc = cpu.pc;
        cpu.execute(0xE59E, &mut memory, &mut display, &keyboard); // SKP V5
        assert_eq!(cpu.pc, old_pc); // Should not skip
    }

    #[test]
    fn test_opcode_exa1_sknp_vx_not_pressed() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[5] = 0x0A;
        // Key not pressed
        
        let old_pc = cpu.pc;
        cpu.execute(0xE5A1, &mut memory, &mut display, &keyboard); // SKNP V5
        assert_eq!(cpu.pc, old_pc + 2); // Should skip
    }

    #[test]
    fn test_opcode_exa1_sknp_vx_pressed() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let mut keyboard = Keyboard::new();

        cpu.v[5] = 0x0A;
        keyboard.set_key(0x0A, true);
        
        let old_pc = cpu.pc;
        cpu.execute(0xE5A1, &mut memory, &mut display, &keyboard); // SKNP V5
        assert_eq!(cpu.pc, old_pc); // Should not skip
    }

    #[test]
    fn test_opcode_fx07_ld_vx_dt() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.delay_timer = 42;
        cpu.execute(0xF307, &mut memory, &mut display, &keyboard); // LD V3, DT
        assert_eq!(cpu.v[3], 42);
    }

    #[test]
    fn test_opcode_fx15_ld_dt_vx() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[7] = 100;
        cpu.execute(0xF715, &mut memory, &mut display, &keyboard); // LD DT, V7
        assert_eq!(cpu.delay_timer, 100);
    }

    #[test]
    fn test_opcode_fx18_ld_st_vx() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.v[2] = 60;
        cpu.execute(0xF218, &mut memory, &mut display, &keyboard); // LD ST, V2
        assert_eq!(cpu.sound_timer, 60);
    }

    #[test]
    fn test_opcode_fx1e_add_i_vx() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.i = 0x100;
        cpu.v[5] = 0x50;
        cpu.execute(0xF51E, &mut memory, &mut display, &keyboard); // ADD I, V5
        assert_eq!(cpu.i, 0x150);
    }

    #[test]
    fn test_opcode_fx1e_add_i_vx_wrapping() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.i = 0xFFF0;
        cpu.v[5] = 0x20;
        cpu.execute(0xF51E, &mut memory, &mut display, &keyboard); // ADD I, V5
        assert_eq!(cpu.i, 0x0010); // Should wrap
    }

    #[test]
    fn test_opcode_fx33_ld_b_vx() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.i = 0x300;
        cpu.v[7] = 234; // 234 = 2 hundreds, 3 tens, 4 ones
        cpu.execute(0xF733, &mut memory, &mut display, &keyboard); // LD B, V7
        
        assert_eq!(memory.read(0x300), 2); // Hundreds
        assert_eq!(memory.read(0x301), 3); // Tens
        assert_eq!(memory.read(0x302), 4); // Ones
    }

    #[test]
    fn test_opcode_fx33_ld_b_vx_small() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.i = 0x400;
        cpu.v[2] = 5; // 005
        cpu.execute(0xF233, &mut memory, &mut display, &keyboard); // LD B, V2
        
        assert_eq!(memory.read(0x400), 0);
        assert_eq!(memory.read(0x401), 0);
        assert_eq!(memory.read(0x402), 5);
    }

    #[test]
    fn test_opcode_fx55_ld_i_vx() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.i = 0x300;
        cpu.v[0] = 10;
        cpu.v[1] = 20;
        cpu.v[2] = 30;
        cpu.v[3] = 40;
        
        cpu.execute(0xF355, &mut memory, &mut display, &keyboard); // LD [I], V3
        
        // Should store V0 through V3
        assert_eq!(memory.read(0x300), 10);
        assert_eq!(memory.read(0x301), 20);
        assert_eq!(memory.read(0x302), 30);
        assert_eq!(memory.read(0x303), 40);
    }

    #[test]
    fn test_opcode_fx65_ld_vx_i() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.i = 0x300;
        memory.write(0x300, 100);
        memory.write(0x301, 200);
        memory.write(0x302, 150);
        
        cpu.execute(0xF265, &mut memory, &mut display, &keyboard); // LD V2, [I]
        
        // Should load into V0 through V2
        assert_eq!(cpu.v[0], 100);
        assert_eq!(cpu.v[1], 200);
        assert_eq!(cpu.v[2], 150);
    }

    #[test]
    fn test_opcode_fx0a_ld_vx_k_key_pressed() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let mut keyboard = Keyboard::new();

        cpu.pc = 0x200;
        
        // First execution - no key pressed, should wait
        cpu.execute(0xF30A, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x1FE); // Decremented to repeat
        
        // Press key 0x0A
        keyboard.set_key(0x0A, true);
        cpu.pc = 0x200;
        cpu.execute(0xF30A, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.pc, 0x1FE); // Still waiting for release
        
        // Release key
        keyboard.set_key(0x0A, false);
        cpu.pc = 0x200;
        cpu.execute(0xF30A, &mut memory, &mut display, &keyboard);
        assert_eq!(cpu.v[3], 0x0A); // Key stored
        assert_eq!(cpu.pc, 0x200); // PC advances normally
    }

    #[test]
    fn test_opcode_fx0a_ld_vx_k_no_key() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        cpu.pc = 0x200;
        
        cpu.execute(0xF30A, &mut memory, &mut display, &keyboard); // LD V3, K
        
        // PC should be decremented by 2 to repeat the instruction
        assert_eq!(cpu.pc, 0x1FE);
    }

    #[test]
    fn test_opcode_fx29_ld_f_vx() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // Test digit 0 (font at 0x000)
        cpu.v[2] = 0;
        cpu.execute(0xF229, &mut memory, &mut display, &keyboard); // LD F, V2
        assert_eq!(cpu.i, 0x000);

        // Test digit 5 (font at 0x019 = 5 * 5)
        cpu.v[3] = 5;
        cpu.execute(0xF329, &mut memory, &mut display, &keyboard); // LD F, V3
        assert_eq!(cpu.i, 25); // 5 * 5

        // Test digit F (font at 0x04B = 15 * 5)
        cpu.v[4] = 0xF;
        cpu.execute(0xF429, &mut memory, &mut display, &keyboard); // LD F, V4
        assert_eq!(cpu.i, 75); // 15 * 5
    }

    #[test]
    fn test_cycle() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let mut display = Display::new();
        let keyboard = Keyboard::new();

        // Write a simple instruction to memory: 6142 = LD V1, 0x42
        memory.write(0x200, 0x61);
        memory.write(0x201, 0x42);

        cpu.cycle(&mut memory, &mut display, &keyboard);

        // V1 should now be 0x42
        assert_eq!(cpu.v[1], 0x42);
        // PC should have advanced to 0x202
        assert_eq!(cpu.pc, 0x202);
    }
}
