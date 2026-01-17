@macro
COMPLETE_TESTS:
  mov a, 0
  HALT
@end

@macro
FAIL:
  lda [fail]
  jnz 1
@end

@macro
ASSERT %r0, %i1:
  cmp %r0, %i1 ; compare
  mov z, f ; copy flags
  nor z, z ; invert flags
  and z, 0b01000000 ; mask 
  lda [fail]
  jnz z 
  add d, 1
@end

@macro
ASSERT16 %r0, %r1, %r2, %r3:
  cmp %r0, %r2 
  mov z, f
  cmp %r1, %r3 
  and z, f
  nor z, z
  and z, 0x40
  lda [fail]
  jnz z 
  add d, 1
@end

@macro
ASSERT16 %r0, %r1, %i2:
  cmp %r0, (%i2 >> 8)
  mov z, f
  cmp %r1, (%i2)
  and z, f
  nor z, z
  and z, 0x40
  lda [fail]
  jnz z 
  add d, 1
@end

fail:
  mov a, 1
  HALT

