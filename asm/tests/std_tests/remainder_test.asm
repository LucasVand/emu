@include <testing.asm>
@include <remainder.asm>

main:

push 20
push 3
CALL [remainder]
dec_sp 2
ASSERT z, 2

push 130
push 21
CALL [remainder]
dec_sp 2
ASSERT z, 4


COMPLETE_TESTS
