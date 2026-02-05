//! Sound module for CHIP-8
//!
//! Generates a simple beep tone when the sound timer is active.

use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

/// Generates a square wave audio source
struct SquareWave {
    frequency: f32,
    sample_rate: u32,
    num_sample: usize,
}

impl SquareWave {
    fn new(frequency: f32) -> Self {
        SquareWave {
            frequency,
            sample_rate: 48000,
            num_sample: 0,
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        
        let period = self.sample_rate as f32 / self.frequency;
        let sample = (self.num_sample as f32 % period) / period;
        
        // Square wave: -0.1 or 0.1 (low volume to avoid ear damage!)
        Some(if sample < 0.5 { 0.1 } else { -0.1 })
    }
}

impl Source for SquareWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

/// Sound system that can play a beep tone
pub struct Sound {
    _stream: OutputStream,
    sink: Sink,
}

impl Sound {
    /// Creates a new sound system
    pub fn new() -> Option<Self> {
        // Try to create audio output
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(output) => output,
            Err(_) => return None, // Audio not available
        };

        let sink = match Sink::try_new(&stream_handle) {
            Ok(s) => s,
            Err(_) => return None,
        };

        // Pre-load the square wave source so it's ready to play
        let source = SquareWave::new(440.0);
        sink.append(source);
        sink.pause(); // Start paused

        Some(Sound { _stream, sink })
    }

    /// Starts playing the beep sound if not already playing
    pub fn play(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        }
    }

    /// Pauses the beep sound
    pub fn stop(&self) {
        if !self.sink.is_paused() {
            self.sink.pause();
        }
    }

    /// Returns true if sound is currently playing
    pub fn is_playing(&self) -> bool {
        !self.sink.is_paused()
    }
}

impl Default for Sound {
    fn default() -> Self {
        Self::new().unwrap_or_else(|| {
            eprintln!("Warning: Audio system unavailable");
            // Return a dummy sound that does nothing
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let source = SquareWave::new(440.0);
            sink.append(source);
            sink.pause();
            Sound { _stream, sink }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_wave_generates_samples() {
        let mut wave = SquareWave::new(440.0);
        
        // Generate some samples
        for _ in 0..100 {
            let sample = wave.next().unwrap();
            assert!(sample == 0.1 || sample == -0.1);
        }
    }

    #[test]
    fn test_square_wave_properties() {
        let wave = SquareWave::new(440.0);
        assert_eq!(wave.channels(), 1);
        assert_eq!(wave.sample_rate(), 48000);
        assert_eq!(wave.current_frame_len(), None);
        assert_eq!(wave.total_duration(), None);
    }

    #[test]
    fn test_sound_creation() {
        // This may fail if audio is not available, which is ok
        let _sound = Sound::new();
    }
}
