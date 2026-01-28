@define memBankAddr 0xFFFA

@include <debug.asm>
@include "/text/alphabet.asm"
@include <multiply16.asm>

@define graphicsModeAddr 0xFFF8
@define graphics 0x8000
@define tile_table 0x9000

main:
mov a, 1
str a, [memBankAddr]

str a, [graphicsModeAddr]

; copy tile map into vram
lda16 a, b, A_letter
push 1
push a
push b
CALL [set_tile]
dec_sp 3

lda16 a, b, B_letter
push 2
push a
push b
CALL [set_tile]
dec_sp 3


mov a, 1
str a, [0x8001]
mov a, 2
str a, [0x8002]



HALT

; params
  ; tile num
  ; 16 bit location of data
set_tile:
  set_fp
  pushm a, b, c, d
  ldr_fp c, -4 ; get the high of the data location
  ldr_fp d, -3 ; get the low of the data location
  
  ldr_fp z, -5 ; load the tile number
   
  push 0
  push 64
  push 0
  push z ; push the tile number
  CALL [multiply16]
  dec_sp 2
  popm a, b ; get the multiplied value
  add16 a, b, tile_table
  Print


  mov z, 64
  lda [copy_loop]
  copy_loop:
    push z
    ldr z, [cd] 
    str z, [ab]
    add16 a, b, 1
    add16 c, d, 1
    pop z
    sub z, 1 
    jnz z
  
  popm a, b, c, d
  RET
; end of function 

  



