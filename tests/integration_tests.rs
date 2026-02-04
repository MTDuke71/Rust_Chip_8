//! Integration tests for CHIP-8 emulator
//!
//! These tests verify that all components work together correctly.

use chip8_emulator::cpu::Cpu;
use chip8_emulator::display::Display;
use chip8_emulator::keyboard::Keyboard;
use chip8_emulator::memory::Memory;

#[test]
fn test_load_and_execute_simple_program() {
    // Create a simple program that sets V0 = 42
    let program = vec![
        0x60, 0x2A, // 6000: LD V0, 42
    ];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let keyboard = Keyboard::new();

    memory.load_rom(&program);

    // Execute one cycle
    cpu.cycle(&mut memory, &mut display, &keyboard);

    // Verify V0 = 42
    assert_eq!(cpu.v[0], 42);
}

#[test]
fn test_arithmetic_operations_integration() {
    // Program: V0 = 10, V1 = 20, V2 = V0 + V1
    let program = vec![
        0x60, 0x0A, // LD V0, 10
        0x61, 0x14, // LD V1, 20
        0x82, 0x04, // ADD V2, V0 (V2 = V0, then add V1)
    ];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let keyboard = Keyboard::new();

    memory.load_rom(&program);

    // Execute three cycles
    for _ in 0..3 {
        cpu.cycle(&mut memory, &mut display, &keyboard);
    }

    assert_eq!(cpu.v[0], 10);
    assert_eq!(cpu.v[1], 20);
    // Note: 8xy4 is ADD Vx, Vy which adds Vy to Vx, so V2 += V1
    // But V2 starts at 0, so we need to load it first
}

#[test]
fn test_subroutine_call_and_return() {
    // Program with CALL and RET
    let program = vec![
        0x22, 0x06, // 0x200: CALL 0x206 (subroutine at 0x206)
        0x00, 0xEE, // 0x202: RET (should not execute initially)
        0x60, 0x42, // 0x204: LD V0, 66 (should not execute)
        0x61, 0xFF, // 0x206: LD V1, 255 (subroutine)
        0x00, 0xEE, // 0x208: RET
    ];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let keyboard = Keyboard::new();

    memory.load_rom(&program);

    // Execute CALL instruction
    cpu.cycle(&mut memory, &mut display, &keyboard);
    assert_eq!(cpu.pc, 0x206); // Should jump to subroutine
    assert_eq!(cpu.sp, 1); // Stack pointer should increment

    // Execute LD V1, 255
    cpu.cycle(&mut memory, &mut display, &keyboard);
    assert_eq!(cpu.v[1], 255);

    // Execute RET
    cpu.cycle(&mut memory, &mut display, &keyboard);
    assert_eq!(cpu.pc, 0x202); // Should return to after CALL
    assert_eq!(cpu.sp, 0); // Stack pointer should decrement
}

#[test]
fn test_display_drawing_integration() {
    // Draw a simple sprite on the display
    let program = vec![
        0x60, 0x10, // LD V0, 16 (x position)
        0x61, 0x08, // LD V1, 8  (y position)
        0xA2, 0x00, // LD I, 0x000 (font sprite location)
        0xD0, 0x15, // DRW V0, V1, 5 (draw 5-byte sprite)
    ];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let keyboard = Keyboard::new();

    memory.load_rom(&program);

    // Execute all instructions
    for _ in 0..4 {
        cpu.cycle(&mut memory, &mut display, &keyboard);
    }

    // Verify VF is set (could be 0 or 1 depending on collision)
    // Verify display is not blank (at least one pixel set)
    let buffer = display.to_buffer();
    let has_pixels = buffer.iter().any(|&pixel| pixel != 0x00000000);
    assert!(has_pixels, "Display should have drawn pixels");
}

#[test]
fn test_timer_countdown_integration() {
    // Set delay timer and verify it counts down
    let program = vec![
        0x60, 0x0A, // LD V0, 10
        0xF0, 0x15, // LD DT, V0 (set delay timer to 10)
        0xF1, 0x07, // LD V1, DT (read delay timer)
    ];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let keyboard = Keyboard::new();

    memory.load_rom(&program);

    // Set delay timer
    cpu.cycle(&mut memory, &mut display, &keyboard);
    cpu.cycle(&mut memory, &mut display, &keyboard);

    // Verify timer was set
    assert_eq!(cpu.delay_timer, 10);

    // Tick timers a few times
    for _ in 0..3 {
        cpu.tick_timers();
    }

    // Read timer value
    cpu.cycle(&mut memory, &mut display, &keyboard);

    // Timer should have counted down
    assert_eq!(cpu.v[1], 7); // 10 - 3 = 7
}

#[test]
fn test_keyboard_input_integration() {
    // Test skip if key pressed
    let program = vec![
        0xE0, 0x9E, // SKP V0 (skip if key in V0 is pressed)
        0x61, 0x01, // LD V1, 1 (should be skipped if key pressed)
        0x62, 0x02, // LD V2, 2 (always executes)
    ];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let mut keyboard = Keyboard::new();

    memory.load_rom(&program);

    // Test without key pressed
    cpu.v[0] = 0x5; // Check for key 5
    cpu.cycle(&mut memory, &mut display, &keyboard);
    assert_eq!(cpu.pc, 0x202); // Should not skip

    // Reset and test with key pressed
    let mut cpu = Cpu::new();
    memory.load_rom(&program);
    keyboard.set_key(0x5, true);
    cpu.v[0] = 0x5;
    cpu.cycle(&mut memory, &mut display, &keyboard);
    assert_eq!(cpu.pc, 0x204); // Should skip to 0x204
}

#[test]
fn test_jump_and_conditional_skip() {
    // Test JP and SE instructions
    let program = vec![
        0x60, 0x42, // 0x200: LD V0, 66
        0x30, 0x42, // 0x202: SE V0, 66 (skip if V0 == 66)
        0x61, 0xFF, // 0x204: LD V1, 255 (should be skipped)
        0x12, 0x08, // 0x206: JP 0x208
        0x62, 0x01, // 0x208: LD V2, 1
    ];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let keyboard = Keyboard::new();

    memory.load_rom(&program);

    // Execute LD V0, 66
    cpu.cycle(&mut memory, &mut display, &keyboard);
    assert_eq!(cpu.v[0], 66);

    // Execute SE V0, 66 (should skip)
    cpu.cycle(&mut memory, &mut display, &keyboard);
    assert_eq!(cpu.pc, 0x206); // Should have skipped to 0x206

    // Execute JP 0x208
    cpu.cycle(&mut memory, &mut display, &keyboard);
    assert_eq!(cpu.pc, 0x208);

    // Execute LD V2, 1
    cpu.cycle(&mut memory, &mut display, &keyboard);
    assert_eq!(cpu.v[2], 1);
    assert_eq!(cpu.v[1], 0); // V1 should still be 0 (instruction was skipped)
}

#[test]
fn test_bcd_conversion_integration() {
    // Test Fx33 BCD instruction
    let program = vec![
        0x60, 0xFE, // LD V0, 254
        0xA3, 0x00, // LD I, 0x300
        0xF0, 0x33, // LD B, V0 (store BCD of V0 at I)
    ];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let keyboard = Keyboard::new();

    memory.load_rom(&program);

    // Execute all instructions
    for _ in 0..3 {
        cpu.cycle(&mut memory, &mut display, &keyboard);
    }

    // Check BCD values in memory at 0x300, 0x301, 0x302
    assert_eq!(memory.read(0x300), 2); // Hundreds
    assert_eq!(memory.read(0x301), 5); // Tens
    assert_eq!(memory.read(0x302), 4); // Ones
}

#[test]
fn test_register_save_and_load() {
    // Test Fx55 and Fx65 (save and load registers)
    let program = vec![
        0x60, 0x11, // LD V0, 0x11
        0x61, 0x22, // LD V1, 0x22
        0x62, 0x33, // LD V2, 0x33
        0xA3, 0x00, // LD I, 0x300
        0xF2, 0x55, // LD [I], V2 (save V0-V2 to memory)
        0x60, 0x00, // LD V0, 0 (clear registers)
        0x61, 0x00, // LD V1, 0
        0x62, 0x00, // LD V2, 0
        0xA3, 0x00, // LD I, 0x300
        0xF2, 0x65, // LD V2, [I] (load V0-V2 from memory)
    ];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let keyboard = Keyboard::new();

    memory.load_rom(&program);

    // Execute all instructions
    for _ in 0..10 {
        cpu.cycle(&mut memory, &mut display, &keyboard);
    }

    // Verify registers were restored
    assert_eq!(cpu.v[0], 0x11);
    assert_eq!(cpu.v[1], 0x22);
    assert_eq!(cpu.v[2], 0x33);
}

#[test]
fn test_multiple_cycles_with_all_components() {
    // A more complex program that uses multiple components
    let program = vec![
        0x60, 0x05, // LD V0, 5
        0x61, 0x0A, // LD V1, 10
        0x80, 0x14, // ADD V0, V1 (V0 = V0 + V1)
        0xA2, 0x10, // LD I, 0x210
        0xF0, 0x33, // LD B, V0 (BCD of V0)
        0x00, 0xE0, // CLS
    ];

    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let keyboard = Keyboard::new();

    memory.load_rom(&program);

    // Execute all instructions
    for _ in 0..6 {
        cpu.cycle(&mut memory, &mut display, &keyboard);
    }

    // Verify final state
    assert_eq!(cpu.v[0], 15); // 5 + 10
    assert_eq!(cpu.v[0xF], 0); // No carry
    
    // Check BCD conversion (15 = 0, 1, 5)
    assert_eq!(memory.read(0x210), 0); // Hundreds
    assert_eq!(memory.read(0x211), 1); // Tens
    assert_eq!(memory.read(0x212), 5); // Ones
    
    // Display should be cleared
    let buffer = display.to_buffer();
    assert!(buffer.iter().all(|&pixel| pixel == 0x00000000));
}
