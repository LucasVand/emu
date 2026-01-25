; calculates the remainder of op1 % op2 
; params
;   op1 - dividend
;   op2 - divisor 

remainder:
  set_fp 
  pushm a, b
  ldr_fp a, -3  ; get the divisor
  ldr_fp b, -4 ; get the dividend
  lda [remainder_loop]
  remainder_loop:
    sub b, a
    jgt b, a

  mov z, b
  popm a, b
  RET

