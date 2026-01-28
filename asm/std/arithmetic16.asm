@macro ; decriment a 16 bit number
DEC16 %r0, %r1:
  SUB16 %r0, %r1, 1
@end

@macro ; deciment a 16 bit number
INC16 %r0, %r1:
  ADD16 %r0, %r1, 1
@end

; add a literal to 16 bit registers
@macro
ADD16 %r0, %r1, %i2:
  add %r1, (%i2)
  adc %r0, (%i2 >> 8)
@end

@macro
ADD16 %r0, %r1, %r2:
  add %r1, %r2
  adc %r0, 0 
@end

 
; add 2 16 bit numbers stored in registers
@macro
ADD16 %r0, %r1, %r2, %r3:
  add %r1, %r3
  adc %r0, %r2 
@end

; sub a literal to 16 bit number
@macro
SUB16 %r0, %r1, %i2:
  sub %r1, (%i2)
  sbb %r0, (%i2 >> 8)
@end
  
; sub 2 16 bit numbers stored in registers
@macro
SUB16 %r0, %r1, %r2, %r3:
  sub %r1, %r3
  sbb %r0, %r2 
@end
@macro
SUB16 %r0, %r1, %r2:
  sub %r1, %r3
  sbb %r0, 0 
@end


; not a 16 bit number
@macro
NOT16 %r0, %r1:
  nor %r0, %r0
  nor %r1, %r1
@end

; and 2 16 bit numbers stored in registers
@macro
AND16 %r0, %r1, %r2, %r3:
  and %r0, %r2
  and %r1, %r3
@end

; and a 16 bit number with a literal
@macro
AND16 %r0, %r1, %i2:
  and %r0, (%i2)
  and %r1, (%i2 >> 8)
@end

@macro
LDR16 %r0, %r1, [%i2]:
  ldr %r0, [%i2]
  ldr %r1, [(%i2 + 1)]
@end

@macro 
STR16 %r0, %r1, [%i2]:
  str %r0, [%i2]
  str %r1, [(%i2 + 1)]
@end

@macro
LDA16 %r0, %r1, %i2:
  mov %r0, (%i2 >> 8)
  mov %r1, (%i2)
@end
