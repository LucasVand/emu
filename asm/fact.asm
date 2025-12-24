push 5
push 5
CALL [mul]

orr f, 1

mul: 
  SET_FP
  push a 
  push b
  push c
  LDR_FP a, -3
  LDR_FP b, -4
  mov c, 0
  mul_loop: 
    add c, a 
    sub b, 1 
    lda [mul_loop]
    jnz b
  
  pop c 
  pop b 
  pop a
  RET
  
  
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
LDR_FP %r1,  %i0:
  add l, %i0
  adc h, 0
  ldr %r1, [hl]
  sub l, %i0
  sbb h, 0
@end

@macro
SET_FP:
  ldr h, [0xFFFC]
  ldr l, [0xFFFD]
@end






