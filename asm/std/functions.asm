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
LDR_FP %r1, %i0:
  ADD16 h, l, %i0
  ldr %r1, [hl]
  SUB16 h, l, %i0

@end

@macro
STR_FP %r1, %i0:
  ADD16 h, l, %i0
  str %r1, [hl]
  SUB16 h, l, %i0
@end


@macro
SET_FP:
  ldr h, [0xFFFC]
  ldr l, [0xFFFD]
@end

