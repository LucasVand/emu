@include <multiply.asm>
@include <remainder.asm>

; gets a random number and save in z
; params
;   max number
random:
  set_fp
  push a
  ldr_fp a, -3 ; load the max value

  ldr z, [seed_addr]
  push z ; push the value
  push 113 ; push the constant
  CALL [multiply] ; multiply 
  dec_sp 2 ; collapse the stack
  add z, 123 ; add some numbers to z
  and z, 0b01111111 ; makes the number postitive

  push z 
  push a
  CALL [remainder]
  dec_sp 2

  str z, [seed_addr]

  pop a
  RET


seed_addr:
  @db 6
