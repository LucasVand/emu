@macro ; multiplies 2 numbers, trashes z, last 2  register is used as the mask and temp value will be trashed
MULTIPLY %r0, %r1, %r2, %r3:
  ; %r0 is a 
  ; %r1 is b
  ; %r2 is mask
  ; z is total
  mov z, 0  ; this is the accumulator
  mov %r2, 1 ; mask %r2
  multiply_loop:
    mov %r3, %r0 ; copy a 
    and %r3, %r2 ; and a with mask
    lda [skip_add]
    jze %r3
    add z, %r1 ; add b to total
    skip_add:
    add %r2, %r2 ; double mask
    add %r1, %r1 ; double b
    lda [multiply_loop]
    jnz %r2
@end

; multiply function
; params a, b
; result in z
multiply:
  SET_FP
  pushm a, b, c, d 
  ldr_fp a, -3 ; get a from params
  ldr_fp b, -4 ; get b from params
  MULTIPLY a, b, c, d
  popm a, b, c, d
  RET

; while mask != 0
;   if a and mask != 0
;     result += b
;   mask += mask
;   b += b
