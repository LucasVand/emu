ldr a, [data]
ASSERT a, 1  ; tests ldr 1

mov b, 100 
str b, [write]
ldr a, [write]
ASSERT a, 100 ; tests str 2

mov a, 0
HALT 

data:
  @db 1, 2, 3, 4, 5, 6, 7, 8, 9
  @dd 300, 400, 500

write:
  @db 0,0,0,0


@macro
HALT:
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
  HALT
