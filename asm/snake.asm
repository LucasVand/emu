mov a, 1
str a, [0xFFFA] ; set the mem bank

LDAR a, b, snake_data ; load and ab with the snake data addr
ADD16 a, b, 40 ; set the snake data to the end

; sets all the values to the pos
mov z, 20 
set: 
  str z, [ab] ; store the inital x pos 
  ADD16 a, b, (-2) ; set the snake data pointer to the prev
  sub z, 1 ; sub counter
  lda [set]
  jnz z ; jump

mov z, 20 ; reset the counter
loop:
  CALL [update_pos] ; call update pos
  sub z, 1 ; decriment counter
  lda [loop] ; set hl
  jnz z ; loop

mov z, 20 ; reset counter
LDAR a, b, snake_data ; set the snake data pointer
LDAR c, d, 0x8000 ; load the vram addr into cd
draw:
  push z  ; save z
  ldr z, [ab] ; load the snake data
  str z, [cd] ; save it in vram
  pop z ; get z back
  sub z, 1 ; reduce counter
  ADD16 a, b, 1 ; increase pointer snake
  ADD16 b, c, 1 ; increase pointer vram
  lda [draw] ; load hl
  jnz z ; jump if still looping
  
orr f, 1

update_pos:
  LDAR a, b, snake_data

  mov c, 20
  update_loop:
  
    ADD16 a, b, 2  ; point to the next x
    ldr z, [ab] ; load it 
    ADD16 a, b, -2 ; point back to original x
    str z, [ab] ; store the new x
    ADD16 a, b, 3 ; point to the next y 
    ldr z, [ab] ; load it 
    ADD16 a, b, -2 ; point back to original y
    str z, [ab] ; store the new y

    ADD16 a, b, 2 ; point to the next pair
    sub c, 1 ; deciment the counter

    lda [update_loop] 
    jnz c
    
  ldr z, [ab] 
  add z, 1 
  str z, [ab]
  RET





snake_data:
  @dw 40 

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

