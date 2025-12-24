mov a, ($) ; 2 bytes
mov b, ($) ; 2 bytes
ldr d, [0x0000] ; 3 bytes
mov c, ($) ; 2 bytes
ASSERT a, 0
ASSERT b, 2
ASSERT c, 7


mov a, (10 + 10)
ASSERT a, 20 ; test plus

mov a, (14 - 10)
ASSERT a, 4 ; test minus

mov a, (10*3) 
ASSERT a, 30 ; test mul

mov a, (40 / 5) 
ASSERT a, 8; test div

mov a, (5 + 50 / 2)
ASSERT a, 30 ; test operator precidence

mov a, (5 + 50 * 2)
ASSERT a, 105 ; test operator precidence

mov a, (2 * (10 + 5))
ASSERT a, 30

mov a, (1 << 3)
ASSERT a, 8

mov a, (16 >> 3) 
ASSERT a, 2

mov a, (16 + -4) 
ASSERT a, 12 

mov a, (-20 + -4)
ASSERT a, -24

mov a, (-20 - -4)
ASSERT a, -16



mov a, 0

orr f, 1



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
