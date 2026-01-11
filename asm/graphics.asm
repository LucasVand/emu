mov a, 1 
str a, [0xFFFA] ; set the mem bank to grapics
@define graphics 0x8000



lda [graphics] ; loads the grapics addr in hl
mov c, 100  ; load our counter
push h  ; push h l
push l

loop:
  pop l  ; get h l after jump
  pop h
  mov a, 0b11100000 ; set the value to red
  str a, [hl]  ; store the red
  ADD16 h, l, (1) ; add one to graphics location
  sub c, 1 ; decrimaent counter
  push h  ; save hl
  push l  ; 
  lda [loop] ; prepare jump
  jnz c ; jump 


LDAR a, b, graphics
LDAR c, d, (graphics + 100)
second_loop:
  ; start of getting color
  ldr z, [0xFFFB]
  ;and z, 0b00001110
  ;lsl z, 1
  ;orr z, 0b10000010

  ;end of getting color
  str z, [cd] ; store the color
  mov z, 0
  str z, [ab] ; store the black
  ADD16 c, d, (1) ; add to front pointer
  ADD16 a, b, (1) ; add to back pointer
  lda [second_loop] 
  jnz 1


orr f, 1


@macro
RESET_POINTER %r0, %r1, %i2, %i3:
  cmp %r0, (%i2 >> 8)
  lda [done]
  JLT
  lda [check_lower]
  JEQ 
  lda [swap]
  jnz 1

check_lower:
  cmp %r1, (%i2)
  lda [done]
  JLT

swap:
  mov %r0, (%i3 >> 8)  
  mov %r1, (%i3)
done: 
@end

@macro 
JLT:
  and f, 0x10000000
  jnz f
@end
@macro 
JEQ:
  and f, 0x01000000
  jnz f
@end



@macro
ADD16 %r0, %r1, %i2:
  add %r1, (%i2)
  adc %r0, (%i2 >> 8)
@end
@macro
LDAR %r0, %r1, %i2:
  mov %r1, (%i2)
  mov %r0, (%i2 >> 8)
@end
