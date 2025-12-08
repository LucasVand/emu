ldr a, [aa]
@define sub 0x8888

main: 
  ldr a, [sub]

data:
  @db 0x88, 'l'
  @ds "this is a string"
  @dd 0x8888

@macro ADD %r0, %r1, [%i2]:
  add %r0, %r1

@macro
ADD:
