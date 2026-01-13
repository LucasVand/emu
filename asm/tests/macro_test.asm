@macro
TEST1 %r0, %i1:
   mov %r0, %i1
@end

@macro
TEST2 %r0, %r1:
   mov %r0, %r1
@end

@macro
TEST3 %r0, %x1:
   mov %r0, %x1
@end

TEST1 a, 10
ASSERT a, 10

mov b, 20
TEST2 a, b
ASSERT a, 20

mov c, 30
TEST3 a, c
ASSERT a, 30

TEST3 a, 100
ASSERT a, 100


END_TESTS

@macro
END_TESTS:
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
