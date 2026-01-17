@include <testing.asm>

main:
mov d, 0
mov a, 20
mov b, 40 
sub a, b
ASSERT a, -20 ; test sub 1

mov a, 20 
mov b, 40 
add a, b 
ASSERT a, 60 ; test add 2

mov f, 0  
mov a, 10 
mov b, 20 
cmp a, b
ASSERT f, 0b10000000 ; test less flag 3

mov f, 0  
mov a, 100 
mov b, 20 
cmp a, b
ASSERT f, 0b00000000 ; test less flag 4

mov f, 0 
mov a, 10 
mov b, 10 
cmp a, b 
ASSERT f, 0b01000000 ; test zero flag 5

mov f, 0 
mov a, 10 
cmp a, 0
ASSERT f, 0b00000000 ; test zero flag 
 
mov f, 0
mov a, 100
mov b, 200 
add a, b 
and f, 0b00100000 
ASSERT f, 0b00100000 ; test the carry flag 6

mov f, 0
mov a, 0 
mov b, 100 
sub a, b 
and f, 0b00010000 
ASSERT f, 0b00010000 ; test the borrow flag 7

mov f, 0
mov a, 100 
mov b, 100 
add a, b 
and f, 0b00001000 
ASSERT f, 0b00001000 ; test the overflow flag 8



mov a, 0
HALT






  
