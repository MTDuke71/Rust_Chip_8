//! CHIP-8 Emulator
//!
//! A CHIP-8 emulator written in Rust.

use chip8_emulator::cpu::Cpu;
use chip8_emulator::display::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use chip8_emulator::keyboard::Keyboard;
use chip8_emulator::memory::Memory;
use chip8_emulator::sound::Sound;
use minifb::{Key, Window, WindowOptions};
use std::env;
use std::fs;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 320;
const CYCLES_PER_FRAME: u32 = 200;  // Instructions per frame - high value for quirks test (DRW breaks early anyway)

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("CHIP-8 Emulator");
        println!("===============");
        println!();
        println!("Usage: {} <rom_file>", args[0]);
        println!();
        println!("Example: {} roms/pong.ch8", args[0]);
        println!();
        println!("Controls:");
        println!("  P           - Pause/Resume");
        println!("  R           - Reset");
        println!("  ]/=         - CPU speed up");
        println!("  [/-         - CPU speed down");
        println!("  Page Up     - Timer speed up");
        println!("  Page Down   - Timer speed down");
        println!("  ESC         - Quit");
        return;
    }

    let rom_path = &args[1];
    
    // Load ROM
    let rom_data = match fs::read(rom_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading ROM '{}': {}", rom_path, e);
            return;
        }
    };

    println!("Loaded ROM: {} ({} bytes)", rom_path, rom_data.len());
    println!();
    println!("Controls:");
    println!("  P           - Pause/Resume");
    println!("  R           - Reset");
    println!("  ]/=         - CPU speed up (current: 1.0x)");
    println!("  [/-         - CPU speed down");
    println!("  Page Up     - Timer speed up (current: 1.0x)");
    println!("  Page Down   - Timer speed down");
    println!("  ESC         - Quit");

    // Initialize components
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let mut keyboard = Keyboard::new();
    let sound = Sound::new().unwrap_or_else(|| {
        eprintln!("Warning: Could not initialize audio system");
        Sound::default()
    });

    // Load ROM into memory
    memory.load_rom(&rom_data);

    // Debug timing variables (commented out)
    // let mut debug_timer = Instant::now();
    // let mut total_cycles: u64 = 0;
    // let mut total_frames: u64 = 0;
    // let mut drw_breaks: u64 = 0;
    // let mut timer_decrements: u64 = 0;

    // Emulator state
    let mut is_paused = false;
    let mut speed_multiplier = 1.0f32; // 1.0 = normal speed, range: 0.25x to 4.0x
    let timer_multiplier = 1.0f32; // 1.0 = normal 60Hz (fixed for proper DISP.WAIT timing)
    let mut last_p_key = false;
    let mut last_r_key = false;
    let mut last_plus_key = false;
    let mut last_minus_key = false;
    let last_pgup_key = false;
    let last_pgdn_key = false;

    // Create window
    let mut window = Window::new(
        "CHIP-8 Emulator - CPU:1.0x Timer:1.0x",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Unable to create window: {}", e);
    });

    // Don't use minifb's rate limiting - we'll do our own precise timing
    window.set_target_fps(0);

    let mut cycles_per_frame = CYCLES_PER_FRAME;
    
    // Precise frame timing for DISP.WAIT
    let frame_duration = Duration::from_nanos(1_000_000_000 / 60); // Exactly 60 Hz
    let mut last_frame_time = Instant::now();

    // Main emulation loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Handle control keys (pause, reset, speed)
        let p_pressed = window.is_key_down(Key::P);
        let r_pressed = window.is_key_down(Key::R);
        // CPU speed control
        let plus_pressed = window.is_key_down(Key::Equal) 
            || window.is_key_down(Key::NumPadPlus)
            || window.is_key_down(Key::RightBracket);  // ] key
        let minus_pressed = window.is_key_down(Key::Minus) 
            || window.is_key_down(Key::NumPadMinus)
            || window.is_key_down(Key::LeftBracket);   // [ key
        // Timer speed control
        let pgup_pressed = window.is_key_down(Key::PageUp);
        let pgdn_pressed = window.is_key_down(Key::PageDown);

        // Toggle pause (detect rising edge)
        if p_pressed && !last_p_key {
            is_paused = !is_paused;
            let status = if is_paused { "PAUSED" } else { "" };
            let title = format!("CHIP-8 Emulator - CPU:{:.2}x Timer:{:.2}x {}", speed_multiplier, timer_multiplier, status);
            window.set_title(&title);
            println!("{}", if is_paused { "Paused" } else { "Resumed" });
        }
        last_p_key = p_pressed;

        // Reset emulator (detect rising edge)
        if r_pressed && !last_r_key {
            cpu = Cpu::new();
            memory = Memory::new();
            display = Display::new();
            keyboard = Keyboard::new();
            memory.load_rom(&rom_data);
            last_frame_time = Instant::now();
            println!("Reset emulator");
        }
        last_r_key = r_pressed;

        // CPU Speed up (detect rising edge) - increases cycles per frame
        if plus_pressed && !last_plus_key {
            speed_multiplier = (speed_multiplier * 2.0).min(4.0);
            cycles_per_frame = (CYCLES_PER_FRAME as f32 * speed_multiplier) as u32;
            let status = if is_paused { "PAUSED" } else { "" };
            let title = format!("CHIP-8 Emulator - CPU:{:.2}x Timer:{:.2}x {}", speed_multiplier, timer_multiplier, status);
            window.set_title(&title);
            println!("CPU Speed: {:.2}x ({} cycles/frame)", speed_multiplier, cycles_per_frame);
        }
        last_plus_key = plus_pressed;

        // CPU Speed down (detect rising edge)
        if minus_pressed && !last_minus_key {
            speed_multiplier = (speed_multiplier / 2.0).max(0.25);
            cycles_per_frame = (CYCLES_PER_FRAME as f32 * speed_multiplier) as u32;
            let status = if is_paused { "PAUSED" } else { "" };
            let title = format!("CHIP-8 Emulator - CPU:{:.2}x Timer:{:.2}x {}", speed_multiplier, timer_multiplier, status);
            window.set_title(&title);
            println!("CPU Speed: {:.2}x ({} cycles/frame)", speed_multiplier, cycles_per_frame);
        }
        last_minus_key = minus_pressed;

        // Timer Speed controls disabled for now - proper DISP.WAIT requires exactly 60 Hz
        // Keeping the key checks to avoid warnings
        let _ = (pgup_pressed, last_pgup_key, pgdn_pressed, last_pgdn_key, timer_multiplier);

        // Debug: Report stats every second (commented out)
        // if debug_timer.elapsed() >= Duration::from_secs(1) {
        //     let elapsed = debug_timer.elapsed().as_secs_f64();
        //     let cycles_per_sec = total_cycles as f64 / elapsed;
        //     let frames_per_sec = total_frames as f64 / elapsed;
        //     let avg_cycles_per_frame = if total_frames > 0 { total_cycles as f64 / total_frames as f64 } else { 0.0 };
        //     println!("DEBUG: Cycles/sec: {:.0}, Frames/sec: {:.1}, Avg cycles/frame: {:.1}, DRW breaks: {}, Timer decs: {}",
        //              cycles_per_sec, frames_per_sec, avg_cycles_per_frame, drw_breaks, timer_decrements);
        //     total_cycles = 0;
        //     total_frames = 0;
        //     drw_breaks = 0;
        //     timer_decrements = 0;
        //     debug_timer = Instant::now();
        // }

        // Skip execution if paused
        if !is_paused {
            // Handle keyboard input
            update_keyboard(&window, &mut keyboard);

            // FRAME-BASED EXECUTION with precise timing:
            // - Wait for next frame boundary (60 Hz)
            // - Run CPU cycles until DRW or cycles_per_frame reached
            // - Timer decrements ONCE per frame (AFTER CPU cycles)
            
            // Precise frame timing: spin-wait until frame boundary
            let now = Instant::now();
            let elapsed = now.duration_since(last_frame_time);
            if elapsed < frame_duration {
                // Sleep for most of the remaining time (minus 1ms for spin accuracy)
                let remaining = frame_duration - elapsed;
                if remaining > Duration::from_millis(1) {
                    std::thread::sleep(remaining - Duration::from_millis(1));
                }
                // Spin-wait for the exact frame boundary
                while Instant::now().duration_since(last_frame_time) < frame_duration {
                    std::hint::spin_loop();
                }
            }
            last_frame_time += frame_duration; // Use addition to prevent drift
            
            // Run CPU cycles for this frame
            // DISP.WAIT: If DRW executes, break loop early
            for _ in 0..cycles_per_frame {
                let wait_for_vblank = cpu.cycle(&mut memory, &mut display, &keyboard);
                if wait_for_vblank {
                    break;
                }
            }

            // Timer decrements at END of frame (after CPU cycles)
            cpu.tick_timers();

            // Handle sound based on sound_timer
            if cpu.sound_timer > 0 {
                sound.play();
            } else {
                sound.stop();
            }
        } else {
            // When paused, still stop sound
            sound.stop();
        }

        // Update display (runs at window refresh rate, ~60 Hz)
        let buffer = display.to_buffer();
        window
            .update_with_buffer(&buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .unwrap();
    }

    println!("Emulator stopped.");
}

/// Maps keyboard input to CHIP-8 keys
/// 
/// CHIP-8 keypad:     Modern keyboard:
/// 1 2 3 C            1 2 3 4
/// 4 5 6 D            Q W E R
/// 7 8 9 E            A S D F
/// A 0 B F            Z X C V
fn update_keyboard(window: &Window, keyboard: &mut Keyboard) {
    // Release all keys first
    for i in 0..16 {
        keyboard.set_key(i, false);
    }

    // Map keyboard to CHIP-8 keys
    let key_map = [
        (Key::Key1, 0x1),
        (Key::Key2, 0x2),
        (Key::Key3, 0x3),
        (Key::Key4, 0xC),
        (Key::Q, 0x4),
        (Key::W, 0x5),
        (Key::E, 0x6),
        (Key::R, 0xD),
        (Key::A, 0x7),
        (Key::S, 0x8),
        (Key::D, 0x9),
        (Key::F, 0xE),
        (Key::Z, 0xA),
        (Key::X, 0x0),
        (Key::C, 0xB),
        (Key::V, 0xF),
    ];

    for (key, chip8_key) in key_map.iter() {
        if window.is_key_down(*key) {
            keyboard.set_key(*chip8_key, true);
        }
    }
}
