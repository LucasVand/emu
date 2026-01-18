@include <multiply16.asm>
@include <testing.asm>

main:
push 0
push 10
push 0
push 10
CALL [multiply16]
dec_sp 2
pop b
pop a

ASSERT16 a, b, 100

push 1
push 10
push 0
push 56
CALL [multiply16]
dec_sp 2
pop b
pop a
ASSERT16 a, b, (((1 << 8) + 10) * ((0 << 8) + 56)) 

COMPLETE_TESTS
