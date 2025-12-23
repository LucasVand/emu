ldr a, [data]
ASSERT a, 1  ; tests ldr 1

mov b, 100 
str b, [write]
ldr a, [write]
ASSERT a, 100 ; tests str 2

(5 >> 9)
ldr a, [(data + 2)]
ASSERT a, 3 ; test addr adding 3

ldr a, [(write - 1)]
ASSERT a, 0x10 ; test addr subbing 

mov c, 0
lda [data]
mov a, h 
mov b, l 
ldr c, [ab] 
ASSERT c, 1  ; test loading from different registers

ldr a, [double_word]
ldr b, [(double_word + 1)]
ASSERT_DOUBLE a, b, 300


code:
  mov a, 

mov a, 0
HALT 

data:
  @db 1, 2, 3, 4, 5, 6, 7, 8, 9
double_word:
  @dd 300, 400, 500
char:
  @db 'a', 0x10

write:
  @db 100,0,0,0


@macro
HALT:
  orr f, 1
@end

@macro
ASSERT_DOUBLE %r0, %r1, %i1:
  cmp %r0, (%i1 >> 8)
  cmp %r1, (%i1)
  mov z, f 
  nor z, z
  and z, 0x40
  lda [fail]
  jnz z 
  add d, 1
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
