@include <testing.asm>
main:

ldr h, [0xFFFC] ; load stack pointer
ldr l, [0xFFFD]
push 6 ; push to stack
ldr a, [hl]
ASSERT a, 6

mov a, 10
push a 
pop b
ASSERT b, 10

mov a, 100 
mov b, 50 
push a 
push b 
pop a 
pop b 
ASSERT a, 50 
ASSERT b, 100

push 100 
push 1 
push 2 
pop a 
ASSERT a, 2

COMPLETE_TESTS


