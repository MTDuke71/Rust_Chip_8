# PONG - Commented Disassembly

A detailed analysis of the classic CHIP-8 Pong game.

## Register Usage

- **VA (0xA)**: Left paddle X position (always 2)
- **VB (0xB)**: Left paddle Y position
- **VC (0xC)**: Right paddle X position (always 63)
- **VD (0xD)**: Right paddle Y position
- **V6**: Ball X position
- **V7**: Ball Y position
- **V8**: Ball X velocity (+2 or -2)
- **V9**: Ball Y velocity (+1, 0, or -1)
- **VE (0xE)**: Score counter
- **V0-V5**: Temporary variables
- **VF**: Flag register (collision, borrow, carry)

## Memory Layout

- **0x2EA-0x2EF**: Left/Right paddle sprite data (6 bytes = 6 rows)
- **0x2F0-0x2F1**: Ball sprite data (1 byte = 1 pixel)
- **0x2F2-0x2F4**: BCD score storage area

---

## Code Analysis

### INITIALIZATION (0x0200 - 0x020E)
```
0x0200   6A02   LD VA, 0x02        ; Left paddle X = 2
0x0202   6B0C   LD VB, 0x0C        ; Left paddle Y = 12
0x0204   6C3F   LD VC, 0x3F        ; Right paddle X = 63
0x0206   6D0C   LD VD, 0x0C        ; Right paddle Y = 12
0x0208   A2EA   LD I, 0x2EA        ; Point to paddle sprite
0x020A   DAB6   DRW VA, VB, 6      ; Draw left paddle (6 rows)
0x020C   DCD6   DRW VC, VD, 6      ; Draw right paddle (6 rows)
0x020E   6E00   LD VE, 0x00        ; Score = 0
0x0210   22D4   CALL 0x2D4         ; Draw score
```

**Purpose**: Initialize paddle positions, draw them, reset score to 0.

---

### GAME START DELAY LOOP (0x0212 - 0x021E) ⏱️ **KEY TIMING CODE**
```
0x0212   6603   LD V6, 0x03        ; Ball X start = 3
0x0214   6802   LD V8, 0x02        ; Ball X velocity = +2 (moving right)
0x0216   6060   LD V0, 0x60        ; Load 96 into V0
0x0218   F015   LD DT, V0          ; Set delay timer = 96 (1.6 seconds at 60Hz)
0x021A   F007   LD V0, DT          ; ┐
0x021C   3000   SE V0, 0x00        ; │ DELAY LOOP: Wait for timer to reach 0
0x021E   121A   JP 0x21A           ; ┘ Keep looping until delay timer = 0
```

**Purpose**: This is the main delay between ball movements! The game waits for the delay timer to count down from 96 to 0 before starting ball movement. At 60Hz, this creates a ~1.6 second delay.

**⚠️ IMPORTANT**: This is why changing timer speed SHOULD affect gameplay - this loop waits for the delay timer!

---

### BALL INITIALIZATION (0x0220 - 0x0228)
```
0x0220   C717   RND V7, 0x17       ; Random Y position (0-23) + ...
0x0222   7708   ADD V7, 0x08       ; ... + 8 = range 8-31
0x0224   69FF   LD V9, 0xFF        ; Ball Y velocity = -1 (moving up)
0x0226   A2F0   LD I, 0x2F0        ; Point to ball sprite
0x0228   D671   DRW V6, V7, 1      ; Draw ball (1 pixel)
```

**Purpose**: Randomize ball starting Y position (middle area), set upward velocity.

---

### MAIN GAME LOOP START (0x022A - 0x0240)
```
0x022A   A2EA   LD I, 0x2EA        ; ┐ Point to paddle sprite
0x022C   DAB6   DRW VA, VB, 6      ; │ Erase left paddle (XOR)
0x022E   DCD6   DRW VC, VD, 6      ; ┘ Erase right paddle (XOR)

0x0230   6001   LD V0, 0x01        ; ┐ Check key 1 (move left paddle up)
0x0232   E0A1   SKNP V0            ; │ Skip next if NOT pressed
0x0234   7BFE   ADD VB, 0xFE       ; ┘ Move left paddle up (-2)

0x0236   6004   LD V0, 0x04        ; ┐ Check key 4 (move left paddle down)
0x0238   E0A1   SKNP V0            ; │ Skip next if NOT pressed
0x023A   7B02   ADD VB, 0x02       ; ┘ Move left paddle down (+2)

0x023C   601F   LD V0, 0x1F        ; ┐ Clamp left paddle Y position
0x023E   8B02   AND VB, V0         ; ┘ to 0-31 (0x1F = 31)

0x0240   DAB6   DRW VA, VB, 6      ; Redraw left paddle at new position
```

**Purpose**: Handle left player input, update paddle position.

---

### AI PADDLE (0x0242 - 0x0252)
```
0x0242   8D70   LD VD, V7          ; Copy ball Y to right paddle Y
0x0244   C00A   RND V0, 0x0A       ; Random 0-10
0x0246   7DFE   ADD VD, 0xFE       ; Subtract 2 from paddle Y
0x0248   4000   SNE V0, 0x00       ; If random != 0 (90% chance)
0x024A   7D02   ADD VD, 0x02       ; Add 2 (cancel the subtract)
                                    ; Net effect: 10% chance to move up slightly
0x024C   6000   LD V0, 0x00        ; (dead code - V0 overwritten next)
0x024E   601F   LD V0, 0x1F        ; ┐ Clamp right paddle Y
0x0250   8D02   AND VD, V0         ; ┘ to 0-31

0x0252   DCD6   DRW VC, VD, 6      ; Redraw right paddle
```

**Purpose**: AI follows ball with 10% error (makes game winnable).

---

### BALL MOVEMENT (0x0254 - 0x0276)
```
0x0254   A2F0   LD I, 0x2F0        ; Point to ball sprite
0x0256   D671   DRW V6, V7, 1      ; Erase ball (XOR)

0x0258   8684   ADD V6, V8         ; Ball X += velocity (±2)
0x025A   8794   ADD V7, V9         ; Ball Y += velocity (±1, 0)

0x025C   603F   LD V0, 0x3F        ; ┐ Clamp ball X to 0-63
0x025E   8602   AND V6, V0         ; ┘

0x0260   611F   LD V1, 0x1F        ; ┐ Clamp ball Y to 0-31
0x0262   8712   AND V7, V1         ; ┘

0x0264   4602   SNE V6, 0x02       ; If ball X == 2 (left paddle column)
0x0266   1278   JP 0x278           ; → Check left paddle collision

0x0268   463F   SNE V6, 0x3F       ; If ball X == 63 (right paddle column)
0x026A   1282   JP 0x282           ; → Check right paddle collision

0x026C   471F   SNE V7, 0x1F       ; If ball Y == 31 (bottom)
0x026E   69FF   LD V9, 0xFF        ; → Set Y velocity = -1 (bounce up)

0x0270   4700   SNE V7, 0x00       ; If ball Y == 0 (top)
0x0272   6901   LD V9, 0x01        ; → Set Y velocity = +1 (bounce down)

0x0274   D671   DRW V6, V7, 1      ; Redraw ball at new position
0x0276   122A   JP 0x22A           ; → Loop back to main game loop
```

**Purpose**: Move ball, handle top/bottom bounces, check paddle collision zones.

**⚠️ NOTE**: No delay here! Ball moves every frame through the main loop, but the loop START has a delay (0x0216-0x021E).

---

### LEFT PADDLE COLLISION (0x0278 - 0x0280)
```
0x0278   6802   LD V8, 0x02        ; Set ball X velocity = +2 (bounce right)
0x027A   6301   LD V3, 0x01        ; Score increment = 1 (left player scored)
0x027C   8070   LD V0, V7          ; ┐ V0 = ball Y - paddle Y
0x027E   80B5   SUB V0, VB         ; ┘ (collision detection)
0x0280   128A   JP 0x28A           ; → Continue collision check
```

---

### RIGHT PADDLE COLLISION (0x0282 - 0x0288)
```
0x0282   68FE   LD V8, 0xFE        ; Set ball X velocity = -2 (bounce left)
0x0284   630A   LD V3, 0x0A        ; Score increment = 10 (right player scored)
0x0286   8070   LD V0, V7          ; ┐ V0 = ball Y - paddle Y
0x0288   80D5   SUB V0, VD         ; ┘
```

---

### COLLISION DETECTION (0x028A - 0x02A0)
```
0x028A   3F01   SE VF, 0x01        ; If borrow flag != 1 (ball above paddle)
0x028C   12A2   JP 0x2A2           ; → Score point (miss)

0x028E   6102   LD V1, 0x02        ; Height check value
0x0290   8015   SUB V0, V1         ; V0 -= 2, check if in paddle zone 1
0x0292   3F01   SE VF, 0x01        ; If borrow (ball in zone 1)
0x0294   12BA   JP 0x2BA           ; → Adjust Y velocity down

0x0296   8015   SUB V0, V1         ; V0 -= 2, check zone 2
0x0298   3F01   SE VF, 0x01        ; If borrow (ball in zone 2)
0x029A   12C8   JP 0x2C8           ; → Normal bounce (no Y change)

0x029C   8015   SUB V0, V1         ; V0 -= 2, check zone 3
0x029E   3F01   SE VF, 0x01        ; If borrow (ball in zone 3)
0x02A0   12C2   JP 0x2C2           ; → Adjust Y velocity up
```

**Purpose**: Paddle hit detection with spin - hitting different parts of paddle changes ball angle.

---

### SCORE POINT (0x02A2 - 0x02B8) 🔊 **SOUND CODE**
```
0x02A2   6020   LD V0, 0x20        ; ┐ Set sound timer = 32
0x02A4   F018   LD ST, V0          ; ┘ Play beep for 32/60 = 0.53 seconds

0x02A6   22D4   CALL 0x2D4         ; Draw current score
0x02A8   8E34   ADD VE, V3         ; Update score (+1 or +10)
0x02AA   22D4   CALL 0x2D4         ; Redraw updated score

0x02AC   663E   LD V6, 0x3E        ; Ball X = 62 (right side)
0x02AE   3301   SE V3, 0x01        ; If score increment != 1 (right scored)
0x02B0   6603   LD V6, 0x03        ; → Ball X = 3 (left side)

0x02B2   68FE   LD V8, 0xFE        ; Ball X velocity = -2
0x02B4   3301   SE V3, 0x01        ; If score increment != 1
0x02B6   6802   LD V8, 0x02        ; → Ball X velocity = +2

0x02B8   1216   JP 0x216           ; → Restart game (with delay!)
```

**Purpose**: Beep sound on score, update score, restart ball from scoring side.

**⚠️ CRITICAL**: Jump to 0x216 restarts with the DELAY LOOP! This is where timer speed control matters.

---

### Y VELOCITY ADJUSTMENTS (0x02BA - 0x02C6)
```
0x02BA   79FF   ADD V9, 0xFF       ; Y velocity -= 1 (more downward)
0x02BC   49FE   SNE V9, 0xFE       ; If velocity == -2 (too fast)
0x02BE   69FF   LD V9, 0xFF        ; → Clamp to -1
0x02C0   12C8   JP 0x2C8           ; Continue

0x02C2   7901   ADD V9, 0x01       ; Y velocity += 1 (more upward)
0x02C4   4902   SNE V9, 0x02       ; If velocity == +2 (too fast)
0x02C6   6901   LD V9, 0x01        ; → Clamp to +1
```

---

### PADDLE HIT SOUND (0x02C8 - 0x02D2) 🔊 **SOUND CODE**
```
0x02C8   6004   LD V0, 0x04        ; ┐ Set sound timer = 4
0x02CA   F018   LD ST, V0          ; ┘ Play short beep (4/60 = 0.067 seconds)

0x02CC   7601   ADD V6, 0x01       ; Ball X += 1 (move away from paddle)
0x02CE   4640   SNE V6, 0x40       ; If ball X == 64 (off screen)
0x02D0   76FE   ADD V6, 0xFE       ; → Move back 2 positions

0x02D2   126C   JP 0x26C           ; → Back to main loop
```

---

### DRAW SCORE SUBROUTINE (0x02D4 - 0x02E8)
```
0x02D4   A2F2   LD I, 0x2F2        ; Point to BCD storage
0x02D6   FE33   LD B, VE           ; Convert score to BCD (decimal digits)
0x02D8   F265   LD V2, [I]         ; Load 3 digits into V0, V1, V2

0x02DA   F129   LD F, V1           ; ┐ Point to font for tens digit
0x02DC   6414   LD V4, 0x14        ; │ X position = 20
0x02DE   6500   LD V5, 0x00        ; │ Y position = 0
0x02E0   D455   DRW V4, V5, 5      ; ┘ Draw tens digit

0x02E2   7415   ADD V4, 0x15       ; X position += 21
0x02E4   F229   LD F, V2           ; ┐ Point to font for ones digit
0x02E6   D455   DRW V4, V5, 5      ; ┘ Draw ones digit

0x02E8   00EE   RET                ; Return
```

---

### SPRITE DATA (0x02EA - 0x02F4)
```
0x02EA   8080   (paddle)           ; ┐
0x02EC   8080   (paddle)           ; │ Paddle sprite (6 bytes)
0x02EE   8080   (paddle)           ; │ Pattern: 10000000 (left edge pixels)
0x02F0   8000   (ball/paddle end)  ; ┘
0x02F2   0000   (BCD storage)      ; Score BCD conversion area
0x02F4   0000   (BCD storage)
```

---

## TIMING ANALYSIS

### Where Timer Speed Matters ⏱️

1. **Game Start Delay (0x0216-0x021E)**: 96 ticks @ 60Hz = 1.6 seconds
   - At 2x timer speed: 96 ticks @ 120Hz = 0.8 seconds ✅
   - At 4x timer speed: 96 ticks @ 240Hz = 0.4 seconds ✅

2. **Sound Beeps**:
   - Score beep: 32 ticks = 0.53 seconds → affected by timer speed ✅
   - Paddle hit: 4 ticks = 0.067 seconds → affected by timer speed ✅

### Where Timer Speed Does NOT Matter ⏱️

The main game loop (0x022A-0x0276) runs continuously with NO timer wait!
- Ball moves every loop iteration
- CPU speed controls loop rate
- Timer speed only affects the initial delay before each serve

### The Problem 🐛

**After a point is scored** (0x02B8), the game jumps to 0x216, which has the delay loop.
But during **normal gameplay** (0x022A-0x0276), there's NO delay - ball moves at CPU speed!

**Solution**: The delay is only at game start and after scoring. To make Pong more playable:
- Increase **CPU speed** (]) to make the main loop run faster
- Increase **timer speed** (Page Up) to make the post-score delay shorter

The ball movement is actually controlled by CPU speed, not timer speed!

---

## Key Insights

1. **Delay timer is only used**:
   - At game start (0x216-0x21E)
   - After each point scored (via jump to 0x216)

2. **Sound timer is used**:
   - Score beep: 32 ticks
   - Paddle hit beep: 4 ticks

3. **Ball speed** is actually controlled by:
   - CPU execution speed (how fast the main loop runs)
   - NOT by timers during active gameplay!

4. **The "slow ball" problem**:
   - We see the delay at game start (1.6 seconds)
   - During play, ball should move at CPU speed
   - Need to check if there's another delay loop we're missing...
