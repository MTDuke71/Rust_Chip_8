| 0nnn | SYS addr | Jump to machine code routine (ignored) |
| 00E0 | CLS | Clear display |
| 00EE | RET | Return from subroutine |
| 1nnn | JP addr | Jump to address nnn |
| 2nnn | CALL addr | Call subroutine at nnn |
| 3xkk | SE Vx, byte | Skip if Vx == kk |
| 4xkk | SNE Vx, byte | Skip if Vx != kk |
| 5xy0 | SE Vx, Vy | Skip if Vx == Vy |
| 6xkk | LD Vx, byte | Set Vx = kk |
| 7xkk | ADD Vx, byte | Set Vx = Vx + kk |
| 8xy0 | LD Vx, Vy | Set Vx = Vy |
| 8xy1 | OR Vx, Vy | Set Vx = Vx OR Vy |
| 8xy2 | AND Vx, Vy | Set Vx = Vx AND Vy |
| 8xy3 | XOR Vx, Vy | Set Vx = Vx XOR Vy |
| 8xy4 | ADD Vx, Vy | Set Vx = Vx + Vy, VF = carry |
| 8xy5 | SUB Vx, Vy | Set Vx = Vx - Vy, VF = !borrow |
| 8xy6 | SHR Vx | Set Vx = Vx >> 1, VF = LSB |
| 8xy7 | SUBN Vx, Vy | Set Vx = Vy - Vx, VF = !borrow |
| 8xyE | SHL Vx | Set Vx = Vx << 1, VF = MSB |
| 9xy0 | SNE Vx, Vy | Skip if Vx != Vy |
| Annn | LD I, addr | Set I = nnn |
| Bnnn | JP V0, addr | Jump to nnn + V0 |
| Cxkk | RND Vx, byte | Set Vx = random AND kk |
| Dxyn | DRW Vx, Vy, n | Draw sprite at (Vx, Vy), VF = collision |
| Ex9E | SKP Vx | Skip if key Vx pressed |
| ExA1 | SKNP Vx | Skip if key Vx not pressed |
| Fx07 | LD Vx, DT | Set Vx = delay timer |
| Fx0A | LD Vx, K | Wait for key, store in Vx |
| Fx15 | LD DT, Vx | Set delay timer = Vx |
| Fx18 | LD ST, Vx | Set sound timer = Vx |
| Fx1E | ADD I, Vx | Set I = I + Vx |
| Fx29 | LD F, Vx | Set I = sprite location for digit Vx |
| Fx33 | LD B, Vx | Store BCD of Vx at I, I+1, I+2 |
| Fx55 | LD [I], Vx | Store V0..Vx at I |
| Fx65 | LD Vx, [I] | Load V0..Vx from I |