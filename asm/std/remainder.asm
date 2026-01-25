; calculates the remainder of op1 % op2 
; params
;   op1 - dividend - b
;   op2 - divisor - a

remainder:
  set_fp 
  pushm a, b
  ldr_fp a, -3  ; get the divisor
  ldr_fp b, -4 ; get the dividend
  lda [remainder_skip] ; load the rem skip
  jle b, a
  lda [remainder_loop]
  remainder_loop:
    sub b, a
    jgt b, a
  remainder_skip:

  mov z, b
  popm a, b
  RET

