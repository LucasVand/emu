@include <testing.asm>

main:
mov a, 1
not a
ASSERT a, 254

mov a, 1
nand a, 0b10010101
ASSERT a, 254 

COMPLETE_TESTS
