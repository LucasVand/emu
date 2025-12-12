
@define sub 0x8888
@define text (4 + 5)
@define aa b
@define label (4 * 5)
@define const 0x000

ldr a, [aa]
str b, [aa]
add b, (5 + 5)


main: 
  ldr a, [sub]
  add a, const
  ADD a, b, [0x6767] 
  ADD a, b, [data]

data:
  @define thing1 'l'
  @db 0x88, 'l', 10, 0b00010, [label]
  @ds "this is a string"
  @dd 0x8888
  @db thing1, thing1

  @undefine thing1

@macro 
ADD %r0, %r1, [%i2]:
  add %r0, %r1
  str %r0, [%i2]
@end


