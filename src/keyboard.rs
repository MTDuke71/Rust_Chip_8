//! Keyboard module for CHIP-8
//!
//! CHIP-8 uses a 16-key hexadecimal keypad (0-F).

/// The 16-key CHIP-8 keyboard
pub struct Keyboard {
    /// State of each key: true = pressed, false = released
    keys: [bool; 16],
}

impl Keyboard {
    /// Creates a new keyboard with all keys released
    pub fn new() -> Self {
        todo!("Implement Keyboard::new()")
    }

    /// Returns true if the given key (0-F) is pressed
    pub fn is_key_pressed(&self, key: u8) -> bool {
        todo!("Implement Keyboard::is_key_pressed()")
    }

    /// Sets the state of a key (for input handling)
    pub fn set_key(&mut self, key: u8, pressed: bool) {
        todo!("Implement Keyboard::set_key()")
    }

    /// Returns the first pressed key, or None if no key is pressed
    pub fn get_pressed_key(&self) -> Option<u8> {
        todo!("Implement Keyboard::get_pressed_key()")
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_new_no_keys_pressed() {
        let keyboard = Keyboard::new();
        for key in 0..16 {
            assert_eq!(keyboard.is_key_pressed(key), false, "Key {:X} should not be pressed", key);
        }
    }

    #[test]
    fn test_keyboard_set_and_check_key() {
        let mut keyboard = Keyboard::new();
        keyboard.set_key(0x5, true);
        assert_eq!(keyboard.is_key_pressed(0x5), true);
        assert_eq!(keyboard.is_key_pressed(0x6), false);
    }

    #[test]
    fn test_keyboard_release_key() {
        let mut keyboard = Keyboard::new();
        keyboard.set_key(0xA, true);
        keyboard.set_key(0xA, false);
        assert_eq!(keyboard.is_key_pressed(0xA), false);
    }

    #[test]
    fn test_keyboard_get_pressed_key() {
        let mut keyboard = Keyboard::new();
        assert_eq!(keyboard.get_pressed_key(), None);
        
        keyboard.set_key(0x7, true);
        assert_eq!(keyboard.get_pressed_key(), Some(0x7));
    }
}
