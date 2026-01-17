@include <testing.asm>

main:
mov a, 1
mov b, 255
ADD16 a, b, 1
ASSERT16 a, b, 512

mov a, 1 
mov b, 0
SUB16 a, b, 1
ASSERT16 a, b, 255

mov a, 1
mov b, 200
SUB16 a, b, 50
ASSERT16 a, b, 406

COMPLETE_TESTS
