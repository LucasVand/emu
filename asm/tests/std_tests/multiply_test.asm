@include <multiply.asm>
@include <testing.asm>

main:
push 10
push 10
CALL [multiply]
DEC_SP 2 ; collapses the stack
ASSERT z, 100

push 5
push 9
CALL [multiply]
DEC_SP 2 ; collapses the stack
ASSERT z, 45

push 11
push 13 
CALL [multiply]
DEC_SP 2 
ASSERT z, 143

COMPLETE_TESTS
