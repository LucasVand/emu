@define IOAddr 0xFFFB
@define VRAM 0x8000
@define Membank 0xFFFA
@define Width 125
@define Color 255
@define Snake_Length 20

mov a, 1 ; set a 1
str a, [Membank] ; set the membank to the vram

CALL [set_body]


; main loop
main: 

  CALL [draw_snake_body_data] 

  
; waste time here
  push a 
  push b
  mov a, 0
  mov b, 1
  lda [time_waste]
  time_waste:
    ADD16 a, b, (-1)
    JNZ_16 a, b
  pop b 
  pop a
  ; end of time waste 

  CALL [save_move_direction]

  CALL [propagate_movement]

  CALL [move]

  ; wipe old_snake_end
  ldr a, [old_snake_end]
  push a
  ldr a, [(old_snake_end + 1)]
  push a
  ; erase call
  CALL [wipe]
  pop z ; remove params
  pop z ; remove params

  
  lda [main] ; jump back to main
  jnz 1

; draws the whole snake body 
draw_snake_body_data:
  PUSH4 a, b, c, d
  LDAR c, d, snake_body_data ; load pointer to body
  mov b, (Snake_Length + 1) ; set the counter
  draw_loop:
    push Color ; push the color

    ldr a, [cd]  ; load the x
    push a ; push the x

    ADD16 c, d, 1 ; point to the y
    ldr a, [cd] ; load the y
    push a  ; push the y
    ADD16 c, d, 1 ; point to the next x

    CALL [draw] ; draw call
    pop z ; remove params
    pop z ; remove params
    pop z ; remove params

    sub b, 1 ; sub counter
    lda [draw_loop] ; load the addr of jump
    jnz b ; jump if still
  POP4 a, b, c, d
  RET

; set snake, sets the snake body to the correct pos
set_body:
  PUSH4 a, b, c, d ; save regs
  LDAR a, b, snake_body_data ; load the snake body
  ADD16 a, b, (Snake_Length * 2) ; point to the end of the snake
  ADD16 a, b, (-2) ; point to the first x to modify
  ldr c, [snake_head_data] ; load the x
  ldr z, [(snake_head_data + 1)] ; load the y
  mov d, Snake_Length ; set the counter to the length
  lda [set_loop] ; load the jump location
  set_loop:
    ADD16 a, b, 1 ; set the pointer to the y
    str z, [ab] ; store the y
    ADD16 a, b, (-1) ; set the pointer back to the x 

    sub c, 1 ; set c to next pos 
    str c, [ab] ; store the new pos
    ADD16 a, b, (-2) ; point to the previous x
    sub d, 1 ; sub counter
    jnz d ; jump if still looping
  
  POP4 a, b, c, d
  RET

save_move_direction:
  push a
  ldr a, [IOAddr] ; get the io 
  and a, 0b11110000 ; mask only the move controls

  lda [save_move_if] ; load the if addr
  jnz a ; if a 
  lda [save_move_else] ; load the else
  jnz 1
  save_move_if:
    str a, [move_direction] ; save the io
  save_move_else:
  pop a
  RET

; gets the current inputs and updates the position
move:
  PUSH4 a, b, c, d
  ldr c, [snake_head_data] ; load snake x
  ldr d, [(snake_head_data + 1)] ; load snake y
  ldr a, [move_direction] ; load the controller

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

  str c, [snake_head_data]
  str d, [(snake_head_data + 1)]
 
  POP4 a, b, c, d
  RET
; end of move function 
  
; draws a pixel
; params
; color, x, y
draw:
  SET_FP
  PUSH4 a, b, c, d ; push regs
  LDR_FP a, -5 ; load the color
  push a ; push color on the stack
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


  ;push c ; push 16 high
  ;push d ; push 16 low
  ;push b ; push 8bit
  ;CALL [multiply] ; multiply call
  ;pop z ; discard 8bit
  ;pop d ; get 16 low
  ;pop c ; get 16 high

  pop a  ; get the color from the stack
  str a, [cd] ; set the pixel color
 
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

; sets a body pixel to the next one
propagate_movement: 
  PUSH4 a, b, c, d ; save the values
  mov z, (Snake_Length) ; set the counter
  LDAR a, b, snake_body_data ; set the pointer to the snake data
  
  ldr c, [ab] ; get the last x
  str c, [old_snake_end] ; save it in the old end 
  ADD16 a, b, 1 ; point to the y
  ldr c, [ab] ; get the last y
  str c, [(old_snake_end + 1)]
  ADD16 a, b, (-1) ; move the pointer back to the x

  lda [propagte_loop] ; load the jump location
  propagte_loop:
    ADD16 a, b, 2 ; point to the next x 
    ldr c, [ab] ; get the value
    ADD16 a, b, (-2) ; point back to the x
    str c, [ab] ; save the new x
    ADD16 a, b, 3 ; point to the next y
    ldr c, [ab] ; load the next y
    ADD16 a, b, (-2) ; point back to the y
    str c, [ab] ; save the new y
    ADD16 a, b, 1 ; point to the next x
    
    sub z, 1 ; sub counter
    jnz z ; jump

  POP4 a, b, c, d ; pop values
  RET ; return

; multiplies a 16 bit number with an 8 bit, updates the stack params
; params
; 16h, 16l, 8bit
multiply:
  SET_FP
  push 0 ; make room for local var c which is 8bit param
  PUSH4 a, b, c, d
  
  LDR_FP a, -5 ; load 16 bit high
  LDR_FP b, -4 ; load 16 bit low
  LDR_FP c, -3 ; load the 8bit
  STR_FP c, 0 ; store the 8bit locally
  mov c, 0 ; acc high
  mov d, 0 ; acc low
  mov z, 1
  PUSH_HL
  multiply_loop: 
    POP_HL
    push z

    push c ; save c
    LDR_FP c, 0 ; load the 8 bit
    and z, c ; and mask with 8 bit 
    pop c ; get back c

    PUSH_HL
    lda [multiply_if] ; load the if
    JZE z ; jump if bit is zero
    add d, b ; add lows
    adc c, a ; add highs
    multiply_if:
    POP_HL

    pop z
    add z, z ; add to mask, par of left shift

    add b, b ; double the number
    adc a, a ; double the number
         
    PUSH_HL
    lda [multiply_loop] ; load loop
    jnz z ; jump
  POP_HL
  STR_FP c, -5 ; save the multiplied value
  STR_FP d, -4 ; save the multiplied value
  
  POP4 a, b, c, d
  pop z ; collapse local var
  RET

;snake body data (x, y) byte1 = x, byte2 = y
snake_body_data:
  @dw (Snake_Length * 2)
; this is the position of the snake
snake_head_data:
  @db (Snake_Length + 5), 20 

; this is where the old position of the end of the snake is 
; gets set in the propagte function
old_snake_end:
  @db 0, 0

; this is where the move direction is saved
move_direction:
  @db 0b10000000, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66

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

@macro
JZE %r0:
  cmp %r0, 0
  and f, 0b01000000
  jnz f
@end
