//! CPU module for CHIP-8
//!
//! The CPU handles the fetch-decode-execute cycle and maintains
//! all registers and the stack.

use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::memory::Memory;

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
}

impl Cpu {
    /// Creates a new CPU with initial state
    /// PC starts at 0x200 where programs are loaded
    pub fn new() -> Self {
        todo!("Implement Cpu::new()")
    }

    /// Executes one fetch-decode-execute cycle
    pub fn cycle(&mut self, memory: &mut Memory, display: &mut Display, keyboard: &Keyboard) {
        todo!("Implement Cpu::cycle()")
    }

    /// Fetches the next 2-byte opcode from memory
    fn fetch(&mut self, memory: &Memory) -> u16 {
        todo!("Implement Cpu::fetch()")
    }

    /// Decodes and executes an opcode
    fn execute(
        &mut self,
        opcode: u16,
        memory: &mut Memory,
        display: &mut Display,
        keyboard: &Keyboard,
    ) {
        todo!("Implement Cpu::execute()")
    }

    /// Decrements timers (call this at 60Hz)
    pub fn tick_timers(&mut self) {
        todo!("Implement Cpu::tick_timers()")
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
}
