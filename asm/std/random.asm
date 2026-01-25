@include <multiply.asm>
@include <remainder.asm>

; gets a random number and save in z
; params
;   max number
random:
  set_fp
  push a
  ldr_fp a, -3

  ldr z, [seed]
  push z ; push the value
  push 127 ; push the constant
  CALL [multiply] ; multiply 
  dec_sp 2 ; collapse the stack
  add z, 123 ; add some numbers to z
  add z, 123 
  add z, 123
  add z, 34

  push z 
  push a
  CALL [remainder]
  dec_sp 2

  str z, [seed]

  pop a
  RET


seed:
  @db 3
