@define test 10

LDA_REG a, b
ldr c, [ab]
mov z, 100
mov d, (1 << 2)

orr f, 1

fail:
  @db 1, 2, 3, 4

@macro
LDA_REG %r0, %r1:
  lda [(fail + 3)]
  mov %r0, h 
  mov %r1, l 
@end
