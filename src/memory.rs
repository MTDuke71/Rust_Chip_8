//! Memory module for CHIP-8
//!
//! CHIP-8 has 4KB (4096 bytes) of RAM.
//! - 0x000-0x1FF: Reserved for interpreter (font data stored here)
//! - 0x200-0xFFF: Program and data space

/// The 4KB memory of the CHIP-8 system

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

pub struct Memory {
    ram: [u8; 4096],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Creates a new Memory instance with font data loaded
    pub fn new() -> Self {
        let mut mem = Memory { ram: [0; 4096] };
        // Load font set into memory starting at 0x000
        for (i, &byte) in FONT_SET.iter().enumerate() {
            mem.ram[i] = byte;
        }
        mem
    }

    /// Reads a byte from the given address
    pub fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    /// Writes a byte to the given address
    pub fn write(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    /// Loads a ROM into memory starting at 0x200
    pub fn load_rom(&mut self, data: &[u8]) {
        let start = 0x200;
        for (i, &byte) in data.iter().enumerate() {
            self.ram[start + i] = byte;
        }
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

    #[test]
fn test_font_data_loaded() {
    let mem = Memory::new();
    // Check first font character "0" starts at 0x000
    assert_eq!(mem.read(0x000), 0xF0);
    assert_eq!(mem.read(0x004), 0xF0);
    // Check font character "1" starts at 0x005
    assert_eq!(mem.read(0x005), 0x20);
}

    #[test]
    fn test_load_rom() {
        let mut mem = Memory::new();
        let rom_data = [0xDE, 0xAD, 0xBE, 0xEF];
        mem.load_rom(&rom_data);
        assert_eq!(mem.read(0x200), 0xDE);
        assert_eq!(mem.read(0x201), 0xAD);
        assert_eq!(mem.read(0x202), 0xBE);
        assert_eq!(mem.read(0x203), 0xEF);
    }

    #[test]
    fn test_memory_default() {
        // Test that Default::default() works the same as new()
        let mem = Memory::default();
        // Should have font data loaded
        assert_eq!(mem.read(0x000), 0xF0);
        // Program area should be zeroed
        assert_eq!(mem.read(0x200), 0);
    }
}
