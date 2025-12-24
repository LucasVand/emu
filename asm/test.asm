mov a, 10
SKIPNZ a, 6
mov a, 100 
mov a, 100 
mov a, 100

mov b, 30
orr f, 1

@macro
SKIPNZ %r0, %i1:
  lda [skip]
  cmp %r0, 0
  and f, 0b01000000 
  jnz f 
  ldr h, [0xFFFE] ; 3 
  ldr l, [0xFFFF] ; 3
  sub l, 3 ; account for the load being 2 instructions ; 2
  add l, (%i1 + 13) ; 2
  adc h, ((%i1 + 13) >> 8) ; 2

  str h, [0xFFFE] ; 3
  str l, [0xFFFF] ; 3
skip:
@end
