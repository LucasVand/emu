; multiply function
; params (a, b), (c, d)
; result in z
multiply16:
  SET_FP

  pushm a, b, c, d 
  str16 0, [mask_save] ; clear mask
  str16 0, [res] ; clear res

  pushm h, l ; save the fp

  ldr_fp a, -6 ; get a from params
  ldr_fp b, -5 ; get b from params
  ldr_fp c, -4 ; get c from params
  ldr_fp d, -3 ; get d from params

  STR16 a, b, [ num1_save ] ; save num1
  str16 c, d, [ num2_save ] ; save num2

  mov z, 1 ; mask
  str z, [(mask_save + 1)] ; save 1 into the mask

  multiply16_loop:
    ldr16 a, b, [ num1_save ]
    ldr16 c, d, [ mask_save ]

    and16 a, b, c, d
    lda [multiply16_skip]
    jze16 a, b  ; jump past the add part if not mask
    
    ldr16 a, b, [ res ] ; get the res
    ldr16 c, d, [ num2_save ] ; get num2
    add16 a, b, c, d ; add res = res + num2
    str16 a, b  [ res ]; save res
    multiply16_skip:
    
    ldr16 a, b, [ mask_save ] ; get mask
    ldr16 c, d, [ num2_save ] ; get num2
    add16 a, b, a, b ; double mask
    add16 c, d, c, d ; double num2
    str16 a, b, [ mask_save ] ; save mask
    str16 c, d, [ num2_save ] ; save num2

    lda [multiply16_loop]
    jnz16 a, b ; jump if mask is not 0


  popm h, l ; get the fp
  ldr16 a, b, [ res ] ; get the res
  str_fp a, -6 ; save a res h
  str_fp b, -5 ; save b  res l

  popm a, b, c, d
  RET

; while mask != 0
;   if a and mask != 0
;     result += b
;   mask += mask
;   b += b
;

res:
 @dd 0
num1_save:
  @dd 0
num2_save:
  @dd 0
mask_save:
  @dd 0

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
STR16 %i0, [%i1]:
  mov z, %i0
  str z, [%i1]
  mov z, (%i0 >> 8)
  str z, [( %i1 + 1 )]
@end



