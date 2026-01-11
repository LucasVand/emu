@define IOAddr 0xFFFB
@define VRAM 0x8000
@define Membank 0xFFFA
@define Width 125 
@define Color 255
@define Snake_Length 10

mov a, 1 ; set a 1
str a, [Membank] ; set the membank to the vram

CALL [set_body]


; main loop
main: 

  LDAR c, d, [snake_body] ; load pointer to body
  mov z, (Snake_Length + 1)
  lda [draw_loop]
  draw_loop:
    ldr a, [cd]  ; load the x
    ADD16 c, d, 1 ; point to the y
    ldr b, [cd] ; load the y
    ; draw call
    push a ; push x 
    push b ; push y
    CALL [draw] 
    pop z ; remove params
    pop z ; remove params

    sub z, 1 ; sub counter
    jnz z ; jump if still


; waste time here
  push a 
  push b
  mov a, 10
  mov b, 0
  lda [time_waist]
  time_waist:
    ADD16 a, b, (-1)
    JNZ_16 a, b
  pop b 
  pop a
  ; end of time waist



  CALL [move]

  ; erase call
  push a 
  push b 
  CALL [wipe]
  pop z ; remove params
  pop z ; remove params

  
  lda [main] ; jump back to main
  jnz 1

; set snake, sets the snake body to the correct pos
set_body:
  PUSH4 a, b, c, d ; save regs
  LDAR a, b, [snake_body] ; load the snake body
  ADD16 a, b, (Snake_Length * 2) ; point to the end of the snake
  ADD16 a, b, (-2) ; point to the first x to modify
  ldr c, [snake_data] ; load the x
  mov d, Snake_Length ; set the counter to the length
  lda [set_loop] ; load the jump location
  set_loop:
     sub c, 1 ; set c to next pos 
     str c, [ab] ; store the new pos
     ADD16 a, b, (-2) ; point to the previous x
     sub d, 1 ; sub counter
     jnz d ; jump if still looping
  
  POP4 a, b, c, d
  RET

; gets the current inputs and updates the position
move:
  PUSH4 a, b, c, d
  ldr c, [snake_data] ; load snake x
  ldr d, [(snake_data + 1)] ; load snake y
  ldr a, [IOAddr] ; load the controller

  mov b, a
  nor b, b
  and b, 0b00010000 ; mask the up 
  lda [up_jump] ; load the jump
  jnz b ; if the flipped bit is active, jump ~active
  sub d, 1 ; move 1 up 
  up_jump: 
    
  mov b, a
  nor b, b ; flip the bit
  and b, 0b00100000 ; mask the down 
  lda [down_jump] ; load the jump
  jnz b ; if the flipped bit is active, jump ~active
  add d, 1 ; move 1 down
  down_jump:

  mov b, a
  nor b, b ; flip the bit
  and b, 0b01000000 ; mask the left
  lda [left_jump] ; load the jump
  jnz b ; if the flipped bit is active, jump ~active
  sub c, 1 ; move 1 left 
  left_jump:

  mov b, a
  nor b, b ; flip the bit
  and b, 0b10000000 ; mask the right
  lda [right_jump] ; load the jump
  jnz b ; if the flipped bit is active, jump ~active
  add c, 1 ; move 1 right 
  right_jump: 




  str c, [snake_data]
  str d, [(snake_data + 1)]
 
  POP4 a, b, c, d
  RET
; end of move function 
  
; draws a pixel
; params
; x, y
draw:
  SET_FP
  PUSH4 a, b, c, d ; push regs
  LDR_FP a, -4 ; load the x
  LDR_FP b, -3 ; load the y
  LDAR c, d, 0x8000 ; load the vram into cd
  ADD16_REG c, d, a ; add the x coord
  lda [mul] ; load mul into hl
  ; repeatedly add Width to the coord
  mul: 
    ADD16_REG c, d, Width ; add Width to coord
    sub b, 1 ; sub counter
    jnz b ; jump if still going

  mov a, Color 
  str a, [cd]
 
  POP4 a, b, c ,d ; pop regs
  RET
    
; wipe
; params
; x, y
wipe:
  SET_FP
  PUSH4 a, b, c, d
  LDR_FP a, -4 ; load the x
  LDR_FP b, -3 ; load the y
  LDAR c, d, 0x8000 ; load the vram into cd
  ADD16_REG c, d, a ; add the x coord
  lda [wipe_mul] ; load mul into hl
  ; repeatedly add Width to the coord
  wipe_mul: 
    ADD16_REG c, d, Width ; add Width to coord
    sub b, 1 ; sub counter
    jnz b ; jump if still going

  mov a, 0 
  str a, [cd]
 
  POP4 a, b, c ,d ; pop regs
  RET



;snake body data (x, y) byte1 = x, byte2 = y
snake_body:
  @dw (Snake_Length * 2)
; this is the position of the snake
snake_data:
  @db 20, 20 

@macro
LDR_FP %r1, %i0:
  add l, (%i0)
  adc h, (%i0 >> 8)
  ldr %r1, [hl]
  sub l, (%i0)
  sbb h, (%i0 >> 8)
@end

@macro
STR_FP %r1, %i0:
  add l, (%i0)
  adc h, (%i0 >> 8)
  str %r1, [hl]
  sub l, (%i0)
  sbb h, (%i0 >> 8)
@end


@macro
SET_FP:
  ldr h, [0xFFFC]
  ldr l, [0xFFFD]
@end


@macro
LDAR %r0, %r1, %i2:
  mov %r1, (%i2)
  mov %r0, (%i2 >> 8)
@end

@macro
CALL [%i0]:
  push (($ + 9) >> 8)  ; 2 bytes h
  push (($ + 7))    ; 2 bytes l
  lda [%i0] ; 3 bytes
  jnz 1 ; 2 bytes
@end

@macro 
RET:
  pop l 
  pop h
  jnz 1
@end


@macro
PUSH_HL:
  push h 
  push l 
@end

@macro
POP_HL:
  pop l 
  pop h 
@end

@macro
ADD16 %r0, %r1, %i2:
  add %r1, (%i2)
  adc %r0, (%i2 >> 8)
@end

@macro
ADD16_REG %r0, %r1, %r2:
  add %r1, %r2 
  adc %r0, 0 
@end

@macro
PUSH4 %r0, %r1, %r2, %r3:
  push %r0 
  push %r1 
  push %r2
  push %r3
@end

@macro
POP4 %r0, %r1, %r2, %r3:
  pop %r3
  pop %r2 
  pop %r1
  pop %r0
@end
@macro 
JNZ_16 %r0, %r1:
  cmp %r0, 0 
  mov z, f
  cmp %r1, 0
  and z, f 
  nor z, z
  and z, 0b01000000 
  jnz z
@end
