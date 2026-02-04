//! CHIP-8 Emulator Library
//!
//! This crate provides the core components for a CHIP-8 emulator:
//! - CPU (fetch, decode, execute)
//! - Memory (4KB RAM)
//! - Display (64x32 pixels)
//! - Keyboard (16 keys)
//! - Sound (beep tone)

pub mod cpu;
pub mod display;
pub mod keyboard;
pub mod memory;
pub mod sound;
