@include <testing.asm>
@include <remainder.asm>

main:

push 20
push 3
CALL [remainder]
dec_sp 2
ASSERT z, 2

push 120 
push 21
CALL [remainder]
dec_sp 2
ASSERT z, 15

push 78
push 3 
CALL [remainder]
dec_sp 2 
ASSERT z, 0 

push 30
push 70
CALL [remainder]
dec_sp 2
ASSERT z, 30


COMPLETE_TESTS
