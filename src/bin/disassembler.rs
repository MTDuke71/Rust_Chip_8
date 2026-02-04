//! CHIP-8 Disassembler
//!
//! Disassembles CHIP-8 ROM files into human-readable assembly.

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("CHIP-8 Disassembler");
        println!("==================");
        println!();
        println!("Usage: {} <rom_file>", args[0]);
        println!();
        println!("Example: {} roms/pong.ch8", args[0]);
        process::exit(1);
    }

    let rom_path = &args[1];

    // Load ROM
    let rom_data = match fs::read(rom_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading ROM '{}': {}", rom_path, e);
            process::exit(1);
        }
    };

    println!("Disassembly of: {}", rom_path);
    println!("Size: {} bytes ({} instructions)", rom_data.len(), rom_data.len() / 2);
    println!();
    println!("Address  Opcode  Instruction");
    println!("-------  ------  -----------");

    // Disassemble each instruction
    let mut pc = 0x200; // Programs start at 0x200
    let mut i = 0;

    while i < rom_data.len() {
        if i + 1 >= rom_data.len() {
            println!("0x{:04X}   {:02X}     [Incomplete instruction]", pc, rom_data[i]);
            break;
        }

        let opcode = ((rom_data[i] as u16) << 8) | (rom_data[i + 1] as u16);
        let instruction = disassemble(opcode);
        
        println!("0x{:04X}   {:04X}   {}", pc, opcode, instruction);

        i += 2;
        pc += 2;
    }

    println!();
    println!("End of disassembly.");
}

/// Disassembles a single CHIP-8 opcode into assembly text
fn disassemble(opcode: u16) -> String {
    let nnn = opcode & 0x0FFF;
    let n = (opcode & 0x000F) as u8;
    let x = ((opcode & 0x0F00) >> 8) as u8;
    let y = ((opcode & 0x00F0) >> 4) as u8;
    let kk = (opcode & 0x00FF) as u8;

    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => "CLS".to_string(),
            0x00EE => "RET".to_string(),
            _ => format!("SYS 0x{:03X}", nnn),
        },
        0x1000 => format!("JP 0x{:03X}", nnn),
        0x2000 => format!("CALL 0x{:03X}", nnn),
        0x3000 => format!("SE V{:X}, 0x{:02X}", x, kk),
        0x4000 => format!("SNE V{:X}, 0x{:02X}", x, kk),
        0x5000 => format!("SE V{:X}, V{:X}", x, y),
        0x6000 => format!("LD V{:X}, 0x{:02X}", x, kk),
        0x7000 => format!("ADD V{:X}, 0x{:02X}", x, kk),
        0x8000 => match n {
            0x0 => format!("LD V{:X}, V{:X}", x, y),
            0x1 => format!("OR V{:X}, V{:X}", x, y),
            0x2 => format!("AND V{:X}, V{:X}", x, y),
            0x3 => format!("XOR V{:X}, V{:X}", x, y),
            0x4 => format!("ADD V{:X}, V{:X}", x, y),
            0x5 => format!("SUB V{:X}, V{:X}", x, y),
            0x6 => format!("SHR V{:X}", x),
            0x7 => format!("SUBN V{:X}, V{:X}", x, y),
            0xE => format!("SHL V{:X}", x),
            _ => format!("UNKNOWN 0x{:04X}", opcode),
        },
        0x9000 => format!("SNE V{:X}, V{:X}", x, y),
        0xA000 => format!("LD I, 0x{:03X}", nnn),
        0xB000 => format!("JP V0, 0x{:03X}", nnn),
        0xC000 => format!("RND V{:X}, 0x{:02X}", x, kk),
        0xD000 => format!("DRW V{:X}, V{:X}, {}", x, y, n),
        0xE000 => match kk {
            0x9E => format!("SKP V{:X}", x),
            0xA1 => format!("SKNP V{:X}", x),
            _ => format!("UNKNOWN 0x{:04X}", opcode),
        },
        0xF000 => match kk {
            0x07 => format!("LD V{:X}, DT", x),
            0x0A => format!("LD V{:X}, K", x),
            0x15 => format!("LD DT, V{:X}", x),
            0x18 => format!("LD ST, V{:X}", x),
            0x1E => format!("ADD I, V{:X}", x),
            0x29 => format!("LD F, V{:X}", x),
            0x33 => format!("LD B, V{:X}", x),
            0x55 => format!("LD [I], V{:X}", x),
            0x65 => format!("LD V{:X}, [I]", x),
            _ => format!("UNKNOWN 0x{:04X}", opcode),
        },
        _ => format!("UNKNOWN 0x{:04X}", opcode),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble_cls() {
        assert_eq!(disassemble(0x00E0), "CLS");
    }

    #[test]
    fn test_disassemble_ret() {
        assert_eq!(disassemble(0x00EE), "RET");
    }

    #[test]
    fn test_disassemble_jp() {
        assert_eq!(disassemble(0x1234), "JP 0x234");
    }

    #[test]
    fn test_disassemble_call() {
        assert_eq!(disassemble(0x2456), "CALL 0x456");
    }

    #[test]
    fn test_disassemble_ld_vx_byte() {
        assert_eq!(disassemble(0x6A42), "LD VA, 0x42");
    }

    #[test]
    fn test_disassemble_add_vx_byte() {
        assert_eq!(disassemble(0x7505), "ADD V5, 0x05");
    }

    #[test]
    fn test_disassemble_ld_vx_vy() {
        assert_eq!(disassemble(0x8AB0), "LD VA, VB");
    }

    #[test]
    fn test_disassemble_drw() {
        assert_eq!(disassemble(0xD125), "DRW V1, V2, 5");
    }

    #[test]
    fn test_disassemble_ld_i() {
        assert_eq!(disassemble(0xA123), "LD I, 0x123");
    }

    #[test]
    fn test_disassemble_ld_dt() {
        assert_eq!(disassemble(0xF507), "LD V5, DT");
    }

    #[test]
    fn test_disassemble_ld_k() {
        assert_eq!(disassemble(0xF30A), "LD V3, K");
    }
}
