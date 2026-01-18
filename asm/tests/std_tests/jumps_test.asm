@include <testing.asm>


main:

mov a, 10
lda [eq_skip]
JEQ a, 10 ; testing jeq jumps
FAIL
eq_skip:

mov a, 67
lda [fail]
jeq a, 55 ; test if jeq does not jump when not equal
  
mov a, 0
lda [ze_skip]
jze a ; testing jze jumps
FAIL
ze_skip:

mov a, 10
lda [fail]
jze a ; makes sure jze does not jump

mov a, 55 
lda [neq_skip]
jne a, 45  ; makes sure it jumps when not equal
FAIL
neq_skip:

mov a, 34
lda [fail]
jne a, 34 ; makes sure it does not jump when not equal

mov a, 14
lda [le_skip] 
JLE a, 20 ; makes sure it jumps when greater
FAIL
le_skip:

mov a, 20
lda [lee_skip]
JLE a, 20 ; makes sure it jumps when equal
FAIL 
lee_skip:


mov a, 10
lda [ge_skip]
JGT a, 20
FAIL
ge_skip:

mov a, 40
lda [fail]
JGT a, 20

mov a, 0
mov b, 0
lda [z16_skip]
JZE16 a, b
FAIL
z16_skip:

mov a, 0
mov b, 4
lda [fail]
JZE16 a, b

mov a, 0
mov b, 1
lda [nz16_skip]
JNZ16 a, b
FAIL
nz16_skip:

mov a, 0
mov b, 0
lda [fail]
JNZ16 a, b


COMPLETE_TESTS
