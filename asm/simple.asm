@include <debug.asm>

@include <random.asm>


main:
  mov b, 30
  mov a, 3 
  str a, [seed_addr]

loop:
  push 100
  CALL [random]
  dec_sp 1
  PRINT a
  lda [loop]
  sub b, 1
  jnz b


  HALT
