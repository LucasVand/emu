@macro
COMPLETE_TESTS:
  mov a, 0
  orr f, 1
@end

@macro
ASSERT %r0, %i1:
  cmp %r0, %i1
  mov z, f 
  nor z, z
  and z, 0x40
  lda [fail]
  jnz z 
  add d, 1
@end

fail:
  mov a, 1
  orr f, 1

