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
const CPU_HZ: u32 = 700;
const TIMER_HZ: u32 = 60;

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

    // Debug timing variables (commented out for production)
    // let mut loop_count: u64 = 0;
    // let mut debug_timer = Instant::now();
    // let mut cpu_cycles: u64 = 0;

    // Emulator state
    let mut is_paused = false;
    let mut speed_multiplier = 1.0f32; // 1.0 = normal speed, range: 0.25x to 4.0x
    let mut timer_multiplier = 1.0f32; // 1.0 = normal 60Hz, range: 0.25x to 4.0x
    let mut last_p_key = false;
    let mut last_r_key = false;
    let mut last_plus_key = false;
    let mut last_minus_key = false;
    let mut last_pgup_key = false;
    let mut last_pgdn_key = false;

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

    // Note: No FPS limiting - let CPU run at full speed (700 Hz)
    // Display updates are naturally limited by monitor refresh rate

    let mut cpu_interval = Duration::from_nanos(1_000_000_000 / CPU_HZ as u64);
    let mut timer_interval = Duration::from_nanos(1_000_000_000 / TIMER_HZ as u64);

    let mut last_cpu_time = Instant::now();
    let mut last_timer_time = Instant::now();

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
            last_cpu_time = Instant::now();
            last_timer_time = Instant::now();
            println!("Reset emulator");
        }
        last_r_key = r_pressed;

        // CPU Speed up (detect rising edge)
        if plus_pressed && !last_plus_key {
            speed_multiplier = (speed_multiplier * 2.0).min(4.0);
            cpu_interval = Duration::from_nanos((1_000_000_000.0 / (CPU_HZ as f32 * speed_multiplier)) as u64);
            let status = if is_paused { "PAUSED" } else { "" };
            let title = format!("CHIP-8 Emulator - CPU:{:.2}x Timer:{:.2}x {}", speed_multiplier, timer_multiplier, status);
            window.set_title(&title);
            println!("CPU Speed: {:.2}x", speed_multiplier);
        }
        last_plus_key = plus_pressed;

        // CPU Speed down (detect rising edge)
        if minus_pressed && !last_minus_key {
            speed_multiplier = (speed_multiplier / 2.0).max(0.25);
            cpu_interval = Duration::from_nanos((1_000_000_000.0 / (CPU_HZ as f32 * speed_multiplier)) as u64);
            let status = if is_paused { "PAUSED" } else { "" };
            let title = format!("CHIP-8 Emulator - CPU:{:.2}x Timer:{:.2}x {}", speed_multiplier, timer_multiplier, status);
            window.set_title(&title);
            println!("CPU Speed: {:.2}x", speed_multiplier);
        }
        last_minus_key = minus_pressed;

        // Timer Speed up (detect rising edge)
        if pgup_pressed && !last_pgup_key {
            timer_multiplier = (timer_multiplier * 2.0).min(4.0);
            timer_interval = Duration::from_nanos((1_000_000_000.0 / (TIMER_HZ as f32 * timer_multiplier)) as u64);
            let status = if is_paused { "PAUSED" } else { "" };
            let title = format!("CHIP-8 Emulator - CPU:{:.2}x Timer:{:.2}x {}", speed_multiplier, timer_multiplier, status);
            window.set_title(&title);
            println!("Timer Speed: {:.2}x", timer_multiplier);
        }
        last_pgup_key = pgup_pressed;

        // Timer Speed down (detect rising edge)
        if pgdn_pressed && !last_pgdn_key {
            timer_multiplier = (timer_multiplier / 2.0).max(0.25);
            timer_interval = Duration::from_nanos((1_000_000_000.0 / (TIMER_HZ as f32 * timer_multiplier)) as u64);
            let status = if is_paused { "PAUSED" } else { "" };
            let title = format!("CHIP-8 Emulator - CPU:{:.2}x Timer:{:.2}x {}", speed_multiplier, timer_multiplier, status);
            window.set_title(&title);
            println!("Timer Speed: {:.2}x", timer_multiplier);
        }
        last_pgdn_key = pgdn_pressed;

        // Debug: Count loops and report every second (commented out for production)
        // loop_count += 1;
        // if debug_timer.elapsed() >= Duration::from_secs(1) {
        //     let elapsed = debug_timer.elapsed().as_secs_f64();
        //     let loops_per_sec = loop_count as f64 / elapsed;
        //     let cpu_cycles_per_sec = cpu_cycles as f64 / elapsed;
        //     println!("DEBUG: Main loops/sec: {:.1}, CPU cycles/sec: {:.1}, Avg loop time: {:.3}ms", 
        //              loops_per_sec, cpu_cycles_per_sec, 1000.0 / loops_per_sec);
        //     loop_count = 0;
        //     cpu_cycles = 0;
        //     debug_timer = Instant::now();
        // }

        // Skip execution if paused
        if !is_paused {
            // Handle keyboard input
            update_keyboard(&window, &mut keyboard);

            // Run CPU cycles - execute multiple instructions per frame if needed
            // This allows the CPU to run at 700 Hz even though display updates at ~60 Hz
            while last_cpu_time.elapsed() >= cpu_interval {
                cpu.cycle(&mut memory, &mut display, &keyboard);
                // cpu_cycles += 1;  // Debug counter (commented out for production)
                last_cpu_time += cpu_interval;
            }

            // Timer tick (60 Hz * timer_multiplier)
            if last_timer_time.elapsed() >= timer_interval {
                cpu.tick_timers();
                last_timer_time = Instant::now();
            }

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
