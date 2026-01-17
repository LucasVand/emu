@include <testing.asm>

main:
ldr a, [data]
ASSERT a, 1  ; tests ldr 1

mov b, 100 
str b, [write]
ldr a, [write]
ASSERT a, 100 ; tests str 2

ldr a, [(data + 2)]
ASSERT a, 3 ; test addr adding 3

ldr a, [(write - 1)]
ASSERT a, 0x10 ; test addr subbing 

mov c, 0
lda [data]
mov a, h 
mov b, l 
ldr c, [ab] 
ASSERT c, 1  ; test loading from different registers

ldr a, [double_word]
ldr b, [(double_word + 1)]
ASSERT16 a, b, 300

COMPLETE_TESTS


data:
  @db 1, 2, 3, 4, 5, 6, 7, 8, 9
double_word:
  @dd 300, 400, 500
char:
  @db 'a', 0x10

write:
  @db 100,0,0,0

