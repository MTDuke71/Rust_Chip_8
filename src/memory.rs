//! Memory module for CHIP-8
//!
//! CHIP-8 has 4KB (4096 bytes) of RAM.
//! - 0x000-0x1FF: Reserved for interpreter (font data stored here)
//! - 0x200-0xFFF: Program and data space

/// The 4KB memory of the CHIP-8 system
pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    /// Creates a new Memory instance with font data loaded
    pub fn new() -> Self {
        todo!("Implement Memory::new()")
    }

    /// Reads a byte from the given address
    pub fn read(&self, addr: u16) -> u8 {
        todo!("Implement Memory::read()")
    }

    /// Writes a byte to the given address
    pub fn write(&mut self, addr: u16, value: u8) {
        todo!("Implement Memory::write()")
    }

    /// Loads a ROM into memory starting at 0x200
    pub fn load_rom(&mut self, data: &[u8]) {
        todo!("Implement Memory::load_rom()")
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests will FAIL until you implement the functions!
    // That's the point of TDD - write tests first, then make them pass.

    #[test]
    fn test_memory_new_is_zeroed() {
        let mem = Memory::new();
        // Memory should be zeroed (except for font data)
        assert_eq!(mem.read(0x200), 0);
        assert_eq!(mem.read(0xFFF), 0);
    }

    #[test]
    fn test_memory_write_read() {
        let mut mem = Memory::new();
        mem.write(0x200, 0x42);
        assert_eq!(mem.read(0x200), 0x42);
    }

    #[test]
    fn test_memory_write_read_multiple() {
        let mut mem = Memory::new();
        mem.write(0x200, 0xAB);
        mem.write(0x201, 0xCD);
        assert_eq!(mem.read(0x200), 0xAB);
        assert_eq!(mem.read(0x201), 0xCD);
    }
}
