//! Display module for CHIP-8
//!
//! CHIP-8 has a 64x32 pixel monochrome display.
//! Sprites are XORed onto the screen.

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

/// The 64x32 monochrome display
pub struct Display {
    /// Pixel buffer: true = white/on, false = black/off
    pixels: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Display {
    /// Creates a new display with all pixels off
    pub fn new() -> Self {
        todo!("Implement Display::new()")
    }

    /// Clears the display (all pixels off)
    pub fn clear(&mut self) {
        todo!("Implement Display::clear()")
    }

    /// Gets the state of a pixel at (x, y)
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        todo!("Implement Display::get_pixel()")
    }

    /// Sets the state of a pixel at (x, y)
    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        todo!("Implement Display::set_pixel()")
    }

    /// Draws a sprite at (x, y) with the given sprite data.
    /// Returns true if any pixel was erased (collision).
    /// Sprites are XORed onto the display.
    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        todo!("Implement Display::draw_sprite()")
    }

    /// Converts the display to a buffer suitable for minifb
    /// Returns a Vec<u32> where each pixel is either white (0xFFFFFF) or black (0x000000)
    pub fn to_buffer(&self) -> Vec<u32> {
        todo!("Implement Display::to_buffer()")
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_new_is_blank() {
        let display = Display::new();
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                assert_eq!(display.get_pixel(x, y), false, "Pixel at ({}, {}) should be off", x, y);
            }
        }
    }

    #[test]
    fn test_display_set_and_get_pixel() {
        let mut display = Display::new();
        display.set_pixel(10, 5, true);
        assert_eq!(display.get_pixel(10, 5), true);
        assert_eq!(display.get_pixel(11, 5), false);
    }

    #[test]
    fn test_display_clear() {
        let mut display = Display::new();
        display.set_pixel(10, 10, true);
        display.set_pixel(20, 20, true);
        display.clear();
        assert_eq!(display.get_pixel(10, 10), false);
        assert_eq!(display.get_pixel(20, 20), false);
    }
}
