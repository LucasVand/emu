@include <multiply16.asm>
@include <debug.asm>
@include <random.asm>

@define IOAddr 0xFFFB
@define VRAM 0x8000
@define Membank 0xFFFA
@define Width 125
@define Color 255
@define Apple_Color 160
@define Snake_Length 20

main:
mov a, 1 ; set a 1
str a, [Membank] ; set the membank to the vram


CALL [set_body]

; inital apple draw
ldr16 a, b, [apple_data]
push Apple_Color 
push a
push b
CALL [draw]
dec_sp 3


; main loop
main_loop: 

  CALL [draw_snake_body_data] 

  
; waste time here
  ; push a 
  ; push b
  ; mov a, 0
  ; mov b, 1
  ; lda [time_waste]
  ; time_waste:
  ;   ADD16 a, b, (-1)
  ;   JNZ_16 a, b
  ; pop b 
  ; pop a
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

  CALL [check_apple] ; check if we are on the apple

  lda [main_loop] ; jump back to main
  jnz 1

check_apple:
  
  pushm a, b, c, d
  ldr16 a, b, [apple_data] ; load snake x and y 
  pushm a, b ; params set for wipe call <---


  ldr16 c, d, [snake_head_data] ; load head x and y

  cmp a, c ; compare high
  mov z, f ; copy
  cmp b, d ; compare low
  and z, f ; and
  and z, 0b01000000 ; mask zero flag
  lda [change_apple_skip]
  jze z ; jump if not equal


  ; only redraw if the location changes
    ; params were set before
  CALL [wipe] ; wipe apple loc

  push 160 ; push color

  push 100 ; max value
  CALL [random]
  dec_sp 1
  push z ; push apple x
  str z, [apple_data] ; store the new apple x

  push 100 ; max value
  CALL [random]
  dec_sp 1
  push z ; push apple y
  str z, [(apple_data + 1)] ; store the new apple y

  CALL [draw]
  dec_sp 3 ; collapse params

  change_apple_skip:
  dec_sp 2 ; collapse params from the wipe function, this needs to happen every time

  popm a, b, c, d
  RET




; draws the whole snake body 
draw_snake_body_data:
  PUSHM a, b, c, d
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

  POPM a, b, c, d
  RET

; set snake, sets the snake body to the correct pos
set_body:
  PUSHM a, b, c, d ; save regs
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
  
  POPM a, b, c, d
  RET

save_move_direction:
  pushm a, b
  ldr a, [IOAddr] ; get the io 
  and a, 0b11110000 ; mask only the move controls

  lda [save_move_if] ; load the if addr
  jnz a ; if a 
  lda [save_move_else] ; load the else
  jnz 1
  save_move_if:
    str a, [move_direction] ; save the io
    ; randomize the random seed 
    ldr b, [seed_addr]
    xor b, a
    str b, [seed_addr]
  save_move_else:

  popm a, b
  RET

; gets the current inputs and updates the position
move:
  PUSHM a, b, c, d
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


  ; we need to make sure the numbers dont go past 125
  ; see we remainder them before saving
  push c
  CALL [bounds_check] 
  dec_sp 1
  str z, [snake_head_data] ; save the x

  push d
  CALL [bounds_check]
  dec_sp 1
  str z, [(snake_head_data + 1)] ; save the y
 
  POPM a, b, c, d
  RET
; end of move function 

; checks the bounds of a values 
; must be 0 < value < 125
; normalizes it to this range
; params
;   value
; return 
;   return value in z
bounds_check:
  SET_FP
  LDR_FP z, -3 ; get the value
  push z  ; push for rem call
  push Width ; push for rem call
  CALL [remainder]  ; rem call
  dec_sp 2 ; collapse frame
  
  ; check if less then 0
  lda [reset_skip]
  jge z, 0
  mov z, Width 
  reset_skip:

  RET
  
; draws a pixel
; params
; color, x, y
draw:
  SET_FP
  PUSHM a, b, c, d ; push regs
  LDR_FP a, -5 ; load the color
  push a ; push color on the stack
  LDR_FP a, -4 ; load the x
  LDR_FP b, -3 ; load the y

  push 0 ; push y 16 high
  push b ; push y 16 low
  push 0 ; push width value
  push Width ; push width 
  CALL [multiply16] ; multiply call
  dec_sp 2 ; dec 2 to collapse first param
  pop d ; get 16 low
  pop c ; get 16 high

  add16 c, d, 0x8000 ; add the num so its in the vram
  add16 c, d, a ; add the x coord

  pop a  ; get the color from the stack
  str a, [cd] ; set the pixel color
 
  POPM a, b, c ,d ; pop regs
  RET
    
; wipe
; params
; x, y
wipe:
  SET_FP
  PUSHM a, b, c, d
  LDR_FP a, -4 ; load the x
  LDR_FP b, -3 ; load the y
  LDAR c, d, 0x8000 ; load the vram into cd
  ADD16 c, d, a ; add the x coord
  lda [wipe_mul] ; load mul into hl
  ; repeatedly add Width to the coord
  wipe_mul: 
    ADD16 c, d, Width ; add Width to coord
    sub b, 1 ; sub counter
    jnz b ; jump if still going

  mov a, 0 
  str a, [cd]
 
  POPM a, b, c ,d ; pop regs
  RET

; sets a body pixel to the next one
propagate_movement: 
  PUSHM a, b, c, d ; save the values
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

  POPM a, b, c, d ; pop values
  RET ; return


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
  @db 0b10000000

apple_data:
  @db 30, 30



@macro
LDAR %r0, %r1, %i2:
  mov %r1, (%i2)
  mov %r0, (%i2 >> 8)
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
JNZ_16 %r0, %r1:
  cmp %r0, 0 
  mov z, f
  cmp %r1, 0
  and z, f 
  nor z, z
  and z, 0b01000000 
  jnz z
@end

