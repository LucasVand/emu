@define memBankAddr 0xFFFA
@define graphics 0x8000

mov a, 1
str a, [memBankAddr] ; set the mem bank to graphics

mov a, 255
str a, [(graphics + 4)]
lda [draw_square]
jnz 1

draw_square:
  ldr a, [(square_data + 1)]
  str a, [(graphics + 1)]
  ldr a, [(square_data + 2)]
  str a, [(graphics + 2)]
  ldr a, [(square_data + 3)]
  str a, [(graphics + 3)]
  orr f, 1


square_data:
  @db 66, 66, 66, 66, 66, 66, 66, 66, 66
