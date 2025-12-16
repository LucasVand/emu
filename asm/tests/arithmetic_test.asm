mov d, 0
mov a, 20
mov b, 40 
sub a, b
ASSERT a, -20

mov a, 20 
mov b, 40 
add a, b 
ASSERT a, 60


mov a, 0b01010010 
mov b, 0b01000101 
and a, b
ASSERT a, 0b01000000

mov a, 0
HALT


@macro
HALT:
  orr f, 1
@end

@macro
ASSERT %r0, %i1:
  cmp %r0, %i1
  mov z, f 
  and z, z
  and z, 0x40
  lda [fail]
  jnz z 
  add d, 1
@end


fail:
  mov a, 1
  HALT
  
