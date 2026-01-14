; add a literal to 16 bit registers
@macro
ADD16 %r0, %r1, %i2:
  add %r1, (%i2)
  adc %r0, (%i2 >> 8)
@end
 
; add 2 16 bit numbers stored in registers
@macro
ADD16 %r0, %r1, %r2, %r3:
  add %r1, %r3
  adc %r0, %r2 
@end

; sub a literal to 16 bit number
@macro
SUB16_IMM %r0, %r1, %i2:
  sub %r1, (%i2)
  sbb %r0, (%i2 >> 8)
@end
  
; sub 2 16 bit numbers stored in registers
@macro
SUB16 %r0, %r1, %r2, %r3:
  sub %r1, %r3
  sbb %r0, %r2 
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
AND16_IMM %r0, %r1, %i2:
  and %r0, (%i2)
  and %r1, (%i2 >> 8)
@end


