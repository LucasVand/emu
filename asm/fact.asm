push 3
push 0
CALL [fact]
pop a

orr f, 1

fact:
  SET_FP 
  push a
  push b
  mov b, 1
  LDR_FP a, -4
  push h 
  push l
  fact_loop:
    push a
    push b
    push 0 
    CALL [mul]
    pop b
    pop z
    pop z
    sub a, 1
    lda [fact_loop]
    jnz a
  pop l 
  pop h
  STR_FP b, -3
  pop a 
  pop b
  RET



mul: 
  SET_FP
  push a 
  push b
  push c
  push z
  LDR_FP a, -4 ; load param 1
  LDR_FP b, -5 ; load param 2
  cmp b, a ; compare the 2 operands
  mov z, f
  and z, 0b10000000 ; isolate the less bit
  push l ; save h and l
  push h
  lda [dont_swap] ; load the location
  jnz z ; jump if we dont want to swap
  mov z, a 
  mov a, b
  mov b, z

  dont_swap: 
  pop h  ; get them back
  pop l 
  mov c, 0
  push l
  push h
  mul_loop: 
    add c, a 
    sub b, 1 
    lda [mul_loop]
    jnz b
  pop h 
  pop l 
  STR_FP c, -3
  pop z
  pop c 
  pop b 
  pop a
  RET

@macro
SKIPNZ %r0, %i1:
  push l 
  push h 
  lda [skip]
  jnz %r0
  push a 
  push b 
  ldr a, [0xFFFE] ; 3 
  ldr b, [0xFFFF] ; 3
  sub b, 3 ; account for the load being 2 instructions ; 2
  add b, (%i1 + 22) ; 2
  adc a, ((%i1 + 22) >> 8) ; 2
  str a, [0xFFFE] ; 3
  str b, [0xFFFF] ; 3
  pop b  ; 1
  pop a  ; 1
skip:
  pop h 
  pop l
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
LDR_FP %r1,  %i0:
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






