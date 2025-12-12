@define start 1

mov a, 10
mov b, 20
mov c, start
add a, b

mov a, 123
mov b, 0
str a, [0xFF]
ldr b, [0xFF]
HALT a


@macro
HALT %r0:
  orr f, 1 
@end
